use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};

use rand::Rng;
use std::collections::VecDeque;
use std::io::{Stdout, Write};
use std::thread;
use std::time::{Duration, Instant};

type Point = (u16, u16);

struct Game {
    snake: VecDeque<Point>,
    food: Point,
    direction: (i16, i16),
    next_direction: (i16, i16),
    score: u32,
    level: u16,
    base_tick_ms: u64,
    width: u16,
    height: u16,
    is_game_over: bool,
}

impl Game {
    fn new(level: u16, base_tick_ms: u64) -> Self {
        let width = 40;
        let height = 20;
        let start_pos = (width / 2, height / 2);

        let mut snake = VecDeque::new();
        snake.push_front(start_pos);

        let mut game = Game {
            snake,
            food: (0, 0),
            direction: (1, 0),
            next_direction: (1, 0),
            score: 0,
            level,
            base_tick_ms,
            width,
            height,
            is_game_over: false,
        };
        game.spawn_food();
        game
    }

    fn spawn_food(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(1..self.width - 1);
            let y = rng.gen_range(1..self.height - 1);
            if !self.snake.contains(&(x, y)) {
                self.food = (x, y);
                break;
            }
        }
    }

    fn get_current_tick_ms(&self) -> u64 {
        //Speed increases with level, minimum 50ms
        let speed_bonus = (self.level.saturating_sub(1) * 20) as u64;
        self.base_tick_ms.saturating_sub(speed_bonus).max(50)
    }
}
