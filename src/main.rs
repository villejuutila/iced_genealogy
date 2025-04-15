mod graph;

use std::time::Duration;

use graph::{Graph, GraphMessage};
use iced::{
    event, time,
    widget::{column, container, row, text},
    Border, Color, Element, Error, Event,
    Length::Fill,
    Shadow, Subscription,
};

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
    Graph(GraphMessage),
    Tick,
}

struct App {
    graph: Graph,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::EventOccurred(event) => self.on_event(event),
            Message::Tick => self.graph.tick(),
            Message::Graph(graph_message) => match graph_message {
                GraphMessage::InsertNode(node) => {
                    self.graph.insert_node(node);
                }
                GraphMessage::MoveCursor(cursor_position) => self.graph.cursor_position = cursor_position,
                GraphMessage::SelectNode(node_id) | GraphMessage::DeselectNode(node_id) => {
                    if self.graph.selected_node() == Some(node_id) {
                        self.graph.set_selected_node(None);
                    } else {
                        self.graph.set_selected_node(Some(node_id));
                    }
                    self.graph.toggle_node_selection(node_id);
                }
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let mut content = row![self.graph.view().map(Message::Graph)];
        let mut side_pane = None;
        // if let Some(selected_node) = self.graph.selected_node() {
        side_pane = Some(
            container(column![text("Graph").color(Color::BLACK)])
                .width(300)
                .height(Fill)
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(Color::WHITE)),
                    shadow: Shadow { ..Default::default() },
                    border: Border {
                        radius: 5.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
        );
        // }
        let content = content.push_maybe(side_pane);

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
