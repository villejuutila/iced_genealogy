mod graph;

use graph::{Graph, GraphMessage};
use iced::{
    event,
    widget::{column, container},
    Element, Error, Event,
    Length::Fill,
    Subscription,
};

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
    Graph(GraphMessage),
}

struct App {
    graph: Graph,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::EventOccurred(event) => self.on_event(event),
            Message::Graph(graph_message) => match graph_message {
                GraphMessage::InsertNode(node) => {
                    self.graph.insert_node(node);
                }
                GraphMessage::MoveCursor(cursor_position) => self.graph.cursor_position = cursor_position,
                GraphMessage::SelectNode(node_id) | GraphMessage::DeselectNode(node_id) => {
                    println!("Node ID: {}", node_id);
                    self.graph.toggle_node_selection(node_id);
                }
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![self.graph.view().map(Message::Graph)];

        container(content).width(Fill).height(Fill).into()
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::EventOccurred)
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
