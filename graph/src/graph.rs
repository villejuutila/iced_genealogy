use iced::{
    event::Status,
    mouse::{self},
    widget::{
        canvas::{self, Cache, Path, Stroke},
        Canvas,
    },
    Color, Element,
    Length::Fill,
    Point, Rectangle, Renderer, Size, Theme, Vector,
};

use crate::node::GraphNodeTrait;

#[derive(Debug, Clone, PartialEq)]
pub enum GraphInteraction {
    None,
    Panning { translation: Vector, start: Point },
    HoverNode(u128),
    DraggingNode(u128, Point),
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
    ClickOutsideNode(mouse::Event),
    Scaled(f32, Option<Vector>),
    Translated(Vector),
    DraggingNode(u128, Point),
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

pub struct Graph<T: GraphNodeTrait> {
    nodes: Vec<T>,
    tick: u128,
    bounds: Rectangle,
    edges: Vec<Edge>,
    scaling: f32,
    translation: Vector,
    selected_node: Option<u128>,
    cache: Cache,
}

impl<T: GraphNodeTrait> Default for Graph<T> {
    fn default() -> Self {
        Self {
            nodes: vec![],
            tick: 0,
            bounds: Rectangle::new(Point::ORIGIN, Size::new(0.0, 0.0)),
            edges: vec![],
            scaling: 1.0,
            translation: Vector::default(),
            selected_node: None,
            cache: Cache::default(),
        }
    }
}

impl<T: GraphNodeTrait> Graph<T> {
    const MIN_SCALING: f32 = 0.1;
    const MAX_SCALING: f32 = 2.0;
    const GRID_SIZE: f32 = 32.0;

    pub fn redraw(&mut self) {
        self.cache.clear();
    }

    fn snap_to_grid(pos: Point) -> Point {
        Point::new(
            (pos.x / Self::GRID_SIZE).round() * Self::GRID_SIZE,
            (pos.y / Self::GRID_SIZE).round() * Self::GRID_SIZE,
        )
    }

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

    pub fn get_node(&self, node_id: Option<u128>) -> Option<&T> {
        self.nodes.iter().find(|node| node.id() == node_id.unwrap_or(0))
    }
    pub fn get_node_unsafe(&self, node_id: Option<u128>) -> &T {
        self.get_node(node_id).unwrap()
    }
    pub fn get_node_mut(&mut self, node_id: Option<u128>) -> Option<&mut T> {
        self.nodes.iter_mut().find(|node| node.id() == node_id.unwrap_or(0))
    }
    pub fn get_node_mut_unsafe(&mut self, node_id: Option<u128>) -> &mut T {
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

    pub fn selected_node(&self) -> Option<&T> {
        self.nodes.iter().find(|node| self.selected_node == Some(node.id()))
    }

    pub fn tick(&mut self) {
        self.tick += 1;
    }

    pub fn view(&self) -> Element<GraphMessage> {
        Canvas::new(self).width(Fill).height(Fill).into()
    }

    pub fn insert_node(&mut self, node: T) {
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
                if self.selected_node == Some(node_id) {
                    println!("Deselecting node {}", node_id);
                    self.selected_node = None;
                } else {
                    println!("Selecting node {}", node_id);
                    self.selected_node = Some(node_id);
                }
            }
            GraphMessage::InsertNode(edge_node_id) => {
                let mut center = self.project(self.bounds().center(), self.bounds().size());
                if let Some(found_node) = self.get_node(edge_node_id) {
                    center = found_node.anchor();
                    center.y += found_node.size().height * 2.0;
                }
                let new_node = T::new(center);
                self.add_edge_between_nodes(edge_node_id, new_node.id());
                self.insert_node(new_node);
                self.cache.clear();
            }
            GraphMessage::DraggingNode(id, offset) => {
                let node = self.get_node_mut_unsafe(Some(id));
                node.set_anchor(Self::snap_to_grid(offset));
            }
            GraphMessage::ClickOutsideNode(_) => {
                if let Some(selected_node) = self.selected_node {
                    println!("Deselecting node {}", selected_node);
                    self.selected_node = None;
                }
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

impl<T: GraphNodeTrait> canvas::Program<GraphMessage> for Graph<T> {
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

        let Some(cursor_position) = cursor.position_in(bounds) else {
            return (status, message);
        };

        let canvas_position = self.window_to_canvas(cursor_position, bounds);
        match event {
            canvas::Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::CursorMoved { position } => match *interaction {
                    GraphInteraction::DraggingNode(id, starting_point) => {
                        if (starting_point - position).x.abs() > 1.0 && (starting_point - position).y.abs() > 1.0 {
                            let offset = self.window_to_canvas(position, bounds);
                            message = Some(GraphMessage::DraggingNode(id, offset));
                            status = Status::Captured;
                        }
                    }
                    GraphInteraction::Panning { translation, start } => {
                        message = Some(GraphMessage::Translated(
                            translation + (cursor_position - start) * (1.0 / self.scaling),
                        ));
                        status = Status::Captured;
                    }
                    _ => {
                        if let Some(hovered_node) = self.nodes.iter().find(|node| node.is_in_bounds(canvas_position)) {
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
                    mouse::Button::Left | mouse::Button::Right => {
                        if matches!(
                            interaction,
                            GraphInteraction::DraggingNode(..) | GraphInteraction::Panning { .. }
                        ) {
                            *interaction = GraphInteraction::None;
                            status = Status::Ignored;
                        }
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
                            if let Some(selected_node) = self.selected_node {
                                if selected_node == id {
                                    *interaction = GraphInteraction::DraggingNode(selected_node, cursor_position);
                                    status = Status::Captured;
                                } else {
                                    message = Some(GraphMessage::ClickNode((id, mouse_event)));
                                    status = Status::Captured;
                                }
                            } else {
                                message = Some(GraphMessage::ClickNode((id, mouse_event)));
                                status = Status::Captured;
                            }
                        } else {
                            message = Some(GraphMessage::ClickOutsideNode(mouse_event));
                            status = Status::Captured;
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
        self.cache.clear();
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
        let center = Vector::new(bounds.width / 2.0, bounds.height / 2.0);
        vec![self.cache.draw(renderer, bounds.size(), |frame| {
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
                let stroke = Stroke::default().with_width(2.0).with_color(Color::WHITE);

                for edge in self.edges() {
                    let start_node = self.get_node_unsafe(Some(edge.start));
                    let end_node = self.get_node_unsafe(Some(edge.end));
                    let (start, end) = if start_node.anchor().y > end_node.anchor().y {
                        (end_node, start_node)
                    } else {
                        (start_node, end_node)
                    };

                    let start_point = Point::new(
                        start.anchor().x + start.size().width / 2.0,
                        start.anchor().y + start.size().height,
                    );
                    let end_point = Point::new(end.anchor().x + end.size().width / 2.0, end.anchor().y);
                    let second_point = Point::new(start_point.x, start_point.y + (end_point.y - start_point.y) / 2.0);
                    let third_point = Point::new(end_point.x, second_point.y);

                    let start_path = Path::line(start_point, second_point);
                    let second_path = Path::line(second_point, third_point);
                    let end_path = Path::line(third_point, end_point);

                    frame.stroke(&start_path, stroke);
                    frame.stroke(&second_path, stroke);
                    frame.stroke(&end_path, stroke);
                }
            })
        })]
    }
}
