//! Модуль для формирования главного меню программы

use cursive::traits::Scrollable;
use cursive::Cursive;

use cursive::views::Dialog;
use cursive::views::LinearLayout;
use cursive::views::Panel;
use cursive::views::SelectView;
use cursive::views::TextView;

use cursive::event::Key;
use cursive::menu;

use crate::consts::DOCUMENTATION_INFO_FILE;
use crate::documentation::Documentation;
use crate::ui::documentation::doc_window;
use crate::ui::search::search_window;
use crate::ui::service::about_window;
use crate::ui::service::is_exit_window;
use crate::ui::service::projects_window;
use crate::ui::service::unimplemented_window;

use super::service::error_window;

pub fn menubar(scr: &mut Cursive) {
    scr.menubar()
        .add_subtree("Файл", menu::Tree::new().leaf("Выход", is_exit_window))
        .add_subtree(
            "Справка",
            menu::Tree::new()
                .leaf("Мои проекты", projects_window)
                .leaf("О программе", about_window),
        );

    scr.set_autohide_menu(false);

    scr.add_global_callback(Key::F1, |s| s.select_menubar());
    scr.add_global_callback('s', search_window);
    scr.add_global_callback('S', search_window);
    scr.add_global_callback(Key::F10, is_exit_window);
}

pub fn main_menu_window(scr: &mut Cursive, doc: &Documentation) {
    let mut menu = SelectView::new().autojump();

    for d in &doc.categories {
        menu.add_item(format!("{:<width$} -->", &d.name, width = 33), d.id.clone());
    }

    menu.add_item("Поиск по документации", "_search".to_string());
    menu.add_item("Другие мои проекты", "_projects".to_string());
    menu.add_item("О программе", "_about".to_string());

    menu.set_on_submit(show_category_contents_window);

    let text = TextView::new(
        "Стрелками на клавиатуре выберите нужную\n\
                  категорию, затем нажмите <Enter>. Для\n\
                  активации верхнего меню нажмите <F1>.",
    );
    let layout = LinearLayout::vertical()
        .child(text)
        .child(Panel::new(menu.scrollable()));

    let win = Dialog::around(layout)
        .title("Главное меню")
        .button("Выход", is_exit_window);
    scr.add_layer(win);
}

pub fn show_category_contents_window(scr: &mut Cursive, category: &str) {
    if category.starts_with('_') {
        match category {
            "_about" => about_window(scr),
            "_projects" => projects_window(scr),
            "_search" => search_window(scr),
            _ => unimplemented_window(scr),
        }
    } else {
        let doc = Documentation::read(DOCUMENTATION_INFO_FILE).unwrap_or_default();
        match doc.docs.get(category) {
            Some(contents) => {
                let mut menu = SelectView::new().autojump();
                for d in contents {
                    menu.add_item(&d.name, format!("{}/{}", category, &d.id));
                }
                menu.set_on_submit(doc_window);

                let win = Dialog::around(Panel::new(menu.scrollable()))
                    .title(format!("Просмотр категории «{}»", category))
                    .button("Назад", |s| {
                        s.pop_layer();
                    });
                scr.add_layer(win);
            }
            None => error_window(
                scr,
                format!("Запрашиваемая категория отсутствует или не определена"),
            ),
        }
    }
}
