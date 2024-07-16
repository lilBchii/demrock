mod button;
mod constants;
mod credits;
mod main_menu;
mod options;
mod style;

pub use credits::credits;
pub use main_menu::main_menu;
pub use options::options;
pub use style::GuiResources;

pub const BUTTON_SIZE: (f32, f32) = (600.0, 100.0);
pub const TITLE_FONT_SIZE: u16 = 45;
