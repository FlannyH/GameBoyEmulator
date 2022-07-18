mod gameboy;
use gameboy::GameBoy;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const PIXEL_SCALE: usize = 2;

fn main() {
    // Turn on the Game Boy
    let game_boy = GameBoy::new();

    // Create a window
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Simulate one frame on Game Boy
        //todo
        game_boy.render_debug(&mut buffer);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
