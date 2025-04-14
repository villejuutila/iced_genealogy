mod node;

use iced::{
    event::Status,
    keyboard, mouse,
    widget::{
        canvas::{self, Frame},
        Canvas,
    },
    Color, Element,
    Length::Fill,
    Point, Rectangle, Renderer, Size, Theme,
};
use node::GraphNode;

#[derive(Debug, Clone, PartialEq)]
pub enum GraphInteraction {
    None,
    HoverGraphNode(u128),
    PhantomGraphNode,
}

impl Default for GraphInteraction {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone)]
pub enum GraphMessage {
    InsertNode(GraphNode),
    MoveCursor(Point),
    SelectNode(u128),
    DeselectNode(u128),
}

pub struct Graph {
    pub cursor_position: Point,
    nodes: Vec<GraphNode>,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            cursor_position: Point::ORIGIN,
            nodes: vec![],
        }
    }
}

impl Graph {
    pub fn toggle_node_selection(&mut self, node_id: u128) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == node_id) {
            node.selected = !node.selected;
        }
    }
    pub fn view(&self) -> Element<GraphMessage> {
        Canvas::new(self).width(Fill).height(Fill).into()
    }

    pub fn insert_node(&mut self, node: GraphNode) {
        self.nodes.push(node);
    }

    pub fn on_event<'a>(
        &self,
        event: canvas::Event,
        state: &'a mut GraphInteraction,
        message: Option<GraphMessage>,
    ) -> (canvas::event::Status, Option<GraphMessage>, &'a mut GraphInteraction) {
        let status = Status::Ignored;

        match event {
            canvas::Event::Mouse(mouse_event) => self.on_mouse_event(mouse_event, message, status, state),
            canvas::Event::Keyboard(keyboard_event) => self.on_keyboard_event(keyboard_event, message, status, state),
            _ => {
                println!("Unhandled event: {:?}", event);
                (status, message, state)
            }
        }
    }

    fn on_keyboard_event<'a>(
        &self,
        event: keyboard::Event,
        message: Option<GraphMessage>,
        mut status: Status,
        state: &'a mut GraphInteraction,
    ) -> (canvas::event::Status, Option<GraphMessage>, &'a mut GraphInteraction) {
        match event {
            keyboard::Event::KeyPressed { key, .. } => match key.as_ref() {
                keyboard::Key::Character("n") => {
                    *state = GraphInteraction::PhantomGraphNode;
                    status = Status::Captured;
                }
                _ => {}
            },
            _ => {}
        }

        (status, message, state)
    }
    fn on_mouse_event<'a>(
        &self,
        event: mouse::Event,
        mut message: Option<GraphMessage>,
        mut status: Status,
        state: &'a mut GraphInteraction,
    ) -> (canvas::event::Status, Option<GraphMessage>, &'a mut GraphInteraction) {
        match event {
            mouse::Event::ButtonReleased(button) => match button {
                mouse::Button::Left => {
                    if state == &GraphInteraction::PhantomGraphNode {
                        message = Some(GraphMessage::InsertNode(GraphNode::new(self.cursor_position)));
                        *state = GraphInteraction::None;
                        status = Status::Captured;
                        return (status, message, state);
                    }
                    println!("Clicked at: {:?}", self.cursor_position);
                    self.nodes.iter().for_each(|node| {
                        if node.is_inside(self.cursor_position) {
                            println!("Clicked Node ID: {}", node.id);
                            *state = GraphInteraction::None;
                            message = if node.selected {
                                Some(GraphMessage::DeselectNode(node.id))
                            } else {
                                Some(GraphMessage::SelectNode(node.id))
                            };
                            status = Status::Captured;
                        }
                    })
                }
                _ => {}
            },
            _ => {}
        }

        (status, message, state)
    }
}

impl canvas::Program<GraphMessage> for Graph {
    type State = GraphInteraction;

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        _bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<GraphMessage>) {
        let mut message = None;
        let status = Status::Ignored;

        let cursor_position = match cursor.position() {
            Some(cursor_position) => {
                if cursor_position != self.cursor_position {
                    message = Some(GraphMessage::MoveCursor(cursor_position));
                }
                cursor_position
            }
            None => return (status, message),
        };

        if state != &GraphInteraction::PhantomGraphNode {
            for node in &self.nodes {
                if node.is_inside(cursor_position) {
                    *state = GraphInteraction::HoverGraphNode(node.id);
                } else {
                    *state = GraphInteraction::None;
                }
            }
        }

        let (status, message, _) = self.on_event(event, state, message);

        (status, message)
    }

    fn draw(
        &self,
        interaction: &GraphInteraction,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        for node in self.nodes.iter() {
            node.draw(&mut frame, interaction);
        }

        match interaction {
            GraphInteraction::PhantomGraphNode => {
                frame.fill_rectangle(
                    self.cursor_position,
                    Size::new(100.0, 100.0),
                    Color { a: 0.2, ..Color::WHITE },
                );
            }
            _ => {}
        };
        vec![frame.into_geometry()]
    }
}
