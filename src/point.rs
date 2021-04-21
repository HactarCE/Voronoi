use random_color::{Luminosity, RandomColor};

#[derive(Debug, Default, Copy, Clone)]
pub struct Point {
    pub pos: [i32; 2],
    pub color: [f32; 3],
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        let mut ret = Self {
            pos: [x, y],
            color: Default::default(),
        };
        ret.set_random_color();
        ret
    }

    pub fn set_random_color(&mut self) {
        let [r, g, b] = RandomColor::new()
            .luminosity(Luminosity::Bright)
            .to_rgb_array();
        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;
        self.color = [r, g, b];
    }

    pub fn pos_i32_tuple(self) -> (i32, i32) {
        (self.pos[0], self.pos[1])
    }
    pub fn color_f32_tuple(self) -> (f32, f32, f32, f32) {
        (self.color[0], self.color[1], self.color[2], 1.0)
    }
}
