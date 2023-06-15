use crate::{PIXEL_HEIGHT, PIXEL_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH};

impl super::VM {
    pub fn set_pixel(&mut self, x: u8, y: u8) -> bool {
        let idx = x as usize + (SCREEN_WIDTH as usize * y as usize);
        if self.screen[idx] {
            self.screen[idx] = false;
            true
        } else {
            self.screen[idx] = true;
            false
        }
    }

    pub fn draw_screen(&self) {
        use macroquad::prelude::*;
        clear_background(BLACK);
        for (i, b) in self.screen.iter().enumerate() {
            let x = i % SCREEN_WIDTH as usize;
            let y = i / SCREEN_WIDTH as usize;
            let (x, y) = world_to_screen(x, y);
            if *b {
                draw_rectangle(x, y, PIXEL_WIDTH, PIXEL_HEIGHT, GREEN);
            }
        }
    }

    pub fn set_screen_border(&mut self) {
        for i in 0..SCREEN_WIDTH {
            self.screen[i as usize] = true;
            self.screen[((SCREEN_HEIGHT - 1) * SCREEN_WIDTH + i) as usize] = true;
        }
        for i in 0..SCREEN_HEIGHT {
            self.screen[(i * SCREEN_WIDTH) as usize] = true;
            self.screen[(i * SCREEN_WIDTH + SCREEN_WIDTH - 1) as usize] = true;
        }
    }
}

// world is 0,0 -> 64,32, screen is 0,0 ->
fn world_to_screen(x: usize, y: usize) -> (f32, f32) {
    let out_x = x as f32 * PIXEL_WIDTH;
    let out_y = y as f32 * PIXEL_HEIGHT;
    (out_x, out_y)
}
