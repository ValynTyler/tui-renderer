use tui_renderer::terminal::canvas::*;

fn main() {
    let mut square = Canvas::new(5, 4, 'x');
    let string: Canvas = "Hello, world!\nfoo\nbar".into();

    square.set((0, 0), '0');
    square.set((1, 1), '1');
    square.set((2, 2), '2');
    square.set((3, 3), '3');

    let canvas = Canvas::new(91, 15, '.');

    let x = canvas.width() / 2 - string.width() / 2;
    let y = canvas.height() / 2 - string.height() / 2;
    let canvas = canvas
        .draw((-1, 5), &square)
        .draw((86, 12), &string)
        .draw((x as isize, y as isize), &string);

    println!("{}", canvas);
}
