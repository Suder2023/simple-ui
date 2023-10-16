use crate::ui::{
    styles::{EUIStatus, StyleInner, TStyle},
    ui_ctrl::{BaseCtrl, CtrlStyle, TCtrlInner},
    ECtrlType, TCtrl,
};

pub trait TButtonDelegate {
    fn on_click(&self);
}

#[derive(Default)]
pub struct ButtonStyle {
    base: CtrlStyle,
}

impl TStyle for ButtonStyle {
    fn get_inner(&self) -> &StyleInner {
        &self.base
    }
    fn get_mut_inner(&mut self) -> &mut StyleInner {
        &mut self.base
    }
}

#[derive(Default)]
pub struct Button {
    pub ctrl: BaseCtrl<ButtonStyle>,
    delegate: Option<Box<dyn TButtonDelegate>>,
}

impl Button {
    pub fn set_delegate(&mut self, delegate: Box<dyn TButtonDelegate>) -> &mut Self {
        self.delegate = Some(delegate);
        self
    }
}

impl TCtrl for Button {
    fn get_inner(&self) -> &TCtrlInner {
        &self.ctrl.inner
    }
    fn get_mut_inner(&mut self) -> &mut TCtrlInner {
        &mut self.ctrl.inner
    }
    fn get_style(&self, status: EUIStatus) -> Option<&StyleInner> {
        self.ctrl.styles.get_inner_style(status)
    }
    fn get_mut_style(&mut self, status: EUIStatus) -> Option<&mut StyleInner> {
        self.ctrl.styles.get_mut_inner_style(status)
    }

    fn type_name(&self) -> &str {
        "Button"
    }
    fn type_(&self) -> ECtrlType {
        ECtrlType::Button
    }
    fn render(&self, canvas: &mut skia_safe::Canvas, dpi: &crate::utils::ScaleDpi) {
        self.ctrl
            .inner
            .render(canvas, dpi, &self.ctrl.get_cur_style().base);
    }
}
