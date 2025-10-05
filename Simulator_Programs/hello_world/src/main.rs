use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};
use mousefood::embedded_graphics::geometry;
use std::io::Error;
use mousefood::prelude::*;
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::{Frame, Terminal};

fn main() -> Result<(), Error> {
    let mut simulator_window = Window::new(
        "Hello World in Mousefood",
        &OutputSettings {
            scale: 4,
            max_fps: 30,
            ..Default::default()
        },
    );

    let mut display = SimulatorDisplay::<Bgr565>::new(geometry::Size::new(128, 64));

    let backend_config = EmbeddedBackendConfig {
        flush_callback: Box::new(move |display| {
            simulator_window.update(display);
            if simulator_window.events().any(|e| e == SimulatorEvent::Quit) {
                panic!("Simulator Window Closed");
            }
        }),
        ..Default::default()
    };

    let backend: EmbeddedBackend<SimulatorDisplay<_>, _> = EmbeddedBackend::new(&mut display, backend_config);

    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(draw)?;
    }
}

fn draw(frame: &mut Frame) {
    let text = "Hello World";
    let paragraph = Paragraph::new(text.dark_gray()).wrap(Wrap { trim: true });

    let bordered_block = Block::bordered()
        .border_style(Style::new().yellow())
        .title("Mousefood");

    frame.render_widget(paragraph.block(bordered_block), frame.area());
}
