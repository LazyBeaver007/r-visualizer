use ratatui::{
    layout::Rect,
    style::{Style, Color},
    widgets::{Paragraph},
    text::{Span, Line},
    Frame,
};
use crate::stale::SharedState;

static mut FRAME: usize = 0;


const PALETTE: [Color; 8] = [
    Color::White,
    Color::Cyan,
    Color::LightYellow,
    Color::Yellow,
    Color::Green,
    Color::Red,
    Color::LightMagenta,
    Color::LightBlue,
];

pub fn draw_black_hole(f: &mut Frame, area: Rect, state: &SharedState) {
    
    let (bass, mids, highs) = {
        if let Ok(st) = state.lock() {
            let bass_slice = &st.freq_bins[0..6];
            let mid_slice  = &st.freq_bins[6..20];
            let high_slice = &st.freq_bins[20..40];

            let b = bass_slice.iter().cloned().fold(0.0_f32, f32::max);
            let m = mid_slice.iter().cloned().fold(0.0_f32, f32::max);
            let h = high_slice.iter().cloned().fold(0.0_f32, f32::max);

            (b, m, h)
        } else {
            (0.0, 0.0, 0.0)
        }
    };

   
    let pulse = (bass / 8.0).min(6.0);  
    let distortion = (mids / 6.0).min(5.0); 
    let scatter_strength = (highs / 5.0).min(3.0); 

   
    let star_r = 10.0 + pulse * 1.5;

  
    let hole_r = 3.0 + (pulse * 0.8);

    let frame = unsafe {
        FRAME += 1;
        FRAME
    };

    let color = PALETTE[frame % PALETTE.len()];

    let mut rows: Vec<String> = Vec::new();

    let w = area.width as i32;
    let h = area.height as i32;
    let cx = w / 2;
    let cy = h / 2;

    for y in 0..h {
        let mut row = String::new();

        for x in 0..w {
            let dx = (x - cx) as f32;
            let dy = (y - cy) as f32;

            let dist = (dx * dx + dy * dy).sqrt();

            let angle = dy.atan2(dx);

            
            let arm_wave =
                (angle * 5.0).sin().abs() * distortion * 3.5;

            let star_radius = star_r + arm_wave;

           
            let scatter_trigger =
                ((angle * 12.0 + frame as f32 / 3.0).sin().abs()
                + (dist / 7.0).sin().abs())
                * scatter_strength;

           
            let ch = if dist < hole_r {
                '0'
            } else if dist < hole_r * 1.5 {
                '/'
            } else if dist < star_radius {
                '*'
            } else if scatter_trigger > 1.0 && dist < star_radius + 3.0 {
                '.'
            } else {
                ' '
            };

            row.push(ch);
        }

        rows.push(row);
    }

   
    let mut lines = Vec::new();
    for row in rows {
        lines.push(Line::from(
            Span::styled(row, Style::default().fg(color)),
        ));
    }

    let paragraph = Paragraph::new(lines)
        .alignment(ratatui::layout::Alignment::Left);

    f.render_widget(paragraph, area);
}

