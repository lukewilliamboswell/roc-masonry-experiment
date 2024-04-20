use masonry::app_driver::{AppDriver, DriverCtx};
use masonry::event_loop_runner::EventLoopRunner;
use masonry::widget::prelude::*;
use masonry::Action;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

mod ui;

struct Program {
    roc_program: host::ProgramForHost,
}

impl Program {
    fn build_root_widget(&mut self) -> impl Widget {
        let root = self.roc_program.render();

        ui::from_json_to_masonry(root)
    }
}

// fn elems_to_widgets(_elem: roc_app::Elem) -> impl Widget {

// }

impl AppDriver for Program {
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
    let initial_width = 400.0;
    let initial_height = 400.0;

    let mut roc_program = host::program_for_host();

    roc_program.init(roc_app::Bounds {
        height: initial_height,
        width: initial_width,
    });

    let mut program = Program { roc_program };

    let event_loop = EventLoop::new().unwrap();
    let window_size = LogicalSize::new(initial_width, initial_height);
    let window = WindowBuilder::new()
        .with_title("title.as_str()")
        .with_resizable(true)
        .with_min_inner_size(window_size)
        .build(&event_loop)
        .unwrap();

    let runner = EventLoopRunner::new(program.build_root_widget(), window, event_loop, program);

    runner.run().unwrap();
}
