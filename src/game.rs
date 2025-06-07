use oorandom::Rand32;
use rand::{self, TryRngCore};

use ggez::{
    audio::Source, graphics::{self, Color}, input::keyboard::{KeyCode, KeyInput}, Context, GameResult
};

use std::collections::VecDeque;

use crate::AppScene;
use crate::audio::AudioManager;

pub const GRID_SIZE: (i32, i32) = (30, 20);

pub const GRID_CELL_SIZE: (i32, i32) = (32, 32);

pub const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

pub const DESIRED_FPS: u32 = 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl GridPos {
 
    pub fn new(x: i32, y: i32) -> Self {
        GridPos { x, y }
    }

    pub fn random(rng: &mut Rand32, max_x: i32, max_y: i32) -> Self {
        (
            rng.rand_range(0..(max_x as u32)) as i32,
            rng.rand_range(0..(max_y as u32)) as i32,
        )
            .into()
    }
    
    fn new_from_move(pos: GridPos, dir: Direction) -> Self {
        match dir {
            Direction::Up => GridPos::new(pos.x, (pos.y - 1).rem_euclid(GRID_SIZE.1)),
            Direction::Down => GridPos::new(pos.x, (pos.y + 1).rem_euclid(GRID_SIZE.1)),
            Direction::Left => GridPos::new((pos.x - 1).rem_euclid(GRID_SIZE.0), pos.y),
            Direction::Right => GridPos::new((pos.x + 1).rem_euclid(GRID_SIZE.0), pos.y),
        }
    }
}

impl From<GridPos> for graphics::Rect {
    fn from(pos: GridPos) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}

impl From<(i32, i32)> for GridPos {
    fn from(pos: (i32, i32)) -> Self {
        GridPos { x: pos.0, y: pos.1 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
  
    pub fn inverse(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn from_keycode(key: KeyCode) -> Option<Direction> {
        match key {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Segment {
    pos: GridPos,
}

impl Segment {
    pub fn new(pos: GridPos) -> Self {
        Segment { pos }
    }
}

#[derive(Debug, Clone)]
struct Obsta {
    data: Vec<GridPos>,
}

#[derive(Debug, Clone)]
struct Obstacles {
    data: Vec<Obsta>,
}

#[derive(Debug, Clone)]
struct Food {
    pos: GridPos,
}

impl Food {
    pub fn new(pos: GridPos) -> Self {
        Food { pos }
    }
    
    fn draw(&self, canvas: &mut graphics::Canvas) {
     
        let color = [0.0, 0.0, 1.0, 1.0];
    
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.pos.into())
                .color(color),
        );
    }
}

#[derive(Clone, Copy, Debug)]
enum Ate {
    Itself,
    Food,
}

#[derive(Debug, Clone)]
struct Snake {
    head: Segment,
    dir: Direction,
    body: VecDeque<Segment>,
    ate: Option<Ate>,
    last_update_dir: Direction,
    next_dir: Option<Direction>,
}

impl Snake {
    pub fn new(pos: GridPos) -> Self {
        let mut body = VecDeque::new();
       
        body.push_back(Segment::new((pos.x - 1, pos.y).into()));
        Snake {
            head: Segment::new(pos),
            dir: Direction::Right,
            last_update_dir: Direction::Right,
            body,
            ate: None,
            next_dir: None,
        }
    }

    fn eats(&self, food: &Food) -> bool {
        self.head.pos == food.pos
    }

    fn eats_self(&self) -> bool {
        for seg in &self.body {
            if self.head.pos == seg.pos {
                return true;
            }
        }
        false
    }

    fn update(&mut self, food: &Food) {
      
        if self.last_update_dir == self.dir && self.next_dir.is_some() {
            self.dir = self.next_dir.unwrap();
            self.next_dir = None;
        }
      
        let new_head_pos = GridPos::new_from_move(self.head.pos, self.dir);
       
        let new_head = Segment::new(new_head_pos);
        
        self.body.push_front(self.head);
        
        self.head = new_head;
       
        if self.eats_self() {
            self.ate = Some(Ate::Itself);
        } else if self.eats(food) {
            self.ate = Some(Ate::Food);
        } else {
            self.ate = None;
        }
        
        if self.ate.is_none() {
            self.body.pop_back();
        }
        
        self.last_update_dir = self.dir;
    }

    fn draw(&self, canvas: &mut graphics::Canvas) {
        
        for seg in &self.body {
         
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(seg.pos.into())
                    .color(Color::from_rgb(92, 43, 117)),
            );
        }
        
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.head.pos.into())
                .color(Color::from_rgb(236, 64, 122)),
        );
    }
}

pub struct GameState {
    snake: Snake,
    food: Food,
    rng: Rand32,
    gameover: bool,
}

impl GameState {
    
    pub fn new() -> Self {
     
       let snake_pos = (GRID_SIZE.0 / 4, GRID_SIZE.1 / 2).into();
        
        let mut seed = [0u8; 8];
        let _ = rand::rngs::OsRng.try_fill_bytes(&mut seed);
        let mut rng = Rand32::new(u64::from_ne_bytes(seed));
        
        let food_pos = GridPos::random(&mut rng, GRID_SIZE.0, GRID_SIZE.1);

        GameState {
            snake: Snake::new(snake_pos),
            food: Food::new(food_pos),
            gameover: false,
            rng,
        }
    }

    pub fn update(&mut self, ctx: &mut Context, audio: &mut AudioManager, app_state: &mut AppScene) -> GameResult {
        
        while ctx.time.check_update_time(DESIRED_FPS) {
            
            if !self.gameover {
                
                self.snake.update(&self.food);
                
                if let Some(ate) = self.snake.ate {
                    
                    match ate {
            
                       Ate::Food => {
                            audio.play_sfx("eat", ctx);
                            let new_food_pos =
                                GridPos::random(&mut self.rng, GRID_SIZE.0, GRID_SIZE.1);
                            self.food.pos = new_food_pos;
                        }
                        
                        Ate::Itself => {
                            audio.play_sfx("die", ctx);
                            *app_state = AppScene::GameOver;
                            self.gameover = true;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from_rgb(15, 15, 28));

        self.snake.draw(&mut canvas);
        self.food.draw(&mut canvas);
  
        canvas.finish(ctx)?;
   
        ggez::timer::yield_now();
        
        Ok(())
    }

    pub fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
       
        if let Some(dir) = input.keycode.and_then(Direction::from_keycode) {
          
            if self.snake.dir != self.snake.last_update_dir && dir.inverse() != self.snake.dir {
                self.snake.next_dir = Some(dir);
            } else if dir.inverse() != self.snake.last_update_dir {
              
                self.snake.dir = dir;
            }
        }
        Ok(())
    }
}
