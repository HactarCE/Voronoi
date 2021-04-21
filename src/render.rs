use glium::index::PrimitiveType;
use glium::texture::{IntegralTexture1d, MipmapsOption, SrgbTexture1d, UncompressedIntFormat};
use glium::{Frame, Program, Surface, VertexBuffer};
use lazy_static::lazy_static;
use send_wrapper::SendWrapper;

use crate::{Point, DISPLAY};

const POINT_RADIUS: f32 = 3.0_f32;

/// Vertex containing only a 2D NDC position.
#[derive(Debug, Default, Copy, Clone)]
pub struct Vertex2D {
    pub ndc: [f32; 2],
}
glium::implement_vertex!(Vertex2D, ndc);

lazy_static! {
    static ref VORONOI_PROGRAM: SendWrapper<Program> = SendWrapper::new(
        glium::program!(
            &**DISPLAY,
            140 => {
                vertex: include_str!("shaders/voronoi.vert"),
                fragment: include_str!("shaders/voronoi.frag"),
                outputs_srgb: true,
            },
        )
        .expect("Failed to compile shader")
    );
    static ref VBO: SendWrapper<VertexBuffer<Vertex2D>> = SendWrapper::new(
        VertexBuffer::immutable(
            &**DISPLAY,
            &[
                Vertex2D { ndc: [-1.0, -1.0] },
                Vertex2D { ndc: [1.0, -1.0] },
                Vertex2D { ndc: [-1.0, 1.0] },
                Vertex2D { ndc: [1.0, 1.0] },
            ]
        )
        .expect("Failed to create vertex buffer")
    );
}

pub fn draw_voronoi(target: &mut Frame, points: &[Point]) {
    let (w, h) = target.get_dimensions();

    target.clear_color_srgb(0.5, 0.0, 0.5, 1.0);

    let points_tex_data: Vec<(i32, i32)> = points.iter().map(|p| p.pos_i32_tuple()).collect();
    let points_tex = IntegralTexture1d::with_format(
        &**DISPLAY,
        points_tex_data,
        UncompressedIntFormat::I32I32,
        MipmapsOption::NoMipmap,
    )
    .expect("Failed to create texture");
    let colors_tex_data: Vec<(f32, f32, f32, f32)> =
        points.iter().map(|c| c.color_f32_tuple()).collect();
    let colors_tex =
        SrgbTexture1d::new(&**DISPLAY, colors_tex_data).expect("Failed to create texture");

    target
        .draw(
            &**VBO,
            &glium::index::NoIndices(PrimitiveType::TriangleStrip),
            &*VORONOI_PROGRAM,
            &glium::uniform! {
                target_size: [w as f32, h as f32],

                points_tex: points_tex.sampled(),
                colors_tex: colors_tex.sampled(),
                point_distance: POINT_RADIUS.powf(2.0),
            },
            &glium::DrawParameters::default(),
        )
        .expect("Failed to draw to screen");
}
