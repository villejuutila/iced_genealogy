mod side_panel;

use std::time::Duration;

use graph::{
    node::genealogical_node::{GenealogicalNode, Sex},
    Graph, GraphMessage,
};
use iced::{
    event, time,
    widget::{container, row},
    Element, Error, Event,
    Length::Fill,
    Subscription,
};
use side_panel::side_panel;

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
    Graph(GraphMessage),
    Tick,
    UpdateNodeName((u128, String)),
    SetNodeSex((u128, Sex)),
}

struct App {
    graph: Graph<GenealogicalNode>,
}

impl App {
    fn update(&mut self, message: Message) {
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
            Message::Graph(graph_message) => self.graph.update(graph_message),
        }
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
