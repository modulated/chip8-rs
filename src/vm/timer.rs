use crate::SOUND;
use macroquad::audio::{play_sound, PlaySoundParams};
impl super::VM {
    pub fn run_timers(&mut self) {
        if self.delay_timer != 0 {
            self.delay_timer -= 1;
        }
        match (self.sound_timer == 0, self.sound_playing) {
            (true, true) => {
                // STOP SOUND
                self.sound_playing = false;
                macroquad::audio::stop_sound(*SOUND.get().expect("could not get sound"));
            }
            (true, false) => {
                // NOP
            }
            (false, false) => {
                // START SOUND
                self.sound_playing = true;
                play_sound(
                    *SOUND.get().expect("could not get sound"),
                    PlaySoundParams {
                        looped: true,
                        volume: 0.2,
                    },
                );
                self.sound_timer -= 1;
            }
            (false, true) => {
                // CONTINUE SOUND
                self.sound_timer -= 1;
            }
        }
    }
}
