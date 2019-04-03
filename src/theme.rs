use piston_window::types::Color;

const inverse_255: f32 = 1.0f32 / 255.0f32;

pub const TEXT_COLOR: Color = [51.0 * inverse_255, 101.0 * inverse_255, 68.0 * inverse_255, 1.0];
pub const BACKGROUND_COLOR: Color = [17.0 * inverse_255, 57.0 * inverse_255, 48.0 * inverse_255, 1.0];
pub const SNAKE_COLOR: Color = [65.0 * inverse_255, 130.0 * inverse_255, 88.0 * inverse_255, 1.0];
pub const FOOD_COLOR: Color = [71.0 * inverse_255, 143.0 * inverse_255, 94.0 * inverse_255, 1.0];
pub const BORDER_COLOR: Color = [35.0 * inverse_255, 82.0 * inverse_255, 57.0 * inverse_255, 1.0];