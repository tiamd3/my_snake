use std::{collections::HashMap, time::Duration};

use ggez::{audio::{SoundSource, Source}, GameResult, Context};

pub enum BgmState {
    Play,
    Pause,
    Empty,
}




struct BgmManager {
    music: Option<Source>,
    state: BgmState,
}

impl BgmManager {
    fn new() -> Self {
         Self { music: None, state: BgmState::Empty }
    }

    fn new_bgm(ctx: &mut Context, path: &str, repeat: bool, volume: f32) -> GameResult<Self> {
        println!("init");
        let mut music = Source::new(ctx, path)?;
        println!("true");
        music.set_repeat(repeat);
        music.set_volume(volume);
        let _ = music.play(ctx)?;
        Ok(Self {
            music: Some(music),
            state: BgmState::Play
        })
    }

    fn stop_bgm(&mut self, ctx: &mut Context) {
        if let Some(m) = &mut self.music {
            self.state = BgmState::Empty;
            let _ = m.stop(ctx);
        }
        self.music = None;
    }

    fn resume_bgm(&mut self, ctx: &mut Context) {
        if let Some(m) = &mut self.music {
            if let BgmState::Pause = self.state {
                self.state = BgmState::Play;
                let _ = m.resume();
            }
        }
    }

    fn pause_bgm(&mut self, ctx: &mut Context) {
        if let Some(m) = &mut self.music {
            self.state = BgmState::Pause;
            let _ = m.pause();
        }
    }

    fn replay_bgm(&mut self, ctx: &mut Context) {
        if let Some(m) = &mut self.music {
            let _ = m.set_start(Duration::ZERO);
            let _ = m.play(ctx);
        }
    }
}
pub struct AudioManager {
    bgm: BgmManager,
    music_volume: f32,
    sfx: HashMap<String, Source>,
    sfx_volume: f32,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            bgm: BgmManager::new(),
            music_volume: 0.5,
            sfx: HashMap::new(),
            sfx_volume: 0.7,
        }
    }

    pub fn play_bgm(&mut self, ctx: &mut Context, path: &str, repeat: bool) -> GameResult {
        self.bgm = BgmManager::new_bgm(ctx, path, repeat, self.music_volume)?;
        Ok(())
    }

    pub fn stop_bgm(&mut self, ctx: &mut Context) {
        self.bgm.stop_bgm(ctx);
    }

    pub fn replay_bgm(&mut self, ctx: &mut Context) {
        self.bgm.replay_bgm(ctx);
    }

   pub fn load_sfx(&mut self, ctx: &mut Context, name: &str, path: &str) -> ggez::GameResult {
        let mut sfx = Source::new(ctx, path)?;
        sfx.set_volume(self.sfx_volume);
        self.sfx.insert(name.to_string(), sfx);
        Ok(())
    }

    pub fn play_sfx(&mut self, name: &str, ctx: &mut Context) {
        if let Some(sfx) = self.sfx.get_mut(name) {
            let _ = sfx.play(ctx);
        }
    }
}