use iced::{
    widget::canvas::{Frame, Geometry, Stroke},
    Color, Point, Rectangle, Renderer, Size,
};
use uuid::Uuid;

use super::GraphInteraction;

#[derive(Debug, Clone)]
pub struct GraphNode {
    pub id: u128,
    pub anchor: Point,
    pub size: Size,
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

    pub fn draw(&self, renderer: &Renderer, bounds: &Rectangle, interaction: &GraphInteraction) -> Geometry {
        let color = if interaction == &GraphInteraction::HoverGraphNode(self.id) {
            Color { a: 0.5, ..Color::WHITE }
        } else {
            Color::WHITE
        };
        let mut frame = Frame::new(renderer, bounds.size());
        frame.fill_rectangle(self.anchor, self.size, color);
        frame.stroke_rectangle(
            self.anchor,
            self.size,
            Stroke::default()
                .with_width(2.0)
                .with_color(Color::from_rgb(255.0, 0.0, 0.0)),
        );
        frame.into_geometry()
    }
}
