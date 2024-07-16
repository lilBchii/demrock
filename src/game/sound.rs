use macroquad::audio::{play_sound, stop_sound, PlaySoundParams, Sound};

#[derive(Clone)]
pub struct MusicParams {
    pub sound: Sound,
    pub is_playing: bool,
    pub is_activated: bool,
    pub volume: f32,
}

pub fn play_music(params: &mut MusicParams) {
    if !params.is_playing && params.is_activated {
        play_sound(
            &params.sound,
            PlaySoundParams {
                looped: true,
                volume: params.volume,
            },
        );
        params.is_playing = true;
    } else if params.is_playing && !params.is_activated {
        stop_sound(&params.sound);
        params.is_playing = false;
    }
}
