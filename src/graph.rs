use iced::{
    event::Status,
    mouse,
    widget::{
        canvas::{self, Frame, Stroke},
        Canvas,
    },
    Color, Element,
    Length::Fill,
    Point, Rectangle, Renderer, Size, Theme,
};
use uuid::Uuid;

use crate::context_menu::ContextMenu;

#[derive(Debug, Clone)]
pub struct GraphNode {
    id: u128,
    anchor: Point,
    size: Size,
}

impl GraphNode {
    pub fn new(anchor: Point) -> Self {
        Self {
            id: Uuid::new_v4().as_u128(),
            anchor,
            size: Size::new(100.0, 100.0),
        }
    }

    pub fn is_inside(&self, point: Point) -> bool {
        point.x >= self.anchor.x
            && point.x <= self.anchor.x + self.size.width
            && point.y >= self.anchor.y
            && point.y <= self.anchor.y + self.size.height
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GraphInteraction {
    None,
    HoverGraphNode(u128),
    ShowContextMenu(Point),
}

impl Default for GraphInteraction {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone)]
pub enum GraphMessage {
    InsertNode(GraphNode),
}

pub struct Graph {
    nodes: Vec<GraphNode>,
}

impl Default for Graph {
    fn default() -> Self {
        Self { nodes: vec![] }
    }
}

impl Graph {
    pub fn view(&self) -> Element<GraphMessage> {
        Canvas::new(self).width(Fill).height(Fill).into()
    }

    pub fn insert_node(&mut self, node: GraphNode) {
        self.nodes.push(node);
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

        let Some(cursor_position) = cursor.position() else {
            return (status, message);
        };
        for node in &self.nodes {
            if node.is_inside(cursor_position) {
                *state = GraphInteraction::HoverGraphNode(node.id);
                status = Status::Captured;
                return (status, message);
            }
        }
        match event {
            canvas::Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonReleased(button) => match button {
                    mouse::Button::Left => message = Some(GraphMessage::InsertNode(GraphNode::new(cursor_position))),
                    mouse::Button::Right => {
                        *state = GraphInteraction::ShowContextMenu(cursor_position);
                        status = Status::Captured;
                    }
                    _ => *state = GraphInteraction::None,
                },
                _ => *state = GraphInteraction::None,
            },
            _ => *state = GraphInteraction::None,
        }
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
        println!("{:?}", interaction);
        let mut frames = self
            .nodes
            .iter()
            .map(|node| {
                let color = if interaction == &GraphInteraction::HoverGraphNode(node.id) {
                    Color { a: 0.5, ..Color::WHITE }
                } else {
                    Color::WHITE
                };
                let mut frame = Frame::new(renderer, bounds.size());
                frame.fill_rectangle(node.anchor, node.size, color);
                frame.stroke_rectangle(
                    node.anchor,
                    node.size,
                    Stroke::default()
                        .with_width(2.0)
                        .with_color(Color::from_rgb(255.0, 0.0, 0.0)),
                );
                frame.into_geometry()
            })
            .collect::<Vec<_>>();
        match interaction {
            GraphInteraction::ShowContextMenu(point) => {
                let mut context_menu = ContextMenu::draw(*point, renderer, bounds);
                frames.append(&mut context_menu);
            }
            _ => {}
        };
        frames
    }
}
