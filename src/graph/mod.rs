mod node;

use iced::{
    event::Status,
    keyboard, mouse,
    widget::{
        canvas::{self, Frame, Stroke},
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
        mut message: Option<GraphMessage>,
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
                        let node = GraphNode::new(self.cursor_position);
                        message = Some(GraphMessage::InsertNode(node));
                        *state = GraphInteraction::None;
                    }
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
        let mut status = Status::Ignored;

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
                    status = Status::Captured;
                    return (status, message);
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
        let mut frames = self
            .nodes
            .iter()
            .map(|node| node.draw(renderer, &bounds, interaction))
            .collect::<Vec<_>>();
        match interaction {
            GraphInteraction::PhantomGraphNode => {
                let mut frame = Frame::new(renderer, bounds.size());
                frame.fill_rectangle(
                    self.cursor_position,
                    Size::new(100.0, 100.0),
                    Color { a: 0.2, ..Color::WHITE },
                );
                frames.push(frame.into_geometry());
            }
            _ => {}
        };
        frames
    }
}
