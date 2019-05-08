extern crate piston_window;
extern crate find_folder;

use piston_window::*;

use crate::draw::{draw_rectangle};
use crate::types::{Direction, State};
use crate::snake::{Snake};
use crate::food::{Food};
use crate::theme;
use crate::ui::{UI};

pub struct Game {
    snake: Snake,
    width: u32,
    height: u32,
    ui: UI,
    food: Food,
    state: State,
}

impl Game {
    pub fn new(viewport: (u32, u32)) -> Game {
        let (width, height) = viewport;
        Game{
            snake: Snake::new(1, 1),
            width,
            height,
            ui: UI::new(viewport),
            state: State::new(),
            food: Food::new(10, 20),
        }
    }

    pub fn draw(&self, context: &Context, g: &mut G2d, glyphs: &mut piston_window::glyph_cache::rusttype::GlyphCache<GfxFactory, G2dTexture>) {

        if self.state.is_pause() && !self.state.is_game_over() {
            self.ui.render_pause(context, g, glyphs);
        }
        
        if !self.state.is_game_over() {
            self.snake.draw(context, g);
        }

        if self.state.is_food_exists() {
            self.food.draw(context, g);
        }
        draw_rectangle(theme::BORDER_COLOR, 0, 0, self.width, 1, context, g);
        draw_rectangle(theme::BORDER_COLOR, 0, self.height - 1, self.width, 1, context, g);
        draw_rectangle(theme::BORDER_COLOR, 0, 0, 1, self.height, context, g);
        draw_rectangle(theme::BORDER_COLOR, self.width - 1, 0, 1, self.height, context, g);

        self.ui.render_score(self.state.get_score(), context, g, glyphs);
        self.ui.render_title(context, g, glyphs);

        if self.state.is_game_over() {
            self.ui.render_game_over(context, g, glyphs);
        }
    }

    pub fn key_pressed(&mut self, key: Key) {

        if key == Key::P {
            self.state.set_pause(!self.state.is_pause());
        }

        if self.state.is_pause() && !self.state.is_game_over() {
            return;
        }

        if self.state.is_game_over() {
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
        if self.state.is_pause() {
            return;
        }
        self.state.add_waiting_time(delta_time);
        if self.state.get_waiting_time() > self.state.get_snake_speed() {
            self.update_snake(None);
        }

        if !self.state.is_food_exists() {
            self.add_food();
        }
    }

    fn update_snake(&mut self, direction: Option<Direction>) {
        if self.is_snake_alive(direction) {
            self.snake.move_forward(direction);
            self.check_eating();
        } else {
            self.state.set_game_over(true);
        }
        self.state.set_waiting_time(0.0);
    }

    fn is_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.is_tail_collision(next_x, next_y) {
            return false;
        }
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn add_food(&mut self) {

        self.food.reposition(self.width, self.height);
        let (mut food_x, mut food_y): (u32, u32) = self.food.position();
        while self.snake.is_tail_collision(food_x, food_y) {
            self.food.reposition(self.width, self.height);
            let new_food_position = self.food.position();
            food_x = new_food_position.0;
            food_y = new_food_position.1;
        }
        self.state.set_food_exists(true);
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (u32, u32) = self.snake.head_position();
        let (food_x, food_y): (u32, u32) = self.food.position();
        if self.state.is_food_exists() && food_x == head_x && food_y == head_y {
            self.state.set_food_exists(false);
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