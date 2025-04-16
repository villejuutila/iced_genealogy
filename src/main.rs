mod graph;
mod side_panel;

use std::time::Duration;

use graph::{
    node::{GenealogicalNode, GraphNodeTrait, Sex},
    Graph, GraphMessage,
};
use iced::{
    event, mouse, time,
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
    graph: Graph,
    selected_node: Option<u128>,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::EventOccurred(event) => self.on_event(event),
            Message::Tick => self.graph.tick(),
            Message::UpdateNodeName((node_id, name)) => {
                let graph::node::GraphNodeType::GenealogicalNode(node) = self.graph.get_node_mut_unsafe(Some(node_id));
                node.set_first_name(name);
            }
            Message::SetNodeSex((node_id, sex)) => {
                let graph::node::GraphNodeType::GenealogicalNode(node) = self.graph.get_node_mut_unsafe(Some(node_id));
                node.set_sex(sex);
            }
            Message::Graph(graph_message) => match graph_message {
                GraphMessage::UpdateBounds(bounds) => {
                    self.graph.set_bounds(bounds);
                }
                GraphMessage::InsertNode(edge_node_id) => {
                    println!("Insert node edge_node_id: {:?}", edge_node_id);
                    let mut center = self.graph.bounds().center();
                    if let Some(found_node) = self.graph.get_node(edge_node_id) {
                        center = found_node.anchor();
                        center.y += found_node.size().height * 2.0;
                    }
                    let new_node = GenealogicalNode::new(center);
                    self.graph.add_edge_between_nodes(edge_node_id, new_node.id());
                    self.graph
                        .insert_node(graph::node::GraphNodeType::GenealogicalNode(new_node));
                }
                GraphMessage::ClickNode((node_id, event)) => match event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        if self.selected_node.is_some() {
                            self.selected_node = None;
                            self.graph.deselect_node(node_id);
                        } else {
                            self.selected_node = Some(node_id);
                            self.graph.set_selected_node(node_id);
                        }
                    }
                    _ => {}
                },
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let selected_node = self
            .graph
            .nodes()
            .iter()
            .find(|node| node.id() == self.selected_node.unwrap_or(0));
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
