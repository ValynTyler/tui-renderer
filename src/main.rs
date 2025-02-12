use tui_renderer::terminal::{canvas::*, render::Renderable};

fn main() {
    let mut canvas = Canvas::new(90, 14, '.');
    let mut square = Canvas::new(5, 4, 'x');
    let string = "Hello, world!\nfoo\nbar";

    square.set((0, 0), '0');
    square.set((1, 1), '1');
    square.set((2, 2), '2');
    square.set((3, 3), '3');

    string.render((5, 5), &mut canvas);
    square.render((15, 0), &mut canvas);
    square.render((60, 9), &mut canvas);
    string.render((20, 10), &mut canvas);

    println!("{}", canvas);
}
