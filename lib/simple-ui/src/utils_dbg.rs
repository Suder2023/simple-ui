use crate::ui::{styles::EUIStatus, Container, TCtrl};
use as_any::Downcast;
use skia_safe::Color;

const COLOR_LIST: [Color; 10] = [
    Color::DARK_GRAY,
    Color::GRAY,
    Color::LIGHT_GRAY,
    Color::WHITE,
    Color::RED,
    Color::GREEN,
    Color::BLUE,
    Color::YELLOW,
    Color::CYAN,
    Color::MAGENTA,
];

static mut COLOR_COUNT: usize = 0usize;

fn get_color() -> Color {
    unsafe {
        let c = COLOR_LIST[COLOR_COUNT];
        COLOR_COUNT += 1;

        if COLOR_COUNT > COLOR_LIST.len() - 1 {
            COLOR_COUNT = 0;
        }

        c
    }
}

fn update_color(ctrl: &mut dyn TCtrl) {
    let cc = ctrl.get_mut_style(EUIStatus::Default).unwrap();
    if cc.bg_color.is_none() {
        cc.bg_color = Some(get_color());
    }
}

pub fn fill_bg_color(root: &mut Container) {
    // if root.

    update_color(root);
    for child in root.get_mut_children() {
        update_color(child.as_mut());
    }

    for child in root.get_mut_children().iter_mut().rev() {
        let child = child.as_mut();
        if child.is_container() {
            let cc = child.downcast_mut::<Container>().unwrap();
            fill_bg_color(cc);
        }
    }
}
