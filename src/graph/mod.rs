pub mod node;

use iced::{
    event::Status,
    keyboard, mouse,
    widget::{
        canvas::{self, Frame, Path, Stroke},
        Canvas,
    },
    Color, Element,
    Length::Fill,
    Point, Rectangle, Renderer, Size, Theme,
};
use node::{GraphNodeTrait, GraphNodeType};

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
    InsertNode(Option<u128>),
    MoveCursor(Point),
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
    selected_node: Option<u128>,
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
            selected_node: None,
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
    pub fn nodes_mut(&mut self) -> &mut Vec<GraphNodeType> {
        &mut self.nodes
    }
    pub fn selected_node(&self) -> Option<u128> {
        self.selected_node
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
        state: &'c mut GraphInteraction,
        message: Option<GraphMessage>,
    ) -> (canvas::event::Status, Option<GraphMessage>, &'c mut GraphInteraction) {
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
        mut status: Status,
        state: &'c mut GraphInteraction,
    ) -> (canvas::event::Status, Option<GraphMessage>, &'c mut GraphInteraction) {
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
    fn on_mouse_event<'c>(
        &self,
        event: mouse::Event,
        mut message: Option<GraphMessage>,
        mut status: Status,
        state: &'c mut GraphInteraction,
    ) -> (canvas::event::Status, Option<GraphMessage>, &'c mut GraphInteraction) {
        match event {
            mouse::Event::ButtonPressed(button) => match button {
                mouse::Button::Left => {
                    self.nodes.iter().for_each(|node| {
                        if node.is_in_bounds(self.cursor_position) {
                            *state = GraphInteraction::None;
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
}

impl canvas::Program<GraphMessage> for Graph {
    type State = GraphInteraction;

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
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
        if bounds != self.bounds {
            message = Some(GraphMessage::UpdateBounds(bounds));
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

        for edge in self.edges() {
            let start = self.get_node_unsafe(Some(edge.start));
            let end = self.get_node_unsafe(Some(edge.end));
            let start_above = start.anchor().y <= end.anchor().y;
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
            // GraphInteraction::DrawPathFromNode((_, start)) => {
            //     let cursor_position = self.cursor_position;
            //     let mut dx = cursor_position.x - start.x;
            //     let mut dy = cursor_position.y - start.y;
            //     let mut length = (dx.abs().powf(2.0) + dy.abs().powf(2.0)).sqrt();
            //     if length == 0.0 {
            //         dx = 1.0;
            //         dy = 1.0;
            //         length = 1.0;
            //     }
            //     let center = Point::new(start.x - (dx / length) * 5.0, start.y - (dy / length) * 5.0);
            //     let x_distance = (cursor_position.x - start.x).abs();
            //     let y_distance = (cursor_position.y - start.y).abs();
            //     let c = if x_distance < y_distance {
            //         Point::new(center.x, cursor_position.y)
            //     } else {
            //         Point::new(cursor_position.x, center.y)
            //     };
            //     let path_1 = Path::line(*start, c);
            //     let path_2 = Path::line(c, cursor_position);
            //     let mut color = Color::WHITE;
            //     if self.tick & 1 == 0 {
            //         color.a = 0.1;
            //     }
            //     let stroke = Stroke::default().with_color(color).with_width(1.0);
            //     frame.stroke(&path_1, stroke);
            //     frame.stroke(&path_2, stroke);
            // }
            _ => {}
        };
        vec![frame.into_geometry()]
    }
}
