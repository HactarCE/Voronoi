use imgui::*;

pub fn build(ui: &Ui, points: &mut Vec<[isize; 2]>) {
    Window::new(im_str!("Voronoi diagram configuration"))
        .resizable(false)
        .build(ui, || ui.text("hello"));
}
