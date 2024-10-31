pub mod consts;
pub mod config;
pub mod documentation;
// pub mod markdown; // NOTE: это экспериментальный парсер Markdown. Он будет использован в будущем.
pub mod tui;

use tui::tui;

fn main() {
    tui();
}
