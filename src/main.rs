mod graph;

use std::time::Duration;

use graph::{node::GenealogicalNode, Graph, GraphMessage};
use iced::{
    event, mouse, time,
    widget::{button, column, container, row, text, text_input, Column},
    Alignment, Border, Color, Element, Error, Event,
    Length::Fill,
    Point, Shadow, Subscription,
};

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
    Graph(GraphMessage),
    Tick,
    UpdateNodeName((u128, String)),
}

struct App {
    graph: Graph,
    selected_node: Option<u128>,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::EventOccurred(event) => self.on_event(event),
            Message::Tick => self.graph.tick(),
            Message::UpdateNodeName((node_id, name)) => {
                if let Some(node) = self.graph.nodes_mut().iter_mut().find(|node| node.id() == node_id) {
                    if let graph::node::GraphNodeType::GenealogicalNode(genealogical_node) = node {
                        genealogical_node.set_first_name(name);
                    }
                }
            }
            Message::Graph(graph_message) => match graph_message {
                GraphMessage::InsertNode(point) => {
                    self.graph
                        .insert_node(graph::node::GraphNodeType::GenealogicalNode(GenealogicalNode::new(
                            point,
                        )));
                }
                GraphMessage::MoveCursor(cursor_position) => self.graph.cursor_position = cursor_position,
                GraphMessage::ClickNode((node_id, event)) => match event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        println!("Clicked node: {}", node_id);
                        if self.selected_node.is_some() {
                            self.selected_node = None;
                            self.graph.deselect_node(node_id);
                        } else {
                            self.selected_node = Some(node_id);
                            self.graph.set_selected_node(node_id);
                        }
                    }
                    _ => {}
                }, // GraphMessage::SelectNode(node_id) | GraphMessage::DeselectNode(node_id) => {
                   //     if self.graph.selected_node() == Some(node_id) {
                   //         self.graph.set_selected_node(None);
                   //     } else {
                   //         self.graph.set_selected_node(Some(node_id));
                   //     }
                   //     self.graph.toggle_node_selection(node_id);
                   // }
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let mut side_panel_column = column![
            text("Graph").color(Color::BLACK),
            button(text("Add new").align_y(Alignment::Center))
                .width(Fill)
                .on_press(Message::Graph(GraphMessage::InsertNode(Point::new(0.0, 0.0))))
        ];
        if let Some(node_id) = self.selected_node {
            let Some(graph::node::GraphNodeType::GenealogicalNode(selected_node)) =
                self.graph.nodes().iter().find(|node| node.id() == node_id)
            else {
                return container(column![]).width(Fill).height(Fill).into();
            };
            let selected_node_widgets: Column<'_, Message> = column![text_input(
                "Input persons name",
                &selected_node.first_name().unwrap_or("".to_string()),
            )
            .on_input(move |input| Message::UpdateNodeName((node_id, input)))];
            side_panel_column = side_panel_column.push(selected_node_widgets);
        }
        let side_panel = container(side_panel_column)
            .width(300)
            .padding(10)
            .height(Fill)
            .style(|_| container::Style {
                background: Some(iced::Background::Color(Color::WHITE)),
                shadow: Shadow { ..Default::default() },
                border: Border {
                    radius: 5.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            });
        let content = row![self.graph.view().map(Message::Graph), side_panel];

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
            selected_node: None,
        }
    }
}

fn main() -> Result<(), Error> {
    iced::application("Iced graph", App::update, App::view)
        .subscription(App::subscription)
        .antialiasing(true)
        .run()
}
