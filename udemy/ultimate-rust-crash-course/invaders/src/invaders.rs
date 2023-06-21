use std::{time::Duration, cmp::max};

use rusty_time::Timer;

use crate::{NUM_COLS, NUM_ROWS, frame::{Drawable, Frame}};

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if x > 1 && x < NUM_COLS - 2 && y > 0 && y < 9 && x % 3 == 0 && y % 3 == 0 {
                    army.push(Invader { x, y })
                }
            }
        }
        Self {
            army: army,
            move_timer: Timer::from_millis(2000),
            direction: 1,
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);

        if self.move_timer.ready {
            self.move_timer.reset();

            // Decide direction of movement
            let mut downwards = false;
            if self.direction == -1 {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }

            // Move position
            if downwards {
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                for invader in self.army.iter_mut(){
                    invader.y += 1;
                }
            }else{
                for invader in &mut self.army.iter_mut(){
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }
            return true;
        }

        false
    }

    /// Return weather all invaders are killed
    pub fn all_killed(&self) -> bool{
        self.army.is_empty()
    }

    /// Return weather any of the invaders reached the bottom
    pub fn reached_bottom(&self) -> bool {
        for invader in self.army.iter(){
            if invader.y == NUM_ROWS - 1{
                return true;
            }
        }
        false
    }

    pub fn kill_invader(&mut self, x: usize, y: usize) -> bool{
        if let Some(index) = self.army.iter().position(|invader| invader.x == x  && invader.y == y ){
            self.army.remove(index);
            return true
        }
        false
    }
}


impl Drawable for Invaders{
    /// Draw the invaders in the screen
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter(){
            frame[invader.x][invader.y] = if self.move_timer.time_left.as_secs_f32() / self.move_timer.duration.as_secs_f32() > 0.5 {
                "x"
            }else{
                "+"
            }
        }
    }
}