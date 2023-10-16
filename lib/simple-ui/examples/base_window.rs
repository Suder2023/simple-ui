use as_any::Downcast;
use simple_ui::{
    ui::*, window::TWindowDelegate, Color, IPoint, IRect, ISize, Window, WindowBuilder,
};
use skia_safe::{Canvas, Paint, Rect};

struct WndDelegate {
    _f_rotation_angle: f32,

    mouse_pos: IPoint,
    paint: Paint,
}

impl WndDelegate {
    pub fn new() -> Self {
        let mut paint = Paint::default();
        paint.set_style(skia_safe::paint::Style::Stroke);
        paint.set_color(Color::RED);
        paint.set_stroke_width(2.);

        Self {
            _f_rotation_angle: 0.,
            mouse_pos: IPoint::default(),
            paint,
        }
    }
}

fn get_text_ctrl(ctrl: Option<&mut dyn TCtrl>) -> Option<&mut Text> {
    if ctrl.is_none() {
        return None;
    }
    let ctrl = ctrl.unwrap();
    if ctrl.type_() != ECtrlType::Text {
        return None;
    }

    ctrl.downcast_mut::<Text>()
}

impl TWindowDelegate for WndDelegate {
    fn on_draw(&mut self, window: &Window, canvas: &mut Canvas) {
        // hover 到某个控件,则会给他绘制一个边框

        let cur_ctrl = window.get_ctrl_by_pos(&self.mouse_pos).unwrap();
        let cur_rect: Rect = cur_ctrl.get_real_rc().clone().into();

        // mouse_pos
        canvas.draw_rect(cur_rect, &self.paint);
    }
    fn on_mouse_moved(&mut self, window: &mut Window, pos: IPoint) {
        self.mouse_pos = pos;
        let hover_ctrl_name = window
            .get_ctrl_by_pos(&pos)
            .unwrap()
            .get_inner()
            .name
            .as_str()
            .to_string();

        match get_text_ctrl(window.get_mut_ctrl_by_name("Status_CurCtrl")) {
            Some(ctrl) => ctrl.set_text(format!("{}", hover_ctrl_name)),
            None => (),
        };

        match get_text_ctrl(window.get_mut_ctrl_by_name("Status_MousePos")) {
            Some(ctrl) => ctrl.set_text(format!("{:4?}, {:4?}", pos.x, pos.y)),
            None => (),
        };

        window.redraw();
    }
}

struct ButtonCallback1 {}
impl ui_button::TButtonDelegate for ButtonCallback1 {
    fn on_click(&self) {}
}

fn create_root() -> Container {
    let mut ui_root = Container::new(LayoutVertical::new());
    ui_root.ctrl.inner.name = "Root".to_string();
    ui_root.ctrl.inner.margin = Some(IRect::new(4, 4, 4, 4));
    ui_root.ctrl.inner.padding = Some(IRect::new(6, 6, 6, 6));
    ui_root.ctrl.inner.border_width = 2;
    ui_root.ctrl.inner.round = 14;
    ui_root.ctrl.styles.default.border_color = Some(Color::DARK_GRAY);
    ui_root.ctrl.styles.default.bg_color = Some(Color::WHITE);

    ui_root
}

fn create_tool_bar() -> Container {
    let mut ui_toolbar = Container::new(LayoutHorizontal::new());
    {
        ui_toolbar.ctrl.inner.name = "UIToobar".to_string();
        ui_toolbar.ctrl.inner.size.height = Some(25);
    }
    {
        let mut ui_text = Text::new("TODO 这里需要放各种工具", 12);
        ui_text.ctrl.inner.name = "Toolbar_TODO".to_string();
        ui_text.ctrl.inner.pos.left = Some(5);
        ui_text.ctrl.inner.pos.top = Some(5);
        ui_text.ctrl.inner.margin = Some(IRect::new(20, 0, 0, 10));

        ui_toolbar.append_child(Box::new(ui_text));
    }

    ui_toolbar
}

fn create_sidebar() -> Container {
    let mut ui_siderbar = Container::new(LayoutVertical::new());
    {
        ui_siderbar.ctrl.inner.name = "UISider".to_string();
        ui_siderbar.ctrl.inner.size.width = Some(100);
    }

    ui_siderbar
}

fn create_ctx() -> Container {
    let mut ui_ctx = Container::new(LayoutHorizontal::new());
    {
        ui_ctx.ctrl.inner.name = "UIContext3".to_string();
    }

    let mut ui_ctx_left = Container::new(LayoutVertical::new());
    {
        ui_ctx_left.ctrl.inner.name = "UICtxLeft".to_string();

        ui_ctx.append_child(Box::new(ui_ctx_left));
    }

    let mut ui_ctx_right = Container::new(LayoutVertical::new());
    {
        ui_ctx_right.ctrl.inner.name = "UICtxRight".to_string();
        ui_ctx.append_child(Box::new(ui_ctx_right));
    }

    ui_ctx
}

fn create_status_bar() -> Container {
    let mut ui_status = Container::new(LayoutHorizontal::new());
    ui_status.ctrl.inner.name = "UIStatus".to_string();
    ui_status.ctrl.inner.size.height = Some(16);

    {
        let mut ui_txt_mouse_pos = Text::new("        ", 12);
        ui_txt_mouse_pos.ctrl.inner.name = "Status_CurCtrl".to_string();
        ui_txt_mouse_pos.ctrl.inner.size.width = Some(160);
        ui_txt_mouse_pos.ctrl.inner.margin = Some(IRect::new(20, 0, 0, 0));

        ui_status.append_child(Box::new(ui_txt_mouse_pos));
    }

    {
        let mut ui_txt_mouse_pos = Text::new("   0,   0", 12);
        ui_txt_mouse_pos.ctrl.inner.name = "Status_MousePos".to_string();
        ui_txt_mouse_pos.ctrl.inner.size.width = Some(80);
        ui_txt_mouse_pos.ctrl.inner.margin = Some(IRect::new(20, 0, 0, 0));

        ui_status.append_child(Box::new(ui_txt_mouse_pos));
    }

    {
        let mut ui_text = Text::new("TODO 这里将会显示控件状态", 12);
        ui_text.ctrl.inner.name = "Status_TODO".to_string();

        ui_status.append_child(Box::new(ui_text));
    }

    ui_status
}

fn create() -> Container {
    let mut ui_root = create_root();

    ui_root.append_child(Box::new(create_tool_bar()));
    let mut ui_context = Container::new(LayoutHorizontal::new());
    {
        ui_context.ctrl.inner.name = "UIContextRoot".to_string();
        ui_context.append_child(Box::new(create_sidebar()));

        let mut ui_context2 = Container::new(LayoutVertical::new());
        ui_context2.ctrl.inner.name = "UIContext2".to_string();
        ui_context2.append_child(Box::new(create_ctx()));

        // 输出框
        {
            // TODO 可以换行的, skia默认不支持,需要自己处理
            let mut ui_text_info = Text::new("", 12);
            ui_text_info.ctrl.inner.name = "TextInfo".to_string();
            ui_text_info.ctrl.inner.size.height = Some(150);

            ui_context2.append_child(Box::new(ui_text_info));
        }

        ui_context.append_child(Box::new(ui_context2));
    }
    ui_root.append_child(Box::new(ui_context));
    ui_root.append_child(Box::new(create_status_bar()));

    simple_ui::utils_dbg::fill_bg_color(&mut ui_root);

    ui_root
}

// fn load_by_file() -> Container {
//     loader::loader(&"examples/base_windows_ui.json".into())
//         .ok()
//         .unwrap()
// }

fn main() {
    let mut window = WindowBuilder::new("Base window", ISize::new(800, 600));

    // let _root = load_by_file();
    let root = create();

    window.set_root_container(root);
    window.set_delegate(Box::new(WndDelegate::new()));
    window.run();
}
