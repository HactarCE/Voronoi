use random_color::{Luminosity, RandomColor};

#[derive(Debug, Default, Copy, Clone)]
pub struct Point {
    pub pos: [i32; 2],
    pub color: [f32; 3],
}
impl Point {
    pub fn new(pos: [i32; 2]) -> Self {
        let color = random_color();
        Self { pos, color }
    }

    pub fn set_random_color(&mut self) {
        self.color = random_color();
    }

    pub fn pos_i32_tuple(self) -> (i32, i32) {
        (self.pos[0], self.pos[1])
    }
    pub fn color_f32_tuple(self) -> (f32, f32, f32, f32) {
        (self.color[0], self.color[1], self.color[2], 1.0)
    }
}

pub fn nearest_point_idx(points: &[Point], pos: [i32; 2]) -> Option<usize> {
    (0..points.len()).min_by_key(|&i| {
        let p = points[i];
        let dx = p.pos[0] - pos[0];
        let dy = p.pos[1] - pos[1];
        dx * dx + dy * dy
    })
}

fn random_color() -> [f32; 3] {
    let [r, g, b] = RandomColor::new()
        .luminosity(Luminosity::Bright)
        .to_rgb_array();
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    [r, g, b]
}
