#[derive(Debug, Clone)]
pub struct Config {
    /// Lp distance metric.
    pub lp: f32,
    /// Kind of Voronoi cell to draw.
    pub voronoi_kind: VoronoiKind,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            lp: 2.0,
            voronoi_kind: VoronoiKind::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VoronoiKind {
    Near,
    Far,
}
impl Default for VoronoiKind {
    fn default() -> Self {
        Self::Near
    }
}
