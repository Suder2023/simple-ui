use crate::{
    ui::{
        styles::{EUIStatus, StyleInner, Styles, TStyle},
        ECtrlStatus, ECtrlType,
    },
    utils::{IRectOpt, ISizeOpt, ScaleDpi},
    IRect,
};
use as_any::AsAny;
use skia_safe::{Canvas, Paint, Rect};

pub trait TCtrl: AsAny {
    fn get_inner(&self) -> &TCtrlInner;
    fn get_mut_inner(&mut self) -> &mut TCtrlInner;
    fn get_style(&self, status: EUIStatus) -> Option<&StyleInner>;
    fn get_mut_style(&mut self, status: EUIStatus) -> Option<&mut StyleInner>;

    fn type_name(&self) -> &str;
    fn type_(&self) -> ECtrlType;
    fn is_container(&self) -> bool {
        self.type_() == ECtrlType::Container
    }

    fn init(&mut self) -> Result<(), ()> {
        Ok(())
    }
    fn update(&mut self, _dpi: &ScaleDpi) {}
    fn render(&self, canvas: &mut Canvas, dpi: &ScaleDpi);
    fn update_dpi(&mut self, _dpi: &ScaleDpi) {}

    fn get_real_rc(&self) -> &IRect {
        &self.get_inner().real_rc
    }

    fn set_name(&mut self, name: &str) -> &mut dyn TCtrl
    where
        Self: Sized,
    {
        self.get_mut_inner().name = name.to_string();
        self
    }
}

#[derive(Default)]
pub struct TCtrlInner {
    pub name: String,

    pub border_width: i32,
    pub round: i32,

    pub(crate) real_rc: IRect,
    pub content_rc: IRect,
    pub size: ISizeOpt,
    pub pos: IRectOpt,

    pub padding: Option<IRect>,
    pub margin: Option<IRect>,
}

impl TCtrlInner {
    pub fn render(&self, canvas: &mut Canvas, dpi: &ScaleDpi, sytle: &CtrlStyle) {
        if self.empty_paint() {
            return;
        }
        let real_rc: Rect = self.real_rc.clone().into();
        let round = self.round as f32;

        let mut paint = Paint::default();

        if sytle.bg_color.is_some() {
            paint.set_color(sytle.bg_color.unwrap());
            paint.set_style(skia_safe::paint::Style::Fill);

            if round == 0. {
                paint.set_anti_alias(false);
                canvas.draw_rect(real_rc, &paint);
            } else {
                paint.set_anti_alias(true);
                canvas.draw_round_rect(real_rc, round, round, &paint);
            }
        }

        if self.border_width != 0 && sytle.border_color.is_some() {
            paint.set_style(skia_safe::paint::Style::Stroke);
            paint.set_color(sytle.border_color.unwrap());
            paint.set_stroke_width(dpi.scale(self.border_width) as f32);

            if round == 0. {
                paint.set_anti_alias(false);
                canvas.draw_rect(real_rc, &paint);
            } else {
                paint.set_anti_alias(true);
                canvas.draw_round_rect(real_rc, round, round, &paint);
            }
        }
    }

    pub fn empty_paint(&self) -> bool {
        let real_rc = self.real_rc;
        if real_rc.width() <= 0 || real_rc.height() <= 0 {
            return true;
        } else {
            return false;
        }
    }
}

pub type CtrlStyle = StyleInner;

impl TStyle for CtrlStyle {
    fn get_inner(&self) -> &StyleInner {
        self
    }
    fn get_mut_inner(&mut self) -> &mut StyleInner {
        self
    }
}

#[derive(Default)]
pub struct BaseCtrl<T: Default + TStyle> {
    pub inner: TCtrlInner,
    pub styles: Styles<T>,

    pub status: ECtrlStatus,
}

impl<T: Default + TStyle> BaseCtrl<T> {
    pub fn get_cur_style(&self) -> &T {
        match self.status {
            ECtrlStatus::Default => &self.styles.default,
            ECtrlStatus::Hover => {
                if self.styles.hover.is_none() {
                    &self.styles.default
                } else {
                    self.styles.hover.as_ref().unwrap()
                }
            }
            ECtrlStatus::Press => {
                if self.styles.press.is_none() {
                    &self.styles.default
                } else {
                    self.styles.press.as_ref().unwrap()
                }
            }
            ECtrlStatus::Disable => {
                if self.styles.disable.is_none() {
                    &self.styles.default
                } else {
                    self.styles.disable.as_ref().unwrap()
                }
            }
        }
    }
}

pub type Ctrl = BaseCtrl<CtrlStyle>;
impl TCtrl for Ctrl {
    fn get_inner(&self) -> &TCtrlInner {
        &self.inner
    }
    fn get_mut_inner(&mut self) -> &mut TCtrlInner {
        &mut self.inner
    }
    fn get_style(&self, status: EUIStatus) -> Option<&StyleInner> {
        self.styles.get_inner_style(status)
    }
    fn get_mut_style(&mut self, status: EUIStatus) -> Option<&mut StyleInner> {
        self.styles.get_mut_inner_style(status)
    }
    fn type_name(&self) -> &str {
        "BaseCtrl"
    }
    fn type_(&self) -> ECtrlType {
        ECtrlType::BaseCtrl
    }

    fn render(&self, canvas: &mut Canvas, dpi: &ScaleDpi) {
        self.get_inner().render(canvas, dpi, self.get_cur_style());
    }
}
