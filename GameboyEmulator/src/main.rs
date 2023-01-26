mod gameboy;
use std::{env, time::Instant};

use gameboy::GameBoy;
use mini_gl_fb::{
    config,
    glutin::{dpi::LogicalSize, event_loop::EventLoop},
};

extern crate mini_gl_fb;

const DEBUG_WIDTH: usize = 1280;
const DEBUG_HEIGHT: usize = 720;
const DEBUG_VIEW_ENABLE: bool = true;
const WIDTH: usize = 642;
const HEIGHT: usize = 578;

fn main() {
    // Get our Game Boy
    let mut game_boy = GameBoy::new();

    // Create a window
    let mut buffer: Vec<u32> = match DEBUG_VIEW_ENABLE {
        true => vec![0; DEBUG_WIDTH * DEBUG_HEIGHT],
        false => vec![0; WIDTH * HEIGHT],
    };

    let mut event_loop = EventLoop::new();

    let mut window = match DEBUG_VIEW_ENABLE {
        true => mini_gl_fb::get_fancy(
            config! {
                window_title: String::from("Flan's Game Boy Emulator"),
                window_size: LogicalSize::new(DEBUG_WIDTH as _, DEBUG_HEIGHT as _),
                buffer_size: Some(LogicalSize::new(DEBUG_WIDTH as _, DEBUG_HEIGHT as _)),
                invert_y: false,
            },
            &event_loop,
        ),
        false => mini_gl_fb::get_fancy(
            config! {
                window_title: String::from("Flan's Game Boy Emulator"),
                window_size: LogicalSize::new(WIDTH as _, HEIGHT as _),
                buffer_size: Some(LogicalSize::new(WIDTH as _, HEIGHT as _)),
                invert_y: false,
            },
            &event_loop,
        ),
    };

    // Insert a cartridge
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        game_boy.insert_cartridge(args[1].as_str());
    } else {
        game_boy.insert_cartridge("");
    }

    let mut now = Instant::now();

    // Main loop
    window.glutin_handle_basic_input(&mut event_loop, |window, basic_input| {
        // Handle delta time
        if now.elapsed().as_secs_f32() < 69905_f32 / (1 << 22) as f32 {
            return true;
        }

        now = Instant::now();

        // Get input
        game_boy.update_input(basic_input);

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
        window.update_buffer(&buffer);

        true
    });
    game_boy.save_game_if_possible();
    println!("Shut down");
}
