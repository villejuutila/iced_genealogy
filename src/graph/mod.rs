pub mod node;

use iced::{
    event::Status,
    mouse::{self},
    widget::{
        canvas::{self, Frame, Path, Stroke},
        Canvas,
    },
    Color, Element,
    Length::Fill,
    Point, Rectangle, Renderer, Size, Theme, Vector,
};
use node::GraphNodeType;

#[derive(Debug, Clone, PartialEq)]
pub enum GraphInteraction {
    None,
    Panning { translation: Vector, start: Point },
    HoverNode(u128),
}

impl Default for GraphInteraction {
    fn default() -> Self {
        Self::None
    }
}
#[derive(Debug, Clone)]
pub enum GraphMessage {
    InsertNode(Option<u128>),
    ClickNode((u128, mouse::Event)),
    Scaled(f32, Option<Vector>),
    Translated(Vector),
}

#[derive(Debug, Clone)]
pub struct Edge {
    start: u128,
    end: u128,
}

#[derive(Debug, Clone)]
pub struct Region {
    x: f32,
    y: f32,
}

// impl Edge {}

pub struct Graph {
    nodes: Vec<GraphNodeType>,
    tick: u128,
    bounds: Rectangle,
    edges: Vec<Edge>,
    scaling: f32,
    translation: Vector,
    selected_node: Option<u128>,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            nodes: vec![],
            tick: 0,
            bounds: Rectangle::new(Point::ORIGIN, Size::new(0.0, 0.0)),
            edges: vec![],
            scaling: 1.0,
            translation: Vector::default(),
            selected_node: None,
        }
    }
}

impl Graph {
    const MIN_SCALING: f32 = 0.1;
    const MAX_SCALING: f32 = 2.0;

    fn visible_region(&self, size: Size) -> Region {
        let width = size.width / self.scaling;
        let height = size.height / self.scaling;

        Region {
            x: self.translation.x - width / 2.0,
            y: self.translation.y - height / 2.0,
        }
    }

    fn project(&self, position: Point, size: Size) -> Point {
        let region = self.visible_region(size);

        Point::new(
            position.x / self.scaling - region.x,
            position.y / self.scaling - region.y,
        )
    }

    pub fn get_node(&self, node_id: Option<u128>) -> Option<&GraphNodeType> {
        self.nodes.iter().find(|node| node.id() == node_id.unwrap_or(0))
    }
    pub fn get_node_unsafe(&self, node_id: Option<u128>) -> &GraphNodeType {
        self.get_node(node_id).unwrap()
    }
    pub fn get_node_mut(&mut self, node_id: Option<u128>) -> Option<&mut GraphNodeType> {
        self.nodes.iter_mut().find(|node| node.id() == node_id.unwrap_or(0))
    }
    pub fn get_node_mut_unsafe(&mut self, node_id: Option<u128>) -> &mut GraphNodeType {
        self.get_node_mut(node_id).unwrap()
    }
    pub fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    pub fn add_edge_between_nodes(&mut self, start_node_id: Option<u128>, end_node_id: u128) {
        if let None = start_node_id {
            return;
        }
        self.edges.push(Edge {
            start: start_node_id.unwrap(),
            end: end_node_id,
        });
    }

    pub fn bounds(&self) -> Rectangle {
        self.bounds
    }
    pub fn set_selected_node(&mut self, node_id: u128) {
        self.selected_node = Some(node_id);
    }

    #[allow(dead_code)]
    pub fn deselect_node(&mut self) {
        self.selected_node = None;
    }

    pub fn selected_node(&self) -> Option<&GraphNodeType> {
        self.nodes.iter().find(|node| self.selected_node == Some(node.id()))
    }

    pub fn tick(&mut self) {
        self.tick += 1;
    }

    pub fn view(&self) -> Element<GraphMessage> {
        Canvas::new(self).width(Fill).height(Fill).into()
    }

    pub fn insert_node(&mut self, node: GraphNodeType) {
        self.nodes.push(node);
    }

    pub fn update(&mut self, message: GraphMessage) {
        match message {
            GraphMessage::Scaled(scaling, translation) => {
                self.scaling = scaling;
                if let Some(translation) = translation {
                    self.translation = translation;
                }
            }
            GraphMessage::Translated(translation) => {
                self.translation = translation;
            }
            GraphMessage::ClickNode((node_id, _)) => {
                self.set_selected_node(node_id);
            }
            GraphMessage::InsertNode(edge_node_id) => {
                let mut center = self.project(self.bounds().center(), self.bounds().size());
                if let Some(found_node) = self.get_node(edge_node_id) {
                    center = found_node.anchor();
                    center.y += found_node.size().height * 2.0;
                }
                let new_node = GraphNodeType::GenealogicalNode(node::GenealogicalNode::new(center));
                self.add_edge_between_nodes(edge_node_id, new_node.id());
                self.insert_node(new_node);
            }
        }
    }

    pub fn window_to_canvas(&self, window_pos: Point, bounds: Rectangle) -> Point {
        let center = Vector::new(bounds.width / 2.0, bounds.height / 2.0);

        let translated = window_pos - center;
        let scaled = Point::new(translated.x * (1.0 / self.scaling), translated.y * (1.0 / self.scaling));
        scaled - self.translation
    }
}

impl canvas::Program<GraphMessage> for Graph {
    type State = GraphInteraction;

    fn update(
        &self,
        interaction: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<GraphMessage>) {
        let mut status = Status::Ignored;
        let mut message = None;
        let cursor_position = cursor.position_in(bounds).unwrap_or(Point::ORIGIN);
        match event {
            canvas::Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::CursorMoved { .. } => match *interaction {
                    GraphInteraction::Panning { translation, start } => {
                        message = Some(GraphMessage::Translated(
                            translation + (cursor_position - start) * (1.0 / self.scaling),
                        ));
                        status = Status::Captured;
                    }
                    _ => {
                        if let Some(hovered_node) = cursor.position_in(bounds).map_or(None, |cursor_position| {
                            let canvas_position = self.window_to_canvas(cursor_position, bounds);
                            self.nodes.iter().find(|node| node.is_in_bounds(canvas_position))
                        }) {
                            *interaction = GraphInteraction::HoverNode(hovered_node.id());
                            status = Status::Captured;
                        } else {
                            *interaction = GraphInteraction::None;
                            status = Status::Ignored;
                        }
                    }
                },
                mouse::Event::WheelScrolled { delta } => match delta {
                    mouse::ScrollDelta::Lines { y, .. } | mouse::ScrollDelta::Pixels { y, .. } => {
                        if y < 0.0 && self.scaling > Self::MIN_SCALING || y > 0.0 && self.scaling < Self::MAX_SCALING {
                            let old_scaling = self.scaling;
                            let scaling = (self.scaling * (1.0 + y / 30.0)).clamp(Self::MIN_SCALING, Self::MAX_SCALING);
                            let translation = if let Some(cursor_to_center) = cursor.position_from(bounds.center()) {
                                let factor = scaling - old_scaling;

                                Some(
                                    self.translation
                                        - Vector::new(
                                            cursor_to_center.x * factor / (old_scaling * old_scaling),
                                            cursor_to_center.y * factor / (old_scaling * old_scaling),
                                        ),
                                )
                            } else {
                                None
                            };

                            message = Some(GraphMessage::Scaled(scaling, translation))
                        }
                    }
                },
                mouse::Event::ButtonReleased(button) => match button {
                    mouse::Button::Right => {
                        *interaction = GraphInteraction::None;
                        status = Status::Ignored;
                    }
                    _ => {}
                },
                mouse::Event::ButtonPressed(button) => match button {
                    mouse::Button::Right => {
                        *interaction = GraphInteraction::Panning {
                            translation: self.translation,
                            start: cursor_position,
                        };
                        status = Status::Captured;
                    }
                    mouse::Button::Left => {
                        if let GraphInteraction::HoverNode(id) = *interaction {
                            message = Some(GraphMessage::ClickNode((id, mouse_event)));
                            status = Status::Captured;
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        (status, message)
    }

    fn draw(
        &self,
        interaction: &GraphInteraction,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let hovered_node = cursor.position_in(bounds).map_or(None, |cursor_position| {
            let canvas_position = self.window_to_canvas(cursor_position, bounds);
            self.nodes.iter().find(|node| node.is_in_bounds(canvas_position))
        });
        println!("Hovered node: {:#?}", hovered_node);
        let center = Vector::new(bounds.width / 2.0, bounds.height / 2.0);
        let mut frame = Frame::new(renderer, bounds.size());
        frame.with_save(|mut frame| {
            frame.translate(center);
            frame.scale(self.scaling);
            frame.translate(self.translation);
            for node in self.nodes.iter() {
                let hovered = if let GraphInteraction::HoverNode(id) = *interaction {
                    id == node.id()
                } else {
                    false
                };
                node.draw(&mut frame, hovered);
            }
            for edge in self.edges() {
                let start = self.get_node_unsafe(Some(edge.start));
                let end = self.get_node_unsafe(Some(edge.end));
                // let start_above = start.anchor().y <= end.anchor().y;
                let start_point = Point::new(
                    start.anchor().x + start.size().width / 2.0,
                    start.anchor().y + start.size().height,
                );
                let end_point = Point::new(end.anchor().x + end.size().width / 2.0, end.anchor().y);
                let path = Path::line(start_point, end_point);
                frame.stroke(&path, Stroke::default().with_width(2.0).with_color(Color::WHITE));
            }
        });
        vec![frame.into_geometry()]
    }
}
