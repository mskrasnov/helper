pub mod consts;
pub mod documentation;
// pub mod markdown; // NOTE: это экспериментальный парсер Markdown. Он будет использован в будущем.
pub mod ui;

use ui::ui;

fn main() {
    ui();
}
