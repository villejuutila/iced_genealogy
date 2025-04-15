use iced::{
    widget::canvas::{self, Frame, Stroke},
    Color, Point, Size,
};
use uuid::Uuid;

use super::GraphInteraction;

const ANCHOR_PADDING: f32 = 20.0;
const ANCHOR_RADIUS: f32 = 5.0;

#[derive(Debug, Clone)]
pub struct GraphNode {
    pub id: u128,
    pub anchor: Point,
    pub size: Size,
    pub selected: bool,
}

impl GraphNode {
    pub fn new(anchor: Point) -> Self {
        Self {
            id: Uuid::new_v4().as_u128(),
            anchor,
            size: Size::new(100.0, 100.0),
            selected: false,
        }
    }

    pub fn is_inside(&self, point: Point) -> bool {
        point.x >= self.anchor.x
            && point.x <= self.anchor.x + self.size.width
            && point.y >= self.anchor.y
            && point.y <= self.anchor.y + self.size.height
    }

    pub fn is_on_anchor(&self, point: Point) -> Option<Point> {
        let anchors = self.anchors();
        anchors
            .iter()
            .find(|anchor| (point.x - anchor.x).abs() < ANCHOR_RADIUS && (point.y - anchor.y).abs() < ANCHOR_RADIUS)
            .cloned()
    }

    fn anchors(&self) -> Vec<Point> {
        vec![
            Point::new(self.anchor.x + self.size.width / 2.0, self.anchor.y - ANCHOR_PADDING),
            Point::new(
                (self.anchor.x + ANCHOR_PADDING) + self.size.width,
                self.anchor.y + self.size.height / 2.0,
            ),
            Point::new(
                self.anchor.x + self.size.width / 2.0,
                self.anchor.y + self.size.height + ANCHOR_PADDING,
            ),
            Point::new(self.anchor.x - ANCHOR_PADDING, self.anchor.y + self.size.height / 2.0),
        ]
    }

    pub fn draw<'a>(&self, frame: &'a mut Frame, interaction: &GraphInteraction) -> Vec<&'a Frame> {
        let hovered = interaction == &GraphInteraction::HoverGraphNode(self.id);
        let color = if hovered {
            Color { a: 0.5, ..Color::WHITE }
        } else {
            Color::WHITE
        };

        if self.selected {
            self.anchors().iter().for_each(|anchor| {
                let circle = canvas::Path::circle(*anchor, ANCHOR_RADIUS);
                frame.fill(&circle, Color::WHITE);
            });
        }
        frame.fill_rectangle(self.anchor, self.size, color);
        frame.stroke_rectangle(
            self.anchor,
            self.size,
            Stroke::default()
                .with_width(2.0)
                .with_color(Color::from_rgb(255.0, 0.0, 0.0)),
        );
        frame.fill_text(canvas::Text {
            content: "Testi".to_string(),
            size: 20.0.into(),
            color: Color {
                a: 1.,
                r: 1.,
                g: 0.,
                b: 0.,
            },
            position: self.text_position(),
            vertical_alignment: iced::alignment::Vertical::Center,
            horizontal_alignment: iced::alignment::Horizontal::Center,
            ..Default::default()
        });

        vec![frame]
    }

    fn text_position(&self) -> Point {
        Point::new(
            self.anchor.x + self.size.width / 2.0,
            self.anchor.y + self.size.height / 2.0,
        )
    }
}
