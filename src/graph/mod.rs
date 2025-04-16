pub mod node;

use iced::{
    event::Status,
    keyboard,
    mouse::{self, ScrollDelta},
    widget::{
        canvas::{self, Frame, Path, Stroke},
        Canvas,
    },
    Color, Element,
    Length::Fill,
    Point, Rectangle, Renderer, Size, Theme,
};
use node::GraphNodeType;

#[derive(Debug, Clone, PartialEq)]
pub struct GraphState {
    cursor_position: Point,
    scale: f32,
    offset_x: f32,
    offset_y: f32,
}

impl Default for GraphState {
    fn default() -> Self {
        Self {
            cursor_position: Point::ORIGIN,
            scale: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum GraphMessage {
    InsertNode(Option<u128>),
    ClickNode((u128, mouse::Event)),
    UpdateBounds(Rectangle),
}

#[derive(Debug, Clone)]
pub struct Edge {
    start: u128,
    end: u128,
}

// impl Edge {}

pub struct Graph {
    pub cursor_position: Point,
    nodes: Vec<GraphNodeType>,
    tick: u128,
    bounds: Rectangle,
    edges: Vec<Edge>,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            cursor_position: Point::ORIGIN,
            nodes: vec![],
            tick: 0,
            bounds: Rectangle::new(Point::ORIGIN, Size::new(0.0, 0.0)),
            edges: vec![],
        }
    }
}

impl Graph {
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
    pub fn set_bounds(&mut self, bounds: Rectangle) {
        self.bounds = bounds;
    }
    pub fn nodes(&self) -> &Vec<GraphNodeType> {
        &self.nodes
    }
    pub fn set_selected_node(&mut self, node_id: u128) {
        for node in self.nodes.iter_mut() {
            if node.id() == node_id {
                node.set_selected(true);
            } else {
                node.set_selected(false);
            }
        }
    }

    pub fn deselect_node(&mut self, node_id: u128) {
        for node in self.nodes.iter_mut() {
            if node.id() == node_id {
                node.set_selected(false);
            }
        }
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

    pub fn on_event<'c>(
        &self,
        event: canvas::Event,
        state: &'c mut GraphState,
        message: Option<GraphMessage>,
    ) -> (canvas::event::Status, Option<GraphMessage>, &'c mut GraphState) {
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

    fn on_keyboard_event<'c>(
        &self,
        event: keyboard::Event,
        message: Option<GraphMessage>,
        status: Status,
        state: &'c mut GraphState,
    ) -> (canvas::event::Status, Option<GraphMessage>, &'c mut GraphState) {
        match event {
            _ => {}
        }

        (status, message, state)
    }
    fn on_mouse_event<'c>(
        &self,
        event: mouse::Event,
        mut message: Option<GraphMessage>,
        mut status: Status,
        state: &'c mut GraphState,
    ) -> (canvas::event::Status, Option<GraphMessage>, &'c mut GraphState) {
        match event {
            mouse::Event::CursorMoved { position } => {
                state.cursor_position = self.to_canvas_point(state, position);
            }
            mouse::Event::WheelScrolled { delta } => {
                let y = match delta {
                    ScrollDelta::Lines { x: _, y } => y,
                    ScrollDelta::Pixels { x: _, y } => y,
                };
                println!("delta: {:?}", delta);
                status = Status::Captured;
                let scale_amount = -y / 500.0;
                println!("scale_amount: {}", scale_amount);
                state.scale = state.scale * (1.0 + scale_amount);
            }
            mouse::Event::ButtonPressed(button) => match button {
                mouse::Button::Left => {
                    self.nodes.iter().for_each(|node| {
                        if node.is_in_bounds(self.cursor_position) {
                            message = Some(GraphMessage::ClickNode((node.id(), event)));
                        };
                        status = Status::Captured;
                    });
                }
                _ => {}
            },
            _ => {}
        }

        (status, message, state)
    }

    fn to_canvas_point(&self, state: &GraphState, point: Point) -> Point {
        Point::new(self.to_canvas_x(state, point.x), self.to_canvas_y(state, point.y))
    }

    #[allow(dead_code)]
    fn to_window_point(&self, state: &GraphState, point: Point) -> Point {
        Point::new(self.to_window_x(state, point.x), self.to_window_y(state, point.y))
    }

    fn to_canvas_x(&self, state: &GraphState, window_x: f32) -> f32 {
        (window_x + state.offset_x) * state.scale
    }

    fn to_canvas_y(&self, state: &GraphState, window_y: f32) -> f32 {
        (window_y + state.offset_y) * state.scale
    }
    #[allow(dead_code)]
    fn to_window_x(&self, state: &GraphState, canvas_x: f32) -> f32 {
        canvas_x / state.scale - state.offset_x
    }

    #[allow(dead_code)]
    fn to_window_y(&self, state: &GraphState, canvas_y: f32) -> f32 {
        canvas_y / state.scale - state.offset_y
    }

    #[allow(dead_code)]
    fn true_height(&self, state: &GraphState) -> f32 {
        self.bounds.height / state.scale
    }

    #[allow(dead_code)]
    fn true_width(&self, state: &GraphState) -> f32 {
        self.bounds.width / state.scale
    }
}

impl canvas::Program<GraphMessage> for Graph {
    type State = GraphState;

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<GraphMessage>) {
        let mut message = None;

        if bounds != self.bounds {
            message = Some(GraphMessage::UpdateBounds(bounds));
        }

        let (status, message, _) = self.on_event(event, state, message);

        (status, message)
    }

    fn draw(
        &self,
        state: &GraphState,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        println!("real cursor_position: {:#?}", _cursor.position());
        println!("canvas cursor_position{}", state.cursor_position);
        println!("Scale : {}", state.scale);
        frame.scale(state.scale);
        for edge in self.edges() {
            let start = self.get_node_unsafe(Some(edge.start));
            let end = self.get_node_unsafe(Some(edge.end));
            // let start_above = start.anchor().y <= end.anchor().y;
            let start_point = Point::new(
                start.anchor().x + start.size().width / 2.0,
                start.anchor().y + start.size().height,
            );
            let end_point = Point::new(end.anchor().x + end.size().width / 2.0, end.anchor().y);
            println!("start: {:#?}", start_point);
            println!("end: {:#?}", end_point);
            let path = Path::line(start_point, end_point);
            frame.stroke(&path, Stroke::default().with_width(2.0).with_color(Color::WHITE));
        }
        for node in self.nodes.iter() {
            node.draw(&mut frame);
        }
        vec![frame.into_geometry()]
    }
}
