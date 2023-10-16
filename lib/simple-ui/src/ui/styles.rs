use crate::Color;

pub enum EUIStatus {
    Default,
    Hover,
    Press,
    Disable,
}
impl Default for EUIStatus {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Default)]
pub struct StyleInner {
    pub bg_color: Option<Color>,
    pub border_color: Option<Color>,
}

pub trait TStyle {
    fn get_inner(&self) -> &StyleInner;
    fn get_mut_inner(&mut self) -> &mut StyleInner;
}

#[derive(Default)]
pub struct Styles<T: Default + TStyle> {
    pub default: T,
    pub hover: Option<T>,
    pub press: Option<T>,
    pub disable: Option<T>,
}

impl<T: Default + TStyle> Styles<T> {
    pub fn get_inner_style(&self, status: EUIStatus) -> Option<&StyleInner> {
        match status {
            EUIStatus::Default => Some(self.default.get_inner()),
            EUIStatus::Hover => self.hover.as_ref().map(|f| f.get_inner()),
            EUIStatus::Press => self.press.as_ref().map(|f| f.get_inner()),
            EUIStatus::Disable => self.disable.as_ref().map(|f| f.get_inner()),
        }
    }

    pub fn get_mut_inner_style(&mut self, status: EUIStatus) -> Option<&mut StyleInner> {
        match status {
            EUIStatus::Default => Some(self.default.get_mut_inner()),
            EUIStatus::Hover => self.hover.as_mut().map(|f| f.get_mut_inner()),
            EUIStatus::Press => self.press.as_mut().map(|f| f.get_mut_inner()),
            EUIStatus::Disable => self.disable.as_mut().map(|f| f.get_mut_inner()),
        }
    }
}
