mod common;

use common::*;
use imgui::*;
use simple_logger::SimpleLogger;
use std::error::Error;

const APP_NAME: &str = "collapsing headers";

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init()?;
    let mut state = State {
        render_closable: true,
    };
    System::new(APP_NAME)?.run((), move |run, ui, _| {
        let w = Window::new("Collapsing header")
            .opened(run)
            .position([20.0, 20.0], Condition::Appearing)
            .size([700.0, 500.0], Condition::Appearing);
        w.build(ui, || {
            if CollapsingHeader::new("I'm a collapsing header. Click me!").build(ui) {
                ui.text(
                    "A collapsing header can be used to toggle rendering of a group of widgets",
                );
            }

            ui.spacing();
            if CollapsingHeader::new("I'm open by default")
                .default_open(true)
                .build(ui)
            {
                ui.text("You can still close me with a click!");
            }

            ui.spacing();
            if CollapsingHeader::new("I only open with double-click")
                .open_on_double_click(true)
                .build(ui)
            {
                ui.text("Double the clicks, double the fun!");
            }

            ui.spacing();
            if CollapsingHeader::new("I don't have an arrow")
                .bullet(true)
                .build(ui)
            {
                ui.text("Collapsing headers can use a bullet instead of an arrow");
            }

            ui.spacing();
            if CollapsingHeader::new("I only open if you click the arrow")
                .open_on_arrow(true)
                .build(ui)
            {
                ui.text("You clicked the arrow");
            }

            ui.spacing();
            ui.checkbox(
                "Toggle rendering of the next example",
                &mut state.render_closable,
            );
            if CollapsingHeader::new("I've got a separate close button")
                .build_with_close_button(ui, &mut state.render_closable)
            {
                ui.text("I've got contents just like any other collapsing header");
            }
        });
    })?;

    Ok(())
}

struct State {
    render_closable: bool,
}
