use masonry::app_driver::{AppDriver, DriverCtx};
use masonry::event_loop_runner::EventLoopRunner;
use masonry::widget::prelude::*;
use masonry::widget::{Button, Flex, Label};
use masonry::Action;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;

struct Driver;

impl AppDriver for Driver {
    fn on_action(&mut self, _ctx: &mut DriverCtx<'_>, _widget_id: WidgetId, action: Action) {
        match action {
            Action::ButtonPressed => {
                println!("Hello");
            }
            _ => {}
        }
    }
}

pub fn main() {

    let mut roc_program: host::ProgramForHost = host::program_for_host();

    roc_program.init(roc_app::Bounds { height: 10.0, width: 20.0 });

    let elems = roc_program.render();

    dbg!(elems);

    // let title: roc_std::RocStr = host::main_for_host();

    let event_loop = EventLoop::new().unwrap();
    let window_size = LogicalSize::new(400.0, 400.0);
    let window = WindowBuilder::new()
        .with_title("title.as_str()")
        .with_resizable(true)
        .with_min_inner_size(window_size)
        .build(&event_loop)
        .unwrap();

    let runner = EventLoopRunner::new(build_root_widget(), window, event_loop, Driver);
    
    runner.run().unwrap();
}

fn build_root_widget() -> impl Widget {
    let label = Label::new("Hello").with_text_size(32.0);

    // a button that says "hello"
    let button = Button::new("Say hello");

    // arrange the two widgets vertically, with some padding
    Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(button)
}