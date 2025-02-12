pub trait RenderSurface {
    fn set(&mut self, pos: (usize, usize), value: char);
}
