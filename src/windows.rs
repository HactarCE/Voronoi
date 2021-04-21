use imgui::*;

use crate::Point;

pub fn build_all(ui: &Ui, points: &mut Vec<Point>, target_dimensions: (u32, u32)) {
    Window::new(im_str!("Points"))
        .resizable(false)
        .build(ui, || {
            ui.text("Left click to add a new point");
            ui.text("Shift + left click to move a point");
            ui.text("Right click to remove a point");
            ui.separator();

            for (i, p) in points.iter_mut().enumerate() {
                let label = ImString::from(format!("Point #{}", i + 1));
                ColorEdit::new(&ImString::from(format!("Color #{}", i + 1)), &mut p.color)
                    .build(ui);
                ui.same_line(0.0);
                {
                    let w = ui.push_item_width(-1.0);

                    let min_size = std::cmp::min(target_dimensions.0, target_dimensions.1);
                    let extent = min_size as i32 / 2;

                    Slider::new(&label)
                        .range(-extent..=extent)
                        .build_array(ui, &mut p.pos);

                    w.pop(ui);
                }
            }
        });
}
