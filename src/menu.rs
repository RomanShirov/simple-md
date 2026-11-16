#[cfg(not(target_os = "macos"))]
use gpui_kit::base::GlobalState;
use gpui_kit::{App, Menu, MenuItem};

gpui_kit::actions!(simple_md, [Open, Save]);

pub fn init(cx: &mut App) {
    cx.on_action(|_: &Open, _| {});
    cx.on_action(|_: &Save, _| {});

    let file = Menu::new("File").items([
        MenuItem::action("Open", Open),
        MenuItem::action("Save", Save),
    ]);

    #[cfg(target_os = "macos")]
    cx.set_menus([Menu::new("Simple MD"), file]);

    #[cfg(not(target_os = "macos"))]
    GlobalState::global_mut(cx).set_app_menus(vec![file.owned()]);
}
