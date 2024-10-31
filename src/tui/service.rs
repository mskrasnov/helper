//! Сервисные окна

use cursive::align::HAlign;
use cursive::traits::Resizable;
use cursive::traits::Scrollable;
use cursive::Cursive;

use cursive::views::Dialog;
use cursive::views::LinearLayout;
use cursive::views::Panel;
use cursive::views::TextView;

pub fn about_window(scr: &mut Cursive) {
    let about_program_text = TextView::new(format!(
        "Helper ver.{}. Программа для получения справки по использованию \
         Linux-систем.",
        env!("CARGO_PKG_VERSION")
    ));
    let about_author_text = TextView::new("(C) 2024 Михаил Краснов <michail383krasnov@mail.ru>");
    let links = TextView::new(
        "Репозиторий программы находится по адресу:\nhttps://github.com/mskrasnov/helper",
    );

    let data = LinearLayout::vertical()
        .child(
            Panel::new(about_program_text)
                .title("О программе")
                .title_position(HAlign::Left),
        )
        .child(
            Panel::new(about_author_text)
                .title("Об авторе")
                .title_position(HAlign::Left),
        )
        .child(
            Panel::new(links)
                .title("Ссылки")
                .title_position(HAlign::Left),
        );

    let win = Dialog::around(data)
        .title("О программе Helper")
        .button("OK", |s| {
            s.pop_layer();
        });
    scr.add_layer(win);
}

pub fn projects_window(scr: &mut Cursive) {
    let text = TextView::new("Кроме Helper, у меня есть ещё некоторые проекты, которые, возможно, могут вас заинтересовать:");
    let projects = LinearLayout::vertical()
        .child(TextView::new(" * Linux for ARM (LFA) [https://linux-for-arm.github.io] - руководство по сборке своего дистрибутива Linux для ARM-компьютеров с нуля из исходного кода"))
        .child(TextView::new(" * resistor [https://github.com/mskrasnov/resistor] - программа для рассчёта сопротивления 4-полосных резисторов по их цветовой маркировке."));

    let win = Dialog::around(
        LinearLayout::vertical()
            .child(text)
            .child(Panel::new(projects.scrollable())),
    )
    .title("Проекты автора")
    .button("ОК", |s| {
        s.pop_layer();
    })
    .max_width(80);
    scr.add_layer(win);
}

pub fn is_exit_window(scr: &mut Cursive) {
    let text = TextView::new("Действительно выйти?");
    let win = Dialog::around(text)
        .title("Подтверждение выхода")
        .button("Нет", |s| {
            s.pop_layer();
        })
        .button("Да", |s| s.quit());
    scr.add_layer(win);
}

pub fn error_window<E: Into<String>>(scr: &mut Cursive, error_text: E) {
    let text = TextView::new(error_text);
    let win = Dialog::around(text).title("Ошибка").button("OK", |s| {
        s.pop_layer();
    });
    scr.add_layer(win);
}

pub fn unimplemented_window(scr: &mut Cursive) {
    let text = TextView::new(
        "Данная функция пока не реализована. Если вы видите это\n\
                  сообщение, мы уже занимаемся реализацией этой вещи. В\n\
                  следующих релизах она уже будет!",
    );
    let win = Dialog::around(text)
        .title("Оставь свои шаловливые ручонки при себе")
        .button("OK", |s| {
            s.pop_layer();
        });
    scr.add_layer(win);
}
