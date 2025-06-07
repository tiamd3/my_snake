use crate::game::GridPos;

#[derive(Debug, Clone)]
pub struct LevelSelect {
    levels: Vec<Level>,
}

impl LevelSelect {
    pub fn new() -> Self {
        Self { levels: Level::basic_levels() }
    }
}

#[derive(Debug, Clone)]
pub struct Level {
    pub name: &'static str,
    pub obstacles: Vec<GridPos>,
    pub speed: f32,
}

impl Level {
    pub fn basic_levels() -> Vec<Self> {
        vec![
            Level {
                name: "Easy",
                obstacles: vec![],
                speed: 5.0,
            },
            Level {
                name: "Medium",
                obstacles: vec![
                    GridPos::new(10, 10),
                    GridPos::new(11, 10),
                    GridPos::new(12, 10),
                ],
                speed: 8.0,
            },
            Level {
                name: "Hard",
                obstacles: (5..15)
                    .map(|x| GridPos::new(x, 7))
                    .chain((7..12).map(|y| GridPos::new(5, y)))
                    .collect(),
                speed: 12.0,
            },
        ]
    }
}