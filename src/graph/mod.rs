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
use node::{GenealogicalNode, GraphNode, GraphNodeTrait, GraphNodeType};

#[derive(Debug, Clone, PartialEq)]
pub enum GraphInteraction {
    None,
    HoverGraphNode(u128),
    PhantomGraphNode,
    DrawPathFromNode((u128, Point)),
}

impl Default for GraphInteraction {
    fn default() -> Self {
        Self::None
    }
}

trait Test: GraphNodeTrait + Send + Clone + std::fmt::Debug {}

#[derive(Debug, Clone)]
pub enum GraphMessage {
    InsertNode(Point),
    MoveCursor(Point),
    ClickNode((u128, mouse::Event)), // SelectNode(u128),
                                     // DeselectNode(u128),
}

pub struct Graph {
    pub cursor_position: Point,
    selected_node: Option<u128>,
    nodes: Vec<GraphNodeType>,
    tick: u128,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            cursor_position: Point::ORIGIN,
            nodes: vec![],
            tick: 0,
            selected_node: None,
        }
    }
}

impl Graph {
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

    // pub fn toggle_node_selection(&mut self, node_id: u128) {
    //     if let Some(node) = self.nodes.iter_mut().find(|n| n.id == node_id) {
    //         node.selected = !node.selected;
    //     }
    // }
    pub fn view(&self) -> Element<GraphMessage> {
        Canvas::new(self).width(Fill).height(Fill).into()
    }

    pub fn insert_node(&mut self, node: GraphNodeType) {
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
        message: Option<GraphMessage>,
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
            mouse::Event::ButtonPressed(button) => match button {
                mouse::Button::Left => {
                    if state == &GraphInteraction::PhantomGraphNode {
                        message = Some(GraphMessage::InsertNode(self.cursor_position));
                        *state = GraphInteraction::None;
                        status = Status::Captured;
                        return (status, message, state);
                    }

                    self.nodes.iter().for_each(|node| {
                        if node.is_in_bounds(self.cursor_position) {
                            println!("Clicked on node: {:?}", node);
                            *state = GraphInteraction::None;
                            message = Some(GraphMessage::ClickNode((node.id(), event)));
                        };
                        status = Status::Captured;
                    });

                    // self.nodes.iter().for_each(|node| {
                    //     if let Some(anchor) = node.is_on_anchor(self.cursor_position) {
                    //         *state = GraphInteraction::DrawPathFromNode((node.id, anchor));
                    //         status = Status::Captured;
                    //     }
                    // });
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

    // fn mouse_interaction(
    //     &self,
    //     _state: &Self::State,
    //     _bounds: Rectangle,
    //     _cursor: iced::advanced::mouse::Cursor,
    // ) -> iced::advanced::mouse::Interaction {
    //     if self
    //         .nodes
    //         .iter()
    //         .any(|node| node.is_on_anchor(self.cursor_position).is_some())
    //     {
    //         iced::advanced::mouse::Interaction::Pointer
    //     } else {
    //         iced::advanced::mouse::Interaction::Idle
    //     }
    // }

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        _bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<GraphMessage>) {
        println!("cursor_position {:?}", self.cursor_position);
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

        // if state != &GraphInteraction::PhantomGraphNode {
        //     for node in &self.nodes {
        //         if node.is_inside(cursor_position) {
        //             *state = GraphInteraction::HoverGraphNode(node.id);
        //         } else {
        //             // *state = GraphInteraction::None;
        //         }
        //     }
        // }

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
            GraphInteraction::DrawPathFromNode((_, start)) => {
                let cursor_position = self.cursor_position;
                println!("cursor_position: {:?}", cursor_position);
                println!("start: {:?}", start);
                let mut dx = cursor_position.x - start.x;
                let mut dy = cursor_position.y - start.y;
                println!("dx: {}, dy: {}", dx, dy);
                let mut length = (dx.abs().powf(2.0) + dy.abs().powf(2.0)).sqrt();
                if length == 0.0 {
                    dx = 1.0;
                    dy = 1.0;
                    length = 1.0;
                }
                println!("Length {}", length);
                println!("X {}", start.x - (dx / length) * 5.0);
                let center = Point::new(start.x - (dx / length) * 5.0, start.y - (dy / length) * 5.0);
                println!("center: {:?}", center);
                let x_distance = (cursor_position.x - start.x).abs();
                let y_distance = (cursor_position.y - start.y).abs();
                let c = if x_distance < y_distance {
                    Point::new(center.x, cursor_position.y)
                } else {
                    Point::new(cursor_position.x, center.y)
                };
                let path_1 = Path::line(*start, c);
                let path_2 = Path::line(c, cursor_position);
                let mut color = Color::WHITE;
                if self.tick & 1 == 0 {
                    color.a = 0.1;
                }
                let stroke = Stroke::default().with_color(color).with_width(1.0);
                frame.stroke(&path_1, stroke);
                frame.stroke(&path_2, stroke);
            }
            _ => {}
        };
        vec![frame.into_geometry()]
    }
}
