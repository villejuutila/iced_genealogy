mod genealogical_node;
mod side_panel;

use std::{env::current_dir, fs::File, io::Read, time::Duration};

use gedcom::parse;
use genealogical_node::{GenealogicalNode, Sex};
use graph::{Graph, GraphMessage};
use iced::{
    event, time,
    widget::{container, row},
    Element, Error, Event,
    Length::Fill,
    Subscription, Task,
};
use rfd::{AsyncFileDialog, FileHandle};
use side_panel::side_panel;

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
    Graph(GraphMessage),
    Tick,
    UpdateNodeName((u128, String)),
    SetNodeSex((u128, Sex)),
    OpenFile,
    OpenFileResult(Option<FileHandle>),
}

struct App {
    graph: Graph<GenealogicalNode>,
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EventOccurred(event) => self.on_event(event),
            Message::Tick => self.graph.tick(),
            Message::UpdateNodeName((node_id, name)) => {
                let node = self.graph.get_node_mut_unsafe(Some(node_id));
                node.set_first_name(name);
            }
            Message::SetNodeSex((node_id, sex)) => {
                let node = self.graph.get_node_mut_unsafe(Some(node_id));
                node.set_sex(sex);
            }
            Message::OpenFile => {
                return Task::perform(
                    AsyncFileDialog::new()
                        .set_directory(current_dir().unwrap())
                        .add_filter("", &["GED"])
                        .pick_file(),
                    |handle| Message::OpenFileResult(handle),
                )
            }
            Message::OpenFileResult(handle) => {
                if let Some(handle) = handle {
                    let mut file = File::open(handle.path()).unwrap();
                    let mut content = String::new();
                    file.read_to_string(&mut content).unwrap();
                    let res = parse(content.chars());
                    return Task::batch(
                        res.individuals
                            .iter()
                            .map(|ind| Task::done(Message::Graph(GraphMessage::InsertNode(None)))),
                    );
                }
            }
            Message::Graph(graph_message) => self.graph.update(graph_message),
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let selected_node = self.graph.selected_node();
        let content = row![self.graph.view().map(Message::Graph), side_panel(selected_node)];

        container(content).width(Fill).height(Fill).into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            event::listen().map(Message::EventOccurred),
            time::every(Duration::from_millis(100)).map(|_| Message::Tick),
        ])
    }

    fn on_event(&mut self, _event: Event) {}
}

impl Default for App {
    fn default() -> Self {
        Self {
            graph: Graph::default(),
        }
    }
}

fn main() -> Result<(), Error> {
    iced::application("Iced graph", App::update, App::view)
        .subscription(App::subscription)
        .antialiasing(true)
        .run()
}
