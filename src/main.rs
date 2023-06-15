use chip8::{VM, WINDOW_HEIGHT, WINDOW_WIDTH};
use macroquad::window::Conf;
#[macroquad::main(conf)]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Please supply ROM file as argument");
    }

    let mut vm = VM::new();
    chip8::SOUND
        .set(macroquad::audio::load_sound("buzz.wav").await.unwrap())
        .unwrap();
    match vm.load_program(&args[1]) {
        Ok(_) => vm.run().await,
        Err(e) => panic!("Could not load ROM: {e}"),
    }
}

fn conf() -> Conf {
    Conf {
        window_title: "CHIP-8".to_string(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}
