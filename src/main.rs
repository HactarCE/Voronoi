use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::{
    glutin::event::{Event, StartCause, WindowEvent},
    Surface,
};
use imgui::FontSource;
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use send_wrapper::SendWrapper;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

#[macro_use]
extern crate glium;
#[macro_use]
extern crate lazy_static;

mod point;
mod render;
mod windows;

use point::Point;

const TITLE: &str = "Voronoi";
const FONT_SIZE: f32 = 18.0;

lazy_static! {
    static ref FRAME_DURATION: Duration = Duration::from_secs_f64(1.0 / 60.0);
    static ref EVENT_LOOP: SendWrapper<RefCell<Option<EventLoop<()>>>> =
        SendWrapper::new(RefCell::new(Some(EventLoop::new())));
    static ref DISPLAY: SendWrapper<glium::Display> = SendWrapper::new({
        let wb = WindowBuilder::new().with_title(TITLE.to_owned());
        let cb = ContextBuilder::new().with_vsync(true);
        glium::Display::new(wb, cb, EVENT_LOOP.borrow().as_ref().unwrap())
            .expect("Failed to initialize display")
    });
}

fn main() {
    let display = &**DISPLAY;

    // Initialize runtime data.
    let mut events_buffer = VecDeque::new();
    let mut points = vec![
        Point::new(100, 0),
        Point::new(-50, 40),
        Point::new(-60, -20),
        Point::new(0, 90),
    ];

    // Initialize imgui.
    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);
    let mut platform = WinitPlatform::init(&mut imgui);
    let gl_window = display.gl_window();
    let window = gl_window.window();
    platform.attach_window(imgui.io_mut(), window, HiDpiMode::Default);

    // Initialize imgui fonts.
    let font_size = FONT_SIZE;
    imgui.fonts().add_font(&[FontSource::TtfData {
        data: include_bytes!("../resources/font/NotoSans-Regular.ttf"),
        size_pixels: font_size,
        config: None,
    }]);

    // Initialize imgui renderer.
    let mut renderer = Renderer::init(&mut imgui, display).expect("Failed to initialize renderer");

    // Main loop
    let mut first = true;
    let mut target_dimensions = (0, 0);
    let mut last_frame_time = Instant::now();
    let mut next_frame_time = Instant::now();
    EVENT_LOOP
        .borrow_mut()
        .take()
        .unwrap()
        .run(move |event, _ev_loop, control_flow| {
            // Handle events.
            let mut now = Instant::now();
            let mut do_frame = false;
            match event.to_static() {
                Some(Event::NewEvents(cause)) => match cause {
                    StartCause::ResumeTimeReached {
                        start: _,
                        requested_resume,
                    } => {
                        now = requested_resume;
                        do_frame = true;
                    }
                    StartCause::Init => {
                        next_frame_time = now;
                        do_frame = true;
                    }
                    _ => (),
                },

                // The program is about to exit.
                Some(Event::LoopDestroyed) => (),

                // Queue the event to be handled next time we render
                // everything.
                Some(ev) => events_buffer.push_back(ev),

                // Ignore this event.
                None => (),
            };

            if do_frame && next_frame_time <= now {
                next_frame_time = now + *FRAME_DURATION;
                if next_frame_time < Instant::now() {
                    // Skip a frame (or several).
                    next_frame_time = Instant::now() + *FRAME_DURATION;
                }
                *control_flow = ControlFlow::WaitUntil(next_frame_time);

                // Prep imgui for event handling.
                let imgui_io = imgui.io_mut();
                platform
                    .prepare_frame(imgui_io, gl_window.window())
                    .expect("Failed to start frame");

                if let Some(delta) = now.checked_duration_since(last_frame_time) {
                    imgui_io.update_delta_time(delta);
                }
                last_frame_time = now;

                for ev in events_buffer.drain(..) {
                    // Let imgui handle events.
                    platform.handle_event(imgui_io, gl_window.window(), &ev);

                    // TODO: handle events for visualization.

                    // Handle important window events.
                    match ev {
                        Event::WindowEvent { event, .. } => match event {
                            // Handle window close event.
                            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                            _ => (),
                        },
                        _ => (),
                    }
                }

                // Prep imgui for rendering.
                let imgui_has_mouse = imgui_io.want_capture_mouse;
                let ui = imgui.frame();
                if first {
                    ui.set_color_edit_options(
                        imgui::ColorEditFlags::NO_ALPHA
                            | imgui::ColorEditFlags::NO_INPUTS
                            | imgui::ColorEditFlags::NO_LABEL
                            | imgui::ColorEditFlags::PICKER_HUE_WHEEL,
                    );
                    first = false;
                }
                windows::build_all(&ui, &mut points, target_dimensions);

                let mut target = display.draw();
                target_dimensions = target.get_dimensions();

                render::draw_voronoi(&mut target, &points);

                // Render imgui.
                platform.prepare_render(&ui, gl_window.window());
                let draw_data = ui.render();
                renderer
                    .render(&mut target, draw_data)
                    .expect("Error while rendering imgui");

                // Put it all on the screen.
                target.finish().expect("Failed to swap buffers");
            }
        })
}
