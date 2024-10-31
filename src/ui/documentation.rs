//! Окна документации

use cursive::traits::Resizable;
use cursive::traits::Scrollable;
use cursive::Cursive;

use cursive::views::Dialog;
use cursive::views::Panel;
use cursive::views::TextView;

use crate::config::Config;
use crate::consts::CONFIG_FILE;
use crate::consts::RESOURCES_DIR;
use crate::ui::service::error_window;

use std::fs;
use std::path::Path;

pub fn doc_window(scr: &mut Cursive, doc: &str) {
    let conf = Config::read(CONFIG_FILE).unwrap_or_default();

    let doc_pth = Path::new(RESOURCES_DIR).join(&doc);
    let doc_contents = fs::read_to_string(&doc_pth);

    match doc_contents {
        Ok(doc_txt) => {
            let text = Panel::new(
                TextView::new(format!(
                    "{}\n\n{}",
                    if conf.use_tabs {
                        doc_txt
                    } else {
                        // let doc_txt = &doc_txt;
                        doc_txt.replace('\t', "    ")
                    },
                    "-- конец файла --"
                ))
                .scrollable(),
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
