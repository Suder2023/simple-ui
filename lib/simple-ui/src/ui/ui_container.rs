use crate::{
    ui::{
        styles::{EUIStatus, StyleInner},
        ui_ctrl::{BaseCtrl, CtrlStyle, TCtrlInner},
        ECtrlType, TCtrl, TLayout,
    },
    utils::{in_rc, ScaleDpi},
    IPoint, IRect, ISize,
};
use as_any::Downcast;
use std::rc::Rc;

pub struct Container {
    pub ctrl: BaseCtrl<CtrlStyle>,

    layout: Rc<Box<dyn TLayout>>,
    children: Vec<Box<dyn TCtrl>>,
}

impl Container {
    pub fn new(layout: Box<dyn TLayout>) -> Self {
        Self {
            ctrl: BaseCtrl::default(),
            layout: Rc::new(layout),
            children: Vec::new(),
        }
    }

    pub fn update_self(&mut self, window_size: ISize, dpi: &ScaleDpi) {
        self.ctrl.inner.real_rc = match self.ctrl.inner.margin {
            Some(margin) => {
                let margin = dpi.scale(margin);
                IRect::new(
                    margin.left,
                    margin.top,
                    window_size.width - margin.right,
                    window_size.height - margin.bottom,
                )
            }
            None => IRect::new(0, 0, window_size.width, window_size.height),
        };
    }

    pub fn get_children(&self) -> &Vec<Box<dyn TCtrl>> {
        &self.children
    }
    pub fn get_mut_children(&mut self) -> &mut Vec<Box<dyn TCtrl>> {
        &mut self.children
    }
    pub fn append_child(&mut self, c: Box<dyn TCtrl>) {
        self.children.push(c);
    }

    pub fn get_ctrl_by_name(&self, name: &str) -> Option<&dyn TCtrl> {
        for child in self.children.iter().rev() {
            let child = child.as_ref();
            if child.get_inner().name == name {
                return Some(child);
            }

            if child.is_container() {
                let r = child.downcast_ref::<Self>().unwrap();
                let r = r.get_ctrl_by_name(name);
                if r.is_some() {
                    return r;
                }
            }
        }
        None
    }

    pub fn get_mut_ctrl_by_name(&mut self, name: &str) -> Option<&mut dyn TCtrl> {
        for child in &mut self.children {
            let child = child.as_mut();

            if child.get_inner().name == name {
                return Some(child);
            }

            if child.is_container() {
                let r = child.downcast_mut::<Self>().unwrap();
                let r = r.get_mut_ctrl_by_name(name);
                if r.is_some() {
                    return r;
                }
            }
        }
        None
    }

    pub fn get_ctrl_by_pos(&self, point: &IPoint) -> &dyn TCtrl {
        for child in self.children.iter().rev() {
            if in_rc(&child.get_inner().real_rc, &point) {
                let child = child.as_ref();
                return if child.is_container() {
                    let s = child.downcast_ref::<Self>().unwrap();
                    s.get_ctrl_by_pos(point)
                } else {
                    child
                };
            }
        }
        self
    }
}

impl TCtrl for Container {
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
        "Container"
    }
    fn type_(&self) -> ECtrlType {
        ECtrlType::Container
    }

    fn update(&mut self, dpi: &ScaleDpi) {
        self.layout.clone().update(self, dpi);

        let children: &mut Vec<Box<dyn TCtrl>> = &mut self.children;
        for child in children {
            child.update(dpi);
        }
    }
    fn render(&self, canvas: &mut skia_safe::Canvas, dpi: &ScaleDpi) {
        if self.ctrl.inner.empty_paint() {
            return;
        }
        self.ctrl
            .inner
            .render(canvas, dpi, self.ctrl.get_cur_style());

        for child in &self.children {
            child.render(canvas, dpi);
        }
    }

    fn update_dpi(&mut self, dpi: &ScaleDpi) {
        for child in &mut self.children {
            let child = child.as_mut();
            child.update_dpi(dpi);

            if child.is_container() {
                child.downcast_mut::<Self>().unwrap().update_dpi(dpi);
            }
        }
    }
}
