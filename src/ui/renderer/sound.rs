use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound};

pub struct SoundPlayer {
    win:       Option<Sound>,
    explosion: Option<Sound>,
    flag:      Option<Sound>
}

impl SoundPlayer {
    pub async fn new(win: &[u8], explosion: &[u8], flag: &[u8]) -> SoundPlayer {
        SoundPlayer {
            win:       SoundPlayer::load(win)      .await,
            explosion: SoundPlayer::load(explosion).await,
            flag:      SoundPlayer::load(flag)     .await,
        }
    }

    async fn load(data: &[u8]) -> Option<Sound> {
        let sound = load_sound_from_bytes(data).await;
        if let Err(e) = &sound {
            macroquad::logging::error!("Error '{:?}' loading sound!", e);
        }
        sound.ok()
    }

    fn play(sound: &Option<Sound>, params: PlaySoundParams) {
        if let Some(sound) = sound {
            play_sound(sound, params);
        }
    }

    pub fn play_win(&self) {
        SoundPlayer::play(
            &self.win,
            PlaySoundParams { looped: false, volume: 1.0 },
        );
    }
    pub fn play_explosion(&self) {
        SoundPlayer::play(
            &self.explosion,
            PlaySoundParams { looped: false, volume: 0.3 },
        );
    }
    pub fn play_flag(&self) {
        SoundPlayer::play(
            &self.flag,
            PlaySoundParams { looped: false, volume: 1.0 },
        );
    }
}