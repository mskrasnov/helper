//! Поиск по документации

use cursive::traits::Nameable;
use cursive::traits::Scrollable;
use cursive::Cursive;

use cursive::views::Dialog;
use cursive::views::EditView;
use cursive::views::LinearLayout;
use cursive::views::Panel;
use cursive::views::SelectView;
use cursive::views::TextView;

use crate::consts::DOCUMENTATION_INFO_FILE;
use crate::consts::RESOURCES_DIR;
use crate::documentation::Documentation;
use crate::ui::documentation::doc_window;

use anyhow::Result;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn search_in_file<P: AsRef<Path>>(pth: P, pattern: &str) -> Result<bool> {
    let contents = fs::read_to_string(&pth)?
        // Для того, чтобы поиск был регистронезависимым, нам нужно привести
        // строку запроса и строку, в которой указанный запрос ищем, к одному
        // регистру символов.
        .to_lowercase();
    Ok(contents.contains(&pattern.to_lowercase()))
}

/// Последовательно читает указанные в `documents.toml` файлы и ищет в них
/// подстроку `pattern`. Поиск регистронезависимый.
fn search(doc: &Documentation, pattern: &str) -> Vec<PathBuf> {
    let mut pathes = Vec::new();
    for d in &doc.docs {
        let path = Path::new(RESOURCES_DIR).join(&d.0);
        for page in d.1 {
            let pth = path.join(&page.id);
            match search_in_file(&pth, pattern) {
                Ok(result) => {
                    if result {
                        pathes.push(pth);
                    }
                }
                // В случае "проброшенной" на уровень выше ошибки мы получим
                // пустой вектор. Чтобы этого избежать, просто игнорим все
                // файлы, чтение которых завершилось ошибкой.
                Err(_) => {}
            }
        }
    }
    pathes
}

fn search_fn(scr: &mut Cursive, pattern: &str) {
    if pattern.is_empty() {
        let win = Dialog::around(TextView::new("Вы должны ввести поисковой запрос!"))
            .title("Введите запрос")
            .button("OK", |s| {
                s.pop_layer();
            });
        scr.add_layer(win);
    } else {
        let doc = Documentation::read(DOCUMENTATION_INFO_FILE).unwrap_or_default();
        let pathes = search(&doc, pattern);

        if !pathes.is_empty() {
            search_results(scr, pattern, &pathes)
        } else {
            search_unsuccess(scr, pattern);
        }
    }
}

fn search_results(scr: &mut Cursive, pattern: &str, result: &Vec<PathBuf>) {
    let text = TextView::new(format!("Текст «{pattern}» найден в следующих документах:"));

    let mut results = SelectView::new();
    for doc in result {
        // Функция doc_window сама подставит значение RESOURCES_DIR в путь, но
        // поскольку функция поиска тоже подставляет это значение в путь, нам
        // нужно от RESOURCES_DIR в значении пункта меню избавиться.
        results.add_item_str(format!("{}", doc.display()).replace(RESOURCES_DIR, ""));
    }
    results.set_on_submit(doc_window);

    let layout = LinearLayout::vertical()
        .child(text)
        .child(Panel::new(results.scrollable()));

    let win = Dialog::around(layout)
        .title("Результаты поиска")
        .button("OK", |s| {
            s.pop_layer();
        });
    scr.add_layer(win);
}

fn search_unsuccess(scr: &mut Cursive, pattern: &str) {
    let text = TextView::new(format!(
        "По запросу «{pattern}» ничего не найдено! Попробуйте изменить запрос."
    ));
    let win = Dialog::around(text)
        .title("Результаты поиска")
        .button("OK", |s| {
            s.pop_layer();
        });
    scr.add_layer(win);
}

pub fn search_window(scr: &mut Cursive) {
    let input = EditView::new().on_submit(search_fn).with_name("search_doc");
    let text = TextView::new(
        "Введите поисковой запрос в поле ниже\n\
                  и нажмите клавишу <Enter>:",
    );

    let layout = LinearLayout::vertical().child(text).child(input);

    let win = Dialog::around(layout)
        .title("Поиск по документации")
        .button("Отмена", |s| {
            s.pop_layer();
        });
    scr.add_layer(win);
}
