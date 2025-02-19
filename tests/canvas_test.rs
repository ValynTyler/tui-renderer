use tui_renderer::terminal::canvas::Canvas;

#[test]
fn test_canvas_width() {
    const MAX_LEN: usize = 200;
    for i in 1..MAX_LEN {
        for j in 1..MAX_LEN {
            let canvas = Canvas::new((j, i), '.');
            println!("{}", canvas.width());
            assert_eq!(j, canvas.width());
        }
    }
}

#[test]
fn test_canvas_height() {
    const MAX_LEN: usize = 200;
    for i in 0..MAX_LEN {
        for j in 0..MAX_LEN {
            let canvas = Canvas::new((j, i), '.');
            assert_eq!(i, canvas.height());
        }
    }
}

#[test]
fn test_canvas_size() {
    const MAX_LEN: usize = 200;
    for i in 1..MAX_LEN {
        for j in 1..MAX_LEN {
            let canvas = Canvas::new((j, i), '.');
            assert_eq!((j, i), canvas.size());
        }
    }
}

#[test]
fn test_canvas_get() {
    let canvas = Canvas::new((50, 10), '.');

    for i in 0..10 {
        for j in 0..50 {
            assert_eq!('.', canvas.get((j, i)))
        }
    }
}

#[test]
fn test_canvas_set() {
    let mut canvas = Canvas::new((50, 10), '.');

    for i in 0..10 {
        for j in 0..50 {
            let pos = (j, i);
            assert_eq!('.', canvas.get(pos));
            canvas.set(pos, '$');
            assert_eq!('$', canvas.get(pos));
        }
    }
}
