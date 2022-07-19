mod gameboy;
use gameboy::GameBoy;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1600;
const HEIGHT: usize = 900;

fn main() {
    // Get our Game Boy
    let mut game_boy = GameBoy::new();

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
    //window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // Insert a cartridge
    game_boy.insert_cartridge("../GameboyEmulator/test_roms/cpu_instrs.gb");

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Simulate one frame on Game Boy
        game_boy.run_frame();

        // Render parts of memory
        game_boy.render_memory(&mut buffer, 0x8000, 16, 24, 8, 8, 2);
        game_boy.render_memory(&mut buffer, 0x0000, 32, 32, 272, 8, 1);
        game_boy.render_memory(&mut buffer, 0x4000, 32, 32, 536, 8, 1);
        game_boy.render_screen(&mut buffer, 792, 8);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
    std::fs::write("times.bin", game_boy.times).expect("Couldnt write to file");
}
