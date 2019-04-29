use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 10.0;

pub fn to_coord(game_coord: u32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn draw_block(color: Color, x: u32, y: u32, con: &Context, g: &mut G2d) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [x, y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    )
}

pub fn draw_rectangle(color: Color, x: u32, y: u32, width: u32, height: u32, con: &Context, g: &mut G2d) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [x, y, BLOCK_SIZE * (width as f64), BLOCK_SIZE * (height as f64)],
        con.transform,
        g,
    )
}
