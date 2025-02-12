use super::surface::RenderSurface;

pub trait RenderElement {
    fn get(&self, pos: (usize, usize)) -> char;
    fn render<T>(&self, pos: (usize, usize), surface: &mut T)
        where T: RenderSurface;
}
