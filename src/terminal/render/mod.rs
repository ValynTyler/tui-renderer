use super::canvas::Canvas;

pub trait Renderable {
    fn render(&self, position: (usize, usize), canvas: &mut Canvas);
}

impl Renderable for &str {
    fn render(&self, position: (usize, usize), canvas: &mut Canvas) {
        self.lines().enumerate().for_each(|(i, s)| {
            s.chars().enumerate().for_each(|(j, c)| {
                let x = position.0 + j;
                let y = position.1 + i;

                if x < canvas.width()
                && y < canvas.height() {
                    canvas.set((x, y), c);
                }
            });
        });
    }
}
