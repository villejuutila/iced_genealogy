use iced::{
    advanced::graphics::geometry::Frame,
    widget::canvas::{Geometry, Path, Stroke},
    Color, Point, Rectangle, Renderer, Size,
};

#[derive(Debug, Clone)]
pub enum ContextMenuMessage {}

pub struct ContextMenu {
    anchor: Point,
}

impl ContextMenu {
    pub fn draw(anchor: Point, renderer: &Renderer, bounds: Rectangle) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        frame.fill_rectangle(anchor, Size::new(100.0, 200.0), Color::WHITE);

        let origin = Point::new(anchor.x.round(), anchor.y.round());
        let dividers = (origin.y as i32..origin.y as i32 + 200).step_by(20).map(|y| {
            let mut frame = Frame::new(renderer, bounds.size());
            let from = Point::new(origin.x, y as f32);
            let to = Point::new(origin.x + 100.0, y as f32);
            frame.stroke(
                &Path::line(from, to),
                Stroke::default().with_color(Color::BLACK).with_width(1.0),
            );

            frame.into_geometry()
        });

        let mut geometries = vec![frame.into_geometry()];
        geometries.extend(dividers);
        geometries
    }
}
