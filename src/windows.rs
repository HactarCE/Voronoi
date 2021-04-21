use imgui::*;

use crate::{Config, Point, VoronoiKind};

pub fn build_points_window(ui: &Ui, points: &mut Vec<Point>, target_dimensions: (u32, u32)) {
    let min_size = std::cmp::min(target_dimensions.0, target_dimensions.1);
    let extent = min_size as i32 / 2;

    Window::new(im_str!("Points"))
        .size_constraints([250.0, -1.0], [250.0, -1.0])
        .resizable(false)
        .always_auto_resize(true)
        .build(ui, || {
            ui.text("Left click to move a point");
            ui.text("Shift + left click to assign a new random color");
            ui.text("Ctrl + left click to add a new point");
            ui.text("Right click to remove a point");

            ui.separator();

            for (i, p) in points.iter_mut().enumerate() {
                let label = ImString::from(format!("Point #{}", i + 1));
                ColorEdit::new(&ImString::from(format!("Color #{}", i + 1)), &mut p.color)
                    .build(ui);
                ui.same_line(0.0);

                let w = ui.push_item_width(-1.0);
                Slider::new(&label)
                    .range(-extent..=extent)
                    .build_array(ui, &mut p.pos);
                w.pop(ui);
            }

            if ui.button(
                im_str!("Export to clipboard"),
                [ui.window_content_region_width(), 25.0],
            ) {
                match serde_json::to_string(points) {
                    Ok(s) => crate::clipboard_compat::clipboard_set(s),
                    Err(e) => eprintln!("Error serializing points list: {}", e),
                }
            }
            if ui.button(
                im_str!("Import from clipboard"),
                [ui.window_content_region_width(), 25.0],
            ) {
                if let Some(s) = crate::clipboard_compat::clipboard_get() {
                    match serde_json::from_str(&s) {
                        Ok(pts) => *points = pts,
                        Err(e) => eprintln!("Error deserializing points list: {}", e),
                    }
                }
            }

            ui.separator();

            if ui.button(
                im_str!("Randomize colors"),
                [ui.window_content_region_width(), 30.0],
            ) {
                for p in points {
                    p.set_random_color();
                }
            }
        });
}

pub fn build_config_window(ui: &Ui, config: &mut Config) {
    Window::new(im_str!("Config"))
        .resizable(false)
        .position([720.0, 60.0], Condition::FirstUseEver)
        .build(ui, || {
            ui.text("Kind of Voronoi cell");
            ui.radio_button(
                im_str!("Nearest"),
                &mut config.voronoi_kind,
                VoronoiKind::Near,
            );
            ui.radio_button(
                im_str!("Farthest"),
                &mut config.voronoi_kind,
                VoronoiKind::Far,
            );

            ui.separator();

            ui.text("Lp distance metric");

            let w = ui.push_item_width(-1.0);
            ui.input_float(im_str!("Lp distance metric"), &mut config.lp)
                .build();
            w.pop(ui);
            config.lp = config.lp.clamp(0.5, 10.0);

            for (i, &lp) in [0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 3.0, 10.0]
                .iter()
                .enumerate()
            {
                if i % 4 != 0 {
                    ui.same_line(0.0);
                }

                let label = ImString::from(format!("{}", lp));
                if ui.button(&label, [30.0, 25.0]) {
                    config.lp = lp;
                }
            }
        })
}
