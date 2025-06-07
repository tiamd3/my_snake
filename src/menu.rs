use std::vec;

use ggez::{ event, glam, graphics::{self, Drawable, TextFragment}, Context, GameError, GameResult };
use crate::game;
use crate::level::LevelSelect;

#[derive(Debug, Clone)]
pub struct MenuManager {
    pub main: MainMenu,
    pub level: LevelSelect,
    pub state: MenuState,
}

#[derive(Debug, Clone, Copy)]
pub enum MenuState {
    Main,
    Level,
}

impl MenuManager {
    pub fn new() -> Self{
        Self {
            main: MainMenu::new(),
            level: LevelSelect::new(),
            state: MenuState::Main,
        }
    }
}

impl event::EventHandler<GameError> for MenuManager {
    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        match self.state {
            MenuState::Main => {
                self.main.draw(ctx)
            }
            MenuState::Level => {
                Ok(())
            }
        }
    }

    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct MainMenu {
    pub selected: usize,
    pub options: Vec<&'static str>
}

impl MainMenu {
    pub fn new() -> Self {
        MainMenu {
            selected: 0,
            options: vec!["Start Game", "Exit"],
        }
    }

    pub fn move_selection(&mut self, up: bool) {
        if up {
            self.selected = (self.selected + self.options.len() - 1) % self.options.len();
        } else {
            self.selected = (self.selected + 1) % self.options.len();
        }
    }
}

impl event::EventHandler<GameError> for MainMenu {

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut y = 200.0;

        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 1.0, 0.0, 1.0]));
        let font_size = 50.0;
        for (i, option) in self.options.iter().enumerate() {
            let color = if i == self.selected {
                [1.0, 1.0, 0.0, 1.0]
            } else {
                [1.0, 1.0, 1.0, 1.0]
            };
            let text = graphics::Text::new(
                TextFragment::new((*option).to_string()).scale(font_size)
            );
            let rect = text.dimensions(ctx).unwrap();
            let pos = glam::Vec2::new(game::SCREEN_SIZE.0 / 2.0 - rect.w / 2.0, y);
            canvas.draw(&text, graphics::DrawParam::new().dest(pos).color(color));
            y += 50.0;
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }
}
