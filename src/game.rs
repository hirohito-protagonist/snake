extern crate piston_window;
extern crate find_folder;

use piston_window::*;

use crate::draw::{draw_rectangle};
use crate::types::{Direction, State};
use crate::snake::{Snake};
use crate::food::{Food};
use crate::theme;

pub struct Game {
    snake: Snake,
    width: i32,
    height: i32,
    food: Food,
    state: State,
}

impl Game {
    pub fn new(viewport: (u32, u32)) -> Game {
        let (width, height) = viewport;
        Game{
            snake: Snake::new(1, 1),
            width: width as i32,
            height: height as i32,
            state: State::new(),
            food: Food::new(10, 20),
        }
    }

    pub fn draw(&self, context: &Context, g: &mut G2d, glyphs: &mut piston_window::glyph_cache::rusttype::GlyphCache<GfxFactory, G2dTexture>) {

        if !self.state.is_game_over {
            self.snake.draw(context, g);
        }

        if self.state.food_exist {
            self.food.draw(context, g);
        }
        draw_rectangle(theme::BORDER_COLOR, 0, 0, self.width, 1, context, g);
        draw_rectangle(theme::BORDER_COLOR, 0, self.height - 1, self.width, 1, context, g);
        draw_rectangle(theme::BORDER_COLOR, 0, 0, 1, self.height, context, g);
        draw_rectangle(theme::BORDER_COLOR, self.width - 1, 0, 1, self.height, context, g);

        self.render_score(context, g, glyphs);

        if self.state.is_game_over {
            self.render_game_over(context, g, glyphs);
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.state.is_game_over {
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
        self.state.waiting_time += delta_time;
        if self.state.waiting_time > self.state.snake_speed {
            self.update_snake(None);
        }

        if !self.state.food_exist {
            self.add_food();
        }
    }

    fn update_snake(&mut self, direction: Option<Direction>) {
        if self.is_snake_alive(direction) {
            self.snake.move_forward(direction);
            self.check_eating();
        } else {
            self.state.is_game_over = true;
        }
        self.state.waiting_time = 0.0;
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
            &format!("Score: {}", self.state.score),
            glyphs,
            &context.draw_state,
            transform,
            g
        ).unwrap();
    }

    fn add_food(&mut self) {

        self.food.reposition(self.width, self.height);
        let (mut food_x, mut food_y): (i32, i32) = self.food.position();
        while self.snake.is_tail_collision(food_x, food_y) {
            self.food.reposition(self.width, self.height);
            let new_food_position = self.food.position();
            food_x = new_food_position.0;
            food_y = new_food_position.1;
        }
        self.state.is_food_exist(true);
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        let (food_x, food_y): (i32, i32) = self.food.position();
        if self.state.food_exist && food_x == head_x && food_y == head_y {
            self.state.is_food_exist(false);
            self.snake.add_tail();
            self.state.increase_score();
            self.state.speed_up_snake();
        }
    }

    fn restart(&mut self) {
        self.snake = Snake::new(1, 1);
        self.state = State::new();
        self.food = Food::new(10, 20);
    }
}