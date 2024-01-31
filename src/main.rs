use minifb::{Key, Scale, Window, WindowOptions};

const SCREEN_A_WIDTH: usize = 512;
const SCREEN_A_HEIGHT: usize = 256;
const SCREEN_B_WIDTH: usize = 16;
const SCREEN_B_HEIGHT: usize = 1;

fn main() {
    let screen_a = Window::new(
        "Screen A",
        SCREEN_A_WIDTH,
        SCREEN_A_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();
    let mut screen_b = Window::new(
        "Screen B",
        SCREEN_B_WIDTH,
        SCREEN_B_HEIGHT,
        WindowOptions {
            scale: Scale::FitScreen,
            ..WindowOptions::default()
        },
    )
    .unwrap();
    let buffer: Vec<u32> = vec![0; SCREEN_B_WIDTH * SCREEN_B_HEIGHT];
    while screen_b.is_open() && !screen_b.is_key_down(Key::Escape) {
        screen_b
            .update_with_buffer(&buffer, SCREEN_B_WIDTH, SCREEN_B_HEIGHT)
            .unwrap();
    }
}
