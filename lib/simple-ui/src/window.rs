use cocoa::{appkit::NSView, base::id as cocoa_id};
use core_graphics_types::geometry::CGSize;
use foreign_types_shared::{ForeignType, ForeignTypeRef};
use metal::{Device, MTLPixelFormat, MetalLayer};
use objc::{rc::autoreleasepool, runtime::YES};
use skia_safe::{
    gpu::{self, mtl, BackendRenderTarget, DirectContext, SurfaceOrigin},
    Canvas, ColorType, ISize,
};
use winit::{
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize},
    event::{ElementState, Event, MouseButton, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::macos::WindowExtMacOS,
};

use crate::{
    ui::{Container, TCtrl},
    utils::ScaleDpi,
    IPoint,
};

pub type WinitWindow = winit::window::Window;

pub trait TWindowDelegate {
    fn on_draw(&mut self, _window: &Window, _canvas: &mut Canvas) {}
    fn on_resize(&mut self, _size: ISize) {}
    fn on_mouse_moved(&mut self, _window: &mut Window, _pos: IPoint) {}
}

struct DefWindowDelegate {}

impl TWindowDelegate for DefWindowDelegate {}

pub struct WindowBuilder {
    // size: ISize,
    delegate: Option<Box<dyn TWindowDelegate>>,

    window: winit::window::Window,
    window_events_loop: Option<EventLoop<()>>,

    root_container: Option<Container>,
}

pub struct Window {
    delegate: Option<Box<dyn TWindowDelegate>>,

    window: winit::window::Window,
    context: DirectContext,

    metal_layer: MetalLayer,
    command_queue: metal::CommandQueue,

    root_container: Option<Container>,

    need_rerender: bool,
    dpi_cache: ScaleDpi,
}

impl WindowBuilder {
    pub fn new(title: &str, size: ISize) -> Self {
        let wnd_size = LogicalSize::new(size.width, size.height);
        let window_events_loop = EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_inner_size(wnd_size)
            .with_title(title)
            .build(&window_events_loop)
            .unwrap();

        Self {
            // size,
            delegate: None,

            window,
            window_events_loop: Some(window_events_loop),
            root_container: None,
        }
    }

    pub fn set_root_container(&mut self, container: Container) {
        self.root_container = Some(container);
    }

    pub fn set_delegate(&mut self, d: Box<dyn TWindowDelegate>) {
        self.delegate = Some(d);
    }

    pub fn run(mut self) {
        let window_events_loop = self.window_events_loop.take().unwrap();

        let device = Device::system_default().expect("no device found");

        let metal_layer = {
            let draw_size = self.window.inner_size();
            let layer = MetalLayer::new();
            layer.set_device(&device);
            layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
            layer.set_presents_with_transaction(false);

            unsafe {
                let view = self.window.ns_view() as cocoa_id;
                view.setWantsLayer(YES);
                view.setLayer(layer.as_ref() as *const _ as _);
            }
            layer.set_drawable_size(CGSize::new(draw_size.width as f64, draw_size.height as f64));
            layer
        };

        let command_queue = device.new_command_queue();

        let backend = unsafe {
            mtl::BackendContext::new(
                device.as_ptr() as mtl::Handle,
                command_queue.as_ptr() as mtl::Handle,
                std::ptr::null(),
            )
        };

        let context = DirectContext::new_metal(&backend, None).unwrap();
        let dpi_cache = ScaleDpi::new(self.window.scale_factor());

        let mut window = Window {
            delegate: self.delegate,
            window: self.window,
            context,

            metal_layer,
            command_queue,

            root_container: self.root_container,

            need_rerender: false,
            dpi_cache,
        };

        window_events_loop.run(move |event, _, control_flow| {
            autoreleasepool(|| {
                window.on_event(event, control_flow);
            });
        });
    }
}

impl Window {
    fn on_event(&mut self, event: Event<'_, ()>, control_flow: &mut ControlFlow) {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(start_cause) => self.on_event_new(start_cause),
            Event::WindowEvent { event, .. } => self.on_event_window(event, control_flow),
            Event::RedrawRequested(_) => self.on_draw(),
            _ => {}
        }
    }

    fn on_event_new(&mut self, start_cause: StartCause) {
        match start_cause {
            StartCause::Init => self.on_init(),
            _ => {}
        }
    }

    fn on_init(&mut self) {
        match self.root_container.as_mut() {
            Some(c) => {
                c.update_dpi(&self.dpi_cache);
                c.update(&self.dpi_cache);
            }
            None => (),
        };
    }

    fn on_event_window(&mut self, event: WindowEvent<'_>, control_flow: &mut ControlFlow) {
        use winit::event::WindowEvent::*;

        match event {
            CloseRequested => *control_flow = ControlFlow::Exit,
            Resized(size) => self.on_resize(size),
            MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                position,
                ..
            } => self.on_lbtn_down(IPoint::new(position.x as i32, position.y as i32)),
            MouseInput {
                state: ElementState::Released,
                button: MouseButton::Left,
                position,
                ..
            } => self.on_lbtn_up(self.tran_point(position)),
            CursorMoved { position, .. } => self.on_mouse_moved(self.tran_point(position)),
            _ => (),
        }
    }

    fn tran_point(&self, p: PhysicalPosition<f64>) -> IPoint {
        self.dpi_cache.rescale(IPoint::new(p.x as i32, p.y as i32))
    }

    fn on_resize(&mut self, size: PhysicalSize<u32>) {
        self.metal_layer
            .set_drawable_size(CGSize::new(size.width as f64, size.height as f64));

        match self.root_container.as_mut() {
            Some(c) => {
                c.update_self(
                    ISize::new(size.width as i32, size.height as i32),
                    &self.dpi_cache,
                );

                c.update(&self.dpi_cache);
                self.need_rerender = true;
            }
            None => (),
        };

        self.window.request_redraw();

        if self.delegate.is_some() {
            self.delegate
                .as_mut()
                .unwrap()
                .on_resize(ISize::new(size.width as i32, size.height as i32));
        }
    }

    fn on_draw(&mut self) {
        if !self.need_rerender {
            return;
        }
        self.need_rerender = false;

        let drawable = self.metal_layer.next_drawable();
        if drawable.is_none() {
            return;
        }
        let drawable = drawable.unwrap();
        let drawable_size = {
            let size = self.metal_layer.drawable_size();
            ISize::new(size.width as i32, size.height as i32)
        };

        let mut surface = unsafe {
            let texture_info = mtl::TextureInfo::new(drawable.texture().as_ptr() as mtl::Handle);

            let backend_render_target = BackendRenderTarget::new_metal(
                (drawable_size.width, drawable_size.height),
                1,
                &texture_info,
            );

            gpu::surfaces::wrap_backend_render_target(
                &mut self.context,
                &backend_render_target,
                SurfaceOrigin::TopLeft,
                ColorType::BGRA8888,
                None,
                None,
            )
            .unwrap()
        };
        let dpi = ScaleDpi::new(self.window.scale_factor());

        // Draw UI
        let canvas = surface.canvas();
        canvas.clear(skia_safe::colors::WHITE);
        if self.root_container.is_some() {
            let c = self.root_container.as_mut().unwrap();
            c.render(canvas, &dpi);
        }

        let mut dg = self.delegate.take();
        if dg.is_some() {
            dg.as_mut().unwrap().on_draw(self, canvas);
        }
        self.delegate = dg;

        self.context.flush_and_submit();
        drop(surface);

        let command_buffer = self.command_queue.new_command_buffer();
        command_buffer.present_drawable(drawable);
        command_buffer.commit();
        self.dpi_cache = dpi;
    }

    fn on_lbtn_down(&self, pos: IPoint) {
        let ctrl = self.get_ctrl_by_pos(&pos).unwrap();
        println!("click {}", ctrl.get_inner().name);
    }
    fn on_lbtn_up(&self, _pos: IPoint) {}
    fn on_mouse_moved(&mut self, pos: IPoint) {
        let mut dg = self.delegate.take();
        if dg.is_some() {
            dg.as_mut().unwrap().on_mouse_moved(self, pos);

            self.delegate = dg;
        }
    }

    pub fn get_ctrl_by_name(&self, name: &str) -> Option<&dyn TCtrl> {
        if self.root_container.is_none() {
            return None;
        }

        self.root_container.as_ref().unwrap().get_ctrl_by_name(name)
    }

    pub fn get_mut_ctrl_by_name(&mut self, name: &str) -> Option<&mut dyn TCtrl> {
        if self.root_container.is_none() {
            return None;
        }

        self.root_container
            .as_mut()
            .unwrap()
            .get_mut_ctrl_by_name(name)
    }

    pub fn get_ctrl_by_pos(&self, pos: &IPoint) -> Option<&dyn TCtrl> {
        if self.root_container.is_none() {
            return None;
        }
        let root = self.root_container.as_ref().unwrap();

        Some(root.get_ctrl_by_pos(&self.dpi_cache.scale(pos.clone())))
    }

    pub fn redraw(&mut self) {
        self.need_rerender = true;
        self.window.request_redraw();
    }
}
