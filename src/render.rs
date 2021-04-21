use glium::{Frame, Surface};

use crate::DISPLAY;

pub fn draw_voronoi(frame: &mut Frame, points: &[[isize; 2]]) {
    frame.clear_color_srgb(0.5, 0.0, 0.5, 1.0);
}
