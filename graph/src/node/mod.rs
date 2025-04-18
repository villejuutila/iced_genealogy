use iced::{widget::canvas::Frame, Color, Point, Size};

pub trait GraphNodeTrait {
    fn new(anchor: Point) -> Self;
    fn id(&self) -> u128;
    fn anchor(&self) -> Point;
    fn set_anchor(&mut self, anchor: Point);
    fn size(&self) -> Size;
    fn draw_content<'a>(&self, frame: &'a mut Frame) -> &'a Frame;

    fn draw<'a>(&self, frame: &'a mut Frame, hovered: bool) -> Vec<&'a Frame> {
        let color = if hovered {
            Color { a: 0.5, ..Color::WHITE }
        } else {
            Color::WHITE
        };
        frame.fill_rectangle(self.anchor(), self.size(), color);
        self.draw_content(frame);
        vec![frame]
    }

    fn is_in_bounds(&self, point: Point) -> bool {
        let anchor = self.anchor();
        let size = self.size();
        point.x >= anchor.x
            && point.x <= anchor.x + size.width
            && point.y >= anchor.y
            && point.y <= anchor.y + size.height
    }
}
