use super::canvas::Canvas;

pub trait Renderable {
    fn render(&self, position: (usize, usize), canvas: &mut Canvas);
}

impl Renderable for &str {
    fn render(&self, position: (usize, usize), canvas: &mut Canvas) {
        self.lines().enumerate().for_each(|(i, s)| {
            s.chars().enumerate().for_each(|(j, c)| {
                canvas.set((position.0 + j, position.1 + i), c);
            });
        });
    }
}
