mod menu;
mod game;
mod level;
mod audio;

use std::{env, path};

use ggez::{
    event, glam, 
    graphics::{self, Color, Drawable, TextFragment}, 
    input::keyboard::KeyCode,
    Context, GameError, GameResult
};
use game::GameState;
use menu::{MenuManager, MenuState};
use audio::AudioManager;

#[derive(Debug, Clone, Copy)]
enum AppScene {
    Menu,
    Playing,
    Pause,
    GameOver,
}

struct AppState {
    scene: AppScene,
    menu: MenuManager,
    game: Option<GameState>,
    audio: AudioManager,
}


impl AppState {
    fn new(ctx: &mut Context) -> Self {
        let mut audio = AudioManager::new();
        let _= audio.load_sfx(ctx, "eat", "eat.ogg");
        let _= audio.load_sfx(ctx, "die", "die.ogg");

        let _= audio.play_bgm(ctx, "bgm.mp3", true);

        Self {
            scene: AppScene::Menu,
            menu: MenuManager::new(),
            game: Some(GameState::new()),
            audio,
        }
    }
}

impl event::EventHandler<GameError> for AppState {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        match self.scene {
            AppScene::Menu => Ok(()),
            AppScene::Playing => {
                if let Some(game) = &mut self.game {
                    game.update(ctx, &mut self.audio, &mut self.scene)
                } else {
                    Ok(())
                }
            }
            AppScene::Pause => Ok(()),
            AppScene::GameOver => Ok(()),
        }
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        match self.scene {
            AppScene::Menu => {
                self.menu.draw(ctx)
            }
            AppScene::Playing => {
                if let Some(game) = &mut self.game {
                    game.draw(ctx)
                } else {
                    Ok(())
                }
            }
            AppScene::Pause => {
                let mut canvas =
                        graphics::Canvas::from_frame(ctx, graphics::Color::from_rgb(15, 15, 28));
                // if let Some(game) = &mut self.game {
                //     game.draw(&mut canvas);
                // }   
                let y = 200.0;
                let text_c = graphics::Text::new(
                    TextFragment::new("Continue".to_string()).scale(45.0)
                );
                let rect_c = text_c.dimensions(ctx).unwrap();
                let pos_c = glam::Vec2::new(game::SCREEN_SIZE.0 / 2.0 - rect_c.w / 2.0, y);
                let text_q = graphics::Text::new(
                    TextFragment::new("Quit".to_string()).scale(45.0)
                );
                let rect_q = text_q.dimensions(ctx).unwrap();
                let pos_q = glam::Vec2::new(game::SCREEN_SIZE.0 / 2.0 - rect_q.w / 2.0, y + 100.0);
                canvas.draw(&text_c, graphics::DrawParam::new().dest(pos_c).color(Color::from_rgb(200, 34, 32)));
                canvas.draw(&text_q, graphics::DrawParam::new().dest(pos_q).color(Color::from_rgb(200, 34, 32)));
                canvas.finish(ctx)?;
                Ok(())
            }
            AppScene::GameOver => {
                let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(0, 244, 0));
                let text = graphics::Text::new(
                    TextFragment::new("Gram Over! Press R to Restart".to_string()).scale(35.0)
                );
                let pos = glam::Vec2::new(100.0, 100.0);
                canvas.draw(&text, graphics::DrawParam::new().dest(pos).color([1.0, 0.0, 0.0, 1.0]));
                canvas.finish(ctx)?;
                Ok(())
            }
        }
    }

    fn key_down_event(
            &mut self,
            ctx: &mut ggez::Context,
            input: ggez::input::keyboard::KeyInput,
            _repeated: bool,
        ) -> Result<(), GameError> {
        match self.scene {
            AppScene::Menu => {
                match self.menu.state {
                    MenuState::Main => {
                        let main = &mut self.menu.main;
                        if let Some(code) = input.keycode {
                            match code {
                                KeyCode::Up => main.move_selection(true),
                                KeyCode::Down => main.move_selection(false),
                                KeyCode::Return => {
                                    if main.selected == 0 {
                                        self.scene = AppScene::Playing;
                                        self.game = Some(GameState::new());
                                    } else {
                                        ctx.request_quit();
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    MenuState::Level => {}
                }
                
            }
            AppScene::GameOver => {
                if input.keycode == Some(KeyCode::R) {
                    self.scene = AppScene::Menu;
                }
            }
            AppScene::Pause => {

            }
            AppScene::Playing => {
                if input.keycode == Some(KeyCode::Escape) {
                    self.scene = AppScene::Pause;
                }
                if let Some(game) = &mut self.game {
                    game.key_down_event(ctx, input, false)?;
                }
            }
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let resource_dir = if let Ok(maiifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(maiifest_dir);
        path.push("assert");
        path.push("audio");
        println!("{:?}", path);
        path
    } else {
        println!("re");
        path::PathBuf::from("./resources")
    };
    
    let cb = ggez::ContextBuilder::new("snake", "Gray Olson")
        
        .window_setup(ggez::conf::WindowSetup::default().title("Snake!"))
        
        .window_mode(ggez::conf::WindowMode::default().dimensions(game::SCREEN_SIZE.0, game::SCREEN_SIZE.1))
        
        .add_resource_path(resource_dir);

    let (mut ctx, events_loop) = cb.build()?;

    let state = AppState::new(&mut ctx);
    
    event::run(ctx, events_loop, state)
}