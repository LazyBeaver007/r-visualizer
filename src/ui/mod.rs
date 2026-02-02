/* mod.rs
 * link ratatui and custom renderers
 * root entry for drawing the screen
 */


pub mod blackhole;
pub mod constellations;
use ratatui::prelude::Rect;
use ratatui::text::Line;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    Frame,
    widgets::{Paragraph},
    layout::{Layout, Constraint, Direction},
};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode},
    execute,
};
use std::io::{stdout};
use crate::stale::SharedState;

pub fn start_ui_loop(state: SharedState) -> anyhow::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), crossterm::terminal::EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| draw_ui(f, &state))?;
        std::thread::sleep(std::time::Duration::from_millis(30));
    }
}

fn draw_ui(f: &mut Frame, state: &SharedState) {
    let size = f.size();

    // Layout:
    // 80% center (black hole)
    // 20% bottom (debug text)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(80),
            Constraint::Percentage(20),
        ])
        .split(size);

    // Draw black hole in center
    blackhole::draw_black_hole(f, chunks[0], state);

    // Draw debug FFT bins
    draw_debug(f, chunks[1], state);
}

fn draw_debug(f: &mut Frame, area: Rect, state: &SharedState) {
    let text = {
        if let Ok(st) = state.lock() {
            format!("{:?}", st.freq_bins)
        } else {
            "LOCK ERROR".to_string()
        }
    };

    let paragraph = Paragraph::new(Line::from(text));
    f.render_widget(paragraph, area);
}
