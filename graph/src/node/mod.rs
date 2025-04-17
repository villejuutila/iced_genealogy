pub mod genealogical_node;

use iced::{widget::canvas::Frame, Point, Size};

pub trait GraphNodeTrait {
    fn new(anchor: Point) -> Self;
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
