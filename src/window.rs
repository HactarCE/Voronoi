use imgui::*;

use crate::Point;

pub fn build(ui: &Ui, points: &mut Vec<Point>) {
    Window::new(im_str!("Voronoi diagram configuration"))
        .resizable(false)
        .build(ui, || ui.text("hello"));
}
