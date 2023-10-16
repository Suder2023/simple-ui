use crate::{
    ui::{
        styles::{EUIStatus, StyleInner, TStyle},
        ui_ctrl::{BaseCtrl, CtrlStyle, TCtrlInner},
        ECtrlType, TCtrl,
    },
    utils::ScaleDpi,
    Color,
};
use skia_safe::{paint::Paint, Font, Point, Typeface};

pub struct TextStyle {
    base: CtrlStyle,

    font_color: Color,
}

impl TStyle for TextStyle {
    fn get_inner(&self) -> &StyleInner {
        &self.base
    }
    fn get_mut_inner(&mut self) -> &mut StyleInner {
        &mut self.base
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            base: CtrlStyle::default(),
            font_color: Color::BLACK,
        }
    }
}

pub struct Text {
    pub ctrl: BaseCtrl<TextStyle>,

    text: String,
    font_size: i32,

    font: Font,
    paint: Paint,

    dpi_cache: ScaleDpi,
    offset_y: f32,
}

impl Text {
    pub fn new(d: &str, font_size: i32) -> Self {
        Self {
            ctrl: BaseCtrl::default(),

            text: d.to_string(),
            font_size,

            font: Font::default(),
            paint: Paint::default(),

            dpi_cache: ScaleDpi::default(),
            offset_y: 0.,
        }
    }

    pub fn set_text(&mut self, t: String) {
        self.text = t;
        self.update_rc()
    }

    fn update_ctx(&mut self) {
        self.font
            .set_size(self.dpi_cache.scale(self.font_size) as f32);
        let fm = Typeface::new("PingFang SC", skia_safe::FontStyle::default());
        self.font.set_typeface(fm.unwrap());

        self.paint.set_color(self.ctrl.get_cur_style().font_color);
    }

    fn update_rc(&mut self) {
        self.update_ctx();

        let (_s, rc) = self.font.measure_str(&self.text, Some(&self.paint));

        self.offset_y = rc.bottom;
    }
}

impl TCtrl for Text {
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
        "Text"
    }
    fn type_(&self) -> ECtrlType {
        ECtrlType::Text
    }

    fn render(&self, canvas: &mut skia_safe::Canvas, dpi: &ScaleDpi) {
        assert_eq!(dpi, &self.dpi_cache);

        self.ctrl
            .inner
            .render(canvas, dpi, &self.ctrl.get_cur_style().base);
        let real_rc = self.ctrl.inner.real_rc;

        canvas.draw_str(
            &self.text,
            Point::new(real_rc.left as f32, real_rc.bottom as f32 - self.offset_y),
            &self.font,
            &self.paint,
        );
    }

    fn update_dpi(&mut self, dpi: &ScaleDpi) {
        self.dpi_cache = dpi.clone();
        self.update_rc();
    }
}
