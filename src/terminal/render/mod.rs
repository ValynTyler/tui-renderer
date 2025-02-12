use super::canvas::Canvas;

pub trait Renderable {
    fn render(&self, position: (usize, usize), canvas: &mut Canvas);
}
