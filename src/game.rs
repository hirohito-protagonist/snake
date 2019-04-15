extern crate piston_window;
extern crate find_folder;

use rand::{thread_rng, Rng};
use piston_window::*;

use crate::draw::{draw_rectangle, draw_block};
use crate::snake::{Snake, Direction};
use crate::food::{Food};
use crate::theme;

pub struct Game {
    snake: Snake,
    width: i32,
    height: i32,
    waiting_time: f64,
    is_game_over: bool,
    food_exist: bool,
    score: i32,
    snake_speed: f64,
    food: Food,
}

impl Game {
    pub fn new(viewport: (u32, u32)) -> Game {
        let (width, height) = viewport;
        Game{
            snake: Snake::new(1, 1),
            width: width as i32,
            height: height as i32,
            waiting_time: 0.0,
            is_game_over: false,
            food_exist: false,
            score: 0,
            snake_speed: 0.1,
            food: Food::new(10, 20),
        }
    }

    pub fn draw(&self, context: &Context, g: &mut G2d, glyphs: &mut piston_window::glyph_cache::rusttype::GlyphCache<GfxFactory, G2dTexture>) {

        if !self.is_game_over {
            self.snake.draw(context, g);
        }

        if self.food_exist {
            self.food.draw(context, g);
        }
        draw_rectangle(theme::BORDER_COLOR, 0, 0, self.width, 1, context, g);
        draw_rectangle(theme::BORDER_COLOR, 0, self.height - 1, self.width, 1, context, g);
        draw_rectangle(theme::BORDER_COLOR, 0, 0, 1, self.height, context, g);
        draw_rectangle(theme::BORDER_COLOR, self.width - 1, 0, 1, self.height, context, g);

        self.render_score(context, g, glyphs);

        if self.is_game_over {
            self.render_game_over(context, g, glyphs);
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.is_game_over {
            match key {
                Key::Space => self.restart(),
                _ => {}
            }
        }
        let direction = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction())
        };

        if direction.unwrap() == self.snake.head_direction().opposite() {
            return;
        }
        self.update_snake(direction);
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        if self.waiting_time > self.snake_speed {
            self.update_snake(None);
        }

        if !self.food_exist {
            self.add_food();
        }
    }

    fn update_snake(&mut self, direction: Option<Direction>) {
        if self.is_snake_alive(direction) {
            self.snake.move_forward(direction);
            self.check_eating(direction);
        } else {
            self.is_game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn is_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.is_tail_collision(next_x, next_y) {
            return false;
        }
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn render_game_over(&self, context: &Context, g: &mut G2d, glyphs: &mut piston_window::glyph_cache::rusttype::GlyphCache<GfxFactory, G2dTexture>) {
        let pos_x = (((self.width * 10) as f32 / 2.0) - 60.0).into();
        let pos_y = (((self.height * 10) as f32 / 2.0) + 16.0).into();
        let game_over_pos = context.transform.trans(pos_x, pos_y);
        let reset_information_pos = context.transform.trans(pos_x - 25.0, pos_y + 18.0);

        text::Text::new_color(theme::TEXT_COLOR, 32).draw(
            "Game Over",
            glyphs,
            &context.draw_state,
            game_over_pos,
            g
        ).unwrap();

        text::Text::new_color(theme::TEXT_COLOR, 18).draw(
            "Press SPACEBAR to reset",
            glyphs,
            &context.draw_state,
            reset_information_pos,
            g
        ).unwrap();
    }

    fn render_score(&self, context: &Context, g: &mut G2d, glyphs: &mut piston_window::glyph_cache::rusttype::GlyphCache<GfxFactory, G2dTexture>) {
        let transform = context.transform.trans((self.width * 10 - 200).into(), (self.height * 10 + 15).into());
        
        text::Text::new_color(theme::TEXT_COLOR, 18).draw(
            &format!("Score: {}", self.score),
            glyphs,
            &context.draw_state,
            transform,
            g
        ).unwrap();
    }

    fn add_food(&mut self) {

        self.food.reposition(self.width, self.height);
        let (food_x, food_y): (i32, i32) = self.food.position();
        while self.snake.is_tail_collision(food_x, food_y) {
            self.food.reposition(self.width, self.height);
        }
        self.food_exist = true;
    }

    fn check_eating(&mut self, direction: Option<Direction>) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        let (food_x, food_y): (i32, i32) = self.food.position();
        if self.food_exist && food_x == head_x && food_y == head_y {
            self.food_exist = false;
            self.snake.add_tail(direction);
            self.score = self.score + 1;
            if self.snake_speed > 0.04 {
                self.snake_speed = self.snake_speed - 0.01;
            }
        }
    }

    fn restart(&mut self) {
        self.snake = Snake::new(1, 1);
        self.waiting_time =  0.0;
        self.is_game_over = false;
        self.food_exist = true;
        self.is_game_over = false;
        self.score = 0;
        self.snake_speed = 0.1;
        self.food = Food::new(10, 20);
    }
}