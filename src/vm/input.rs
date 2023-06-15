use macroquad::prelude::*;

static KEYMAP: [KeyCode; 16] = [
    KeyCode::X,    // 0
    KeyCode::Key1, // 1
    KeyCode::Key2, // 2
    KeyCode::Key3, // 3
    KeyCode::Q,    // 4
    KeyCode::W,    // 5
    KeyCode::E,    // 6
    KeyCode::A,    // 7
    KeyCode::S,    // 8
    KeyCode::D,    // 9
    KeyCode::Z,    // A
    KeyCode::C,    // B
    KeyCode::Key4, // C
    KeyCode::R,    // D
    KeyCode::F,    // E
    KeyCode::V,    // F
];

impl super::VM {
    pub fn get_input(&mut self) {
        for (i, k) in KEYMAP.iter().enumerate() {
            if is_key_down(*k) {
                self.key[i] = true;
                // println!("PRESSED {i}");
            } else {
                self.key[i] = false;
            }
        }

        if is_key_down(KeyCode::Escape) {
            std::process::exit(0);
        }
    }
}
