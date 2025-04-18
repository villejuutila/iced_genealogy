#[macro_use]
extern crate objc;

use std::{sync::Mutex, time::Duration};

use cocoa::{
    appkit::{NSApp, NSApplication, NSApplicationActivationPolicy, NSButton, NSMenu, NSMenuItem},
    base::nil,
    foundation::{NSAutoreleasePool, NSString},
};
use dispatch::Queue;
use futures::executor::block_on;
use futures::Stream;
use futures::{channel::mpsc::Sender, sink::SinkExt};
use iced::stream;
use log::error;
use objc::{
    declare::ClassDecl,
    runtime::{Object, Sel},
};
use once_cell::sync::OnceCell;

#[derive(Debug, Clone)]
pub enum Event {
    OpenFile,
}

static EVENT_SENDER: OnceCell<Mutex<Sender<Event>>> = OnceCell::new();

pub fn setup_menu_bar() -> impl Stream<Item = Event> {
    stream::channel(10, |output| async move {
        EVENT_SENDER.set(Mutex::new(output)).unwrap();
        #[allow(dead_code)]
        extern "C" fn open_file_action(_this: &Object, _sel: Sel, _sender: *mut Object) {
            println!("Open file action triggered");
            if let Some(mutex) = EVENT_SENDER.get() {
                println!("Sending open file event");
                if let Ok(mut sender) = mutex.lock() {
                    match block_on(sender.send(Event::OpenFile)) {
                        Err(e) => {
                            error!("Failed to send event: {e}");
                        }
                        _ => {}
                    }
                }
            }
        }

        #[allow(dead_code)]
        fn register_menu_handler_class() -> *mut Object {
            unsafe {
                let superclass = class!(NSObject);
                let mut decl = ClassDecl::new("RustMenuHandler", superclass).unwrap();

                decl.add_method(
                    sel!(openFileAction:),
                    open_file_action as extern "C" fn(&Object, Sel, *mut Object),
                );

                let cls = decl.register();
                let instance: *mut Object = msg_send![cls, new];
                instance
            }
        }

        fn create_menu_bar() {
            unsafe {
                let app = NSApp();
                app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular);

                let handler = register_menu_handler_class();

                let menu_bar = NSMenu::new(nil).autorelease();
                app.setMainMenu_(menu_bar);

                let app_menu_item = NSMenuItem::new(nil).autorelease();
                menu_bar.addItem_(app_menu_item);

                let app_menu = NSMenu::new(nil).autorelease();
                let quit_title = NSString::alloc(nil).init_str("Quit MyApp");
                let quit_action = sel!(terminate:);
                let quit_item = NSMenuItem::alloc(nil)
                    .initWithTitle_action_keyEquivalent_(quit_title, quit_action, NSString::alloc(nil).init_str("q"))
                    .autorelease();
                app_menu.addItem_(quit_item);
                app_menu_item.setSubmenu_(app_menu);

                let file_menu_item = NSMenuItem::new(nil).autorelease();
                menu_bar.addItem_(file_menu_item);

                let file_menu = NSMenu::new(nil).autorelease();
                let import_title = NSString::alloc(nil).init_str("Import");
                let import_action = sel!(openFileAction:);
                let import_item = NSMenuItem::alloc(nil)
                    .initWithTitle_action_keyEquivalent_(
                        import_title,
                        import_action,
                        NSString::alloc(nil).init_str("i"),
                    )
                    .autorelease();

                NSMenuItem::setTarget_(import_item, handler);

                file_menu.addItem_(import_item);

                let file_menu_title = NSString::alloc(nil).init_str("File");
                file_menu_item.setTitle_(file_menu_title);
                file_menu_item.setSubmenu_(file_menu);
            }
        }

        std::thread::spawn(|| Queue::main().exec_after(Duration::from_millis(100), || create_menu_bar()));
    })
}
