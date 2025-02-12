use tui_renderer::terminal::canvas::Canvas;

fn main() {
    let mut canvas = Canvas::new(90, 14, '.');
    let mut square = Canvas::new(4, 4, 'x');
    square.set(0, 0, '*');
    square.set(1, 1, '*');
    square.set(2, 2, '*');
    square.set(3, 3, '*');

    println!("{:?}", square.chars());

    println!("{}", canvas);
    println!("{}", square);
}
