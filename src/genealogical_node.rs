use graph::node::GraphNodeTrait;
use iced::{
    widget::canvas::{Frame, Text},
    Color, Point, Size, Vector,
};
use uuid::Uuid;

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
    const NODE_PADDING_V: f32 = 0.2;
    const NODE_PADDING_H: f32 = 0.2;
    const NODE_FONT_SIZE: f32 = 16.0;

    fn draw_first_name<'a>(&self, frame: &'a mut Frame) -> &'a Frame {
        if let Some(first_name) = &self.first_name {
            let padding_h = self.size().width * Self::NODE_PADDING_H;
            let padding_v = self.size().height * Self::NODE_PADDING_V;

            let anchor = self.anchor() + Vector::new(padding_h, padding_v);
            frame.fill_text(Text {
                content: first_name.clone(),
                size: Self::NODE_FONT_SIZE.into(),
                position: anchor,
                color: Color::BLACK,
                ..Default::default()
            });
        }
        frame
    }

    fn draw_last_name<'a>(&self, frame: &'a mut Frame) -> &'a Frame {
        if let Some(last_name) = &self.last_name {
            let padding_h = self.size().width * Self::NODE_PADDING_H;
            let padding_v = self.size().height * Self::NODE_PADDING_V + Self::NODE_FONT_SIZE + 5.0;

            let anchor = self.anchor() + Vector::new(padding_h, padding_v);
            frame.fill_text(Text {
                content: last_name.clone(),
                size: Self::NODE_FONT_SIZE.into(),
                position: anchor,
                color: Color::BLACK,
                ..Default::default()
            });
        }
        frame
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
    pub fn last_name(&self) -> Option<String> {
        self.last_name.clone()
    }
    pub fn set_last_name(&mut self, last_name: String) {
        self.last_name = Some(last_name);
    }
}

impl GraphNodeTrait for GenealogicalNode {
    fn new(anchor: Point) -> Self {
        Self {
            anchor,
            id: Uuid::new_v4().as_u128(),
            size: Size::new(128.0, 96.0),
            sex: None,
            first_name: None,
            last_name: None,
        }
    }
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

    fn draw_content<'a>(&self, frame: &'a mut Frame) -> &'a Frame {
        self.draw_first_name(frame);
        self.draw_last_name(frame)
    }
}
