//! Окна документации

use cursive::traits::Resizable;
use cursive::traits::Scrollable;
use cursive::Cursive;

use cursive::views::Dialog;
use cursive::views::Panel;
use cursive::views::TextView;

use crate::consts::RESOURCES_DIR;
use crate::ui::service::error_window;

use std::fs;
use std::path::Path;

pub fn doc_window(scr: &mut Cursive, doc: &str) {
    let doc_pth = Path::new(RESOURCES_DIR).join(&doc);
    let doc_contents = fs::read_to_string(&doc_pth);

    match doc_contents {
        Ok(doc_txt) => {
            let text = Panel::new(
                TextView::new(format!("{}\n\n{}", &doc_txt, "-- конец файла --")).scrollable(),
            );
            let win = Dialog::around(text)
                .title(doc)
                .button("Закрыть", |s| {
                    s.pop_layer();
                })
                .max_width(80)
                .max_height(24);
            scr.add_layer(win);
        }
        Err(_) => {
            error_window(
                scr,
                "Ошибка чтения запрашиваемого файла! Он\n\
                             точно существует в файловой системе?",
            );
        }
    }
}
