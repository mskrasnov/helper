use crate::consts::DOCUMENTATION_INFO_FILE;

pub mod documentation;
pub mod menu;
pub mod search;
pub mod service;

pub fn ui() {
    let mut scr = cursive::default();

    let doc =
        crate::documentation::Documentation::read(DOCUMENTATION_INFO_FILE).unwrap_or_default();

    menu::menubar(&mut scr);
    menu::main_menu_window(&mut scr, &doc);

    scr.run();
}
