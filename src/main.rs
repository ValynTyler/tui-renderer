use tui_renderer::terminal::canvas::*;
use tui_renderer::terminal::render::surface::*;
use tui_renderer::terminal::render::element::*;

fn main() {
    let mut canvas = Canvas::new(90, 14, '.');
    let mut square = Canvas::new(5, 4, 'x');

    square.set((0, 0), '0');
    square.set((1, 1), '1');
    square.set((2, 2), '2');
    square.set((3, 3), '3');

    square.render((5, 5), &mut canvas);
    square.render((15, 0), &mut canvas);
    square.render((60, 9), &mut canvas);

    println!("{}", canvas);
    println!("{}", square);
}
