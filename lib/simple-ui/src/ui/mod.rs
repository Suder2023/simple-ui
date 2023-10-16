pub mod layout;
pub mod loader;
pub mod styles;
pub mod ui_button;
pub mod ui_container;
pub mod ui_ctrl;
pub mod ui_text;

pub use layout::TLayout;
pub use ui_ctrl::TCtrl;

pub use ui_button::Button;
pub use ui_container::Container;
pub use ui_ctrl::Ctrl;
pub use ui_text::Text;

pub use layout::{Layout, LayoutHorizontal, LayoutVertical};

#[derive(PartialEq, Eq)]
pub enum ECtrlType {
    Unknow,
    BaseCtrl,
    Container,
    Text,
    Button,
}

pub enum ECtrlStatus {
    Default,
    Hover,
    Press,
    Disable,
}
impl Default for ECtrlStatus {
    fn default() -> Self {
        Self::Default
    }
}
