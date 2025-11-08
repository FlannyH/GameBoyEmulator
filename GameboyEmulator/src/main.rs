use std::{
    env,
    time::{Duration, Instant},
};

use minifb::{Key, Window, WindowOptions};

use crate::gameboy::{GameBoy, InputState};

mod gameboy;

const DEBUG_WIDTH: usize = 1280;
const DEBUG_HEIGHT: usize = 720;
const DEBUG_VIEW_ENABLE: bool = true;
const WIDTH: usize = 642;
const HEIGHT: usize = 578;

fn main() {
    // Create window
    let (w, h) = if DEBUG_VIEW_ENABLE {
        (DEBUG_WIDTH, DEBUG_HEIGHT)
    } else {
        (WIDTH, HEIGHT)
    };

    let mut buffer: Vec<u32> = vec![0; (w * h) as _];

    let mut window = Window::new(
        "Flan's Game Boy Emulator",
        w,
        h,
        WindowOptions {
            resize: false,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open Window");

    // Get our Game Boy
    let mut game_boy = GameBoy::new();

    // Insert a cartridge
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        game_boy.insert_cartridge(args[1].as_str());
    } else {
        game_boy.insert_cartridge("");
    }

    // Keep track of timing and input
    let mut now = Instant::now();
    let mut input_state = InputState::default();

    while window.is_open() {
        // Handle delta time
        let dt = now.elapsed().as_secs_f32();
        let desired_dt = 69905_f32 / (1 << 22) as f32;
        if dt < desired_dt {
            std::thread::sleep(Duration::from_secs_f32(desired_dt - dt));
        }

        now = Instant::now();

        // Get input
        // todo: remappable controls
        input_state.down = window.is_key_down(Key::Down);
        input_state.up = window.is_key_down(Key::Up);
        input_state.left = window.is_key_down(Key::Left);
        input_state.right = window.is_key_down(Key::Right);
        input_state.start = window.is_key_down(Key::Enter);
        input_state.select = window.is_key_down(Key::RightShift);
        input_state.b = window.is_key_down(Key::Z);
        input_state.a = window.is_key_down(Key::X);
        dbg!(&input_state);
        game_boy.update_input(&input_state);

        // Simulate one frame on Game Boy
        game_boy.run_frame();

        // Render parts of memory
        if DEBUG_VIEW_ENABLE {
            game_boy.render_memory(&mut buffer, 0x8000, 16, 24, 8, 8, 2);
            game_boy.render_memory(&mut buffer, 0x0000, 32, 32, 272, 8, 1);
            game_boy.render_memory(&mut buffer, 0x4000, 32, 32, 536, 8, 1);
            game_boy.render_palettes(&mut buffer, 272, 272, 24, DEBUG_WIDTH);
            game_boy.render_screen(&mut buffer, 792, 8, 2, DEBUG_WIDTH);
        } else {
            game_boy.render_screen(&mut buffer, 0, 0, 4, WIDTH)
        }
        window.update_with_buffer(&buffer, w, h).unwrap();
    }
}
