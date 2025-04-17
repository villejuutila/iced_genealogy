use iced::{
    widget::canvas::{self, Frame, Stroke},
    Color, Point, Size,
};
use uuid::Uuid;

pub trait GraphNodeTrait {
    fn id(&self) -> u128;
    fn anchor(&self) -> Point;
    fn set_anchor(&mut self, anchor: Point);
    fn size(&self) -> Size;
    fn draw<'a>(&self, frame: &'a mut Frame, hovered: bool) -> Vec<&'a Frame>;
    fn is_in_bounds(&self, point: Point) -> bool {
        let anchor = self.anchor();
        let size = self.size();
        point.x >= anchor.x
            && point.x <= anchor.x + size.width
            && point.y >= anchor.y
            && point.y <= anchor.y + size.height
    }
}

impl std::fmt::Debug for dyn GraphNodeTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GraphNodeTrait {}", self.id())
    }
}

#[derive(Debug, Clone)]
pub enum GraphNodeType {
    GenealogicalNode(GenealogicalNode),
}

impl GraphNodeType {
    pub fn id(&self) -> u128 {
        match self {
            GraphNodeType::GenealogicalNode(node) => node.id(),
        }
    }
    pub fn anchor(&self) -> Point {
        match self {
            GraphNodeType::GenealogicalNode(node) => node.anchor(),
        }
    }
    pub fn set_anchor(&mut self, anchor: Point) {
        match self {
            GraphNodeType::GenealogicalNode(node) => node.set_anchor(anchor),
        }
    }
    pub fn size(&self) -> Size {
        match self {
            GraphNodeType::GenealogicalNode(node) => node.size(),
        }
    }
    pub fn is_in_bounds(&self, point: Point) -> bool {
        match self {
            GraphNodeType::GenealogicalNode(node) => node.is_in_bounds(point),
        }
    }
    pub fn draw<'a>(&self, frame: &'a mut Frame, hovered: bool) -> Vec<&'a Frame> {
        match self {
            GraphNodeType::GenealogicalNode(node) => node.draw(frame, hovered),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sex {
    Male,
    Female,
}

#[derive(Debug, Clone)]
pub struct GenealogicalNode {
    id: u128,
    anchor: Point,
    size: Size,
    sex: Option<Sex>,
    first_name: Option<String>,
    last_name: Option<String>,
}

impl GenealogicalNode {
    pub fn new(anchor: Point) -> Self {
        Self {
            anchor,
            id: Uuid::new_v4().as_u128(),
            size: Size::new(100.0, 100.0),
            sex: None,
            first_name: None,
            last_name: None,
        }
    }

    fn text_position(&self) -> Point {
        Point::new(
            self.anchor.x + self.size.width / 2.0,
            self.anchor.y + self.size.height / 2.0,
        )
    }
    pub fn sex(&self) -> Option<Sex> {
        self.sex.clone()
    }
    pub fn set_sex(&mut self, sex: Sex) {
        self.sex = Some(sex)
    }
    pub fn first_name(&self) -> Option<String> {
        self.first_name.clone()
    }
    pub fn set_first_name(&mut self, first_name: String) {
        self.first_name = Some(first_name);
    }
}

impl GraphNodeTrait for GenealogicalNode {
    fn id(&self) -> u128 {
        self.id
    }

    fn anchor(&self) -> Point {
        self.anchor
    }

    fn set_anchor(&mut self, anchor: Point) {
        self.anchor = anchor;
    }

    fn size(&self) -> Size {
        self.size
    }

    fn draw<'a>(&self, frame: &'a mut Frame, hovered: bool) -> Vec<&'a Frame> {
        let color = if hovered {
            Color { a: 0.5, ..Color::WHITE }
        } else {
            Color::WHITE
        };
        frame.fill_rectangle(self.anchor, self.size, color);
        if let Some(sex) = self.sex() {
            let border_color = if sex == Sex::Male {
                Color::from_rgb(0.0, 0.0, 255.0)
            } else {
                Color::from_rgb(255.0, 0.0, 0.0)
            };
            frame.stroke_rectangle(
                self.anchor,
                self.size,
                Stroke::default().with_width(2.0).with_color(border_color),
            );
        }
        let mut name = String::new();
        if self.first_name.is_some() {
            name.push_str(&self.first_name.as_ref().unwrap());
        }
        if self.last_name.is_some() {
            name.push_str(&self.last_name.as_ref().unwrap());
        }
        frame.fill_text(canvas::Text {
            content: name,
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
}
