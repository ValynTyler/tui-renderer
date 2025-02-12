use tui_renderer::terminal::canvas::Canvas;

fn main() {
    let mut canvas = Canvas::new(10, 4);
    canvas.set(0, 0, '0');
    canvas.set(1, 1, '1');
    canvas.set(2, 2, '2');
    canvas.set(3, 3, '3');

    println!("{}", canvas);
    println!("{}x{}", canvas.width(), canvas.height());
}
