use tui_renderer::terminal::canvas::Canvas;

fn main() {
    let canvas = Canvas::new(10, 4);

    println!("{}", canvas);
    println!("{}x{}", canvas.width(), canvas.height());
}
