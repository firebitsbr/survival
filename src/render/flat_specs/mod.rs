use amethyst::renderer::*;
use gfx::format::{ChannelType, Format, SurfaceType};
use gfx::pso::buffer::Element;
use gfx::traits::Pod;
use glsl_layout::*;
use serde::{Deserialize, Serialize};

pub const VERT_SRC: &[u8] = include_bytes!("sprite_v.glsl");
pub const FRAG_SRC: &[u8] = include_bytes!("sprite_f.glsl");

macro_rules! attr {
    ($typ:ident, $name:expr,$repr:ty, $surface:path) => {
        #[derive(Clone, Debug)]
        enum $typ {}

        impl Attribute for $typ {
            const NAME: &'static str = $name;
            const FORMAT: Format = Format($surface, ChannelType::Float);
            const SIZE: u32 = ::std::mem::size_of::<$repr>() as u32;
            type Repr = $repr;
        }
    };
    ($off:expr, $typ:ident) => {
        impl With<$typ> for SpriteInstance {
            const FORMAT: AttributeFormat = Element {
                offset: $off,
                format: $typ::FORMAT,
            };
        }
    };
    ($off:expr, $typ:ident, $($a:ident),*) => {
        attr!($off, $typ);
        attr!($off + $typ::SIZE, $($a),*);
    };
}

attr!(DirX, "dir_x", [f32; 2], SurfaceType::R32_G32);
attr!(DirY, "dir_y", [f32; 2], SurfaceType::R32_G32);
attr!(Pos, "pos", [f32; 2], SurfaceType::R32_G32);
attr!(OffsetU, "u_offset", [f32; 2], SurfaceType::R32_G32);
attr!(OffsetV, "v_offset", [f32; 2], SurfaceType::R32_G32);
attr!(Depth, "depth", f32, SurfaceType::R32);

attr!(0, DirX, DirY, Pos, OffsetU, OffsetV, Depth, Color);

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpriteInstance {
    pub dir_x: [f32; 2],
    pub dir_y: [f32; 2],
    pub pos: [f32; 2],
    pub u_offset: [f32; 2],
    pub v_offset: [f32; 2],
    pub depth: f32,
    pub color: [f32; 4],
}

unsafe impl Pod for SpriteInstance {}

impl VertexFormat for SpriteInstance {
    const ATTRIBUTES: Attributes<'static> = &[
        (DirX::NAME, <Self as With<DirX>>::FORMAT),
        (DirY::NAME, <Self as With<DirY>>::FORMAT),
        (Pos::NAME, <Self as With<Pos>>::FORMAT),
        (OffsetU::NAME, <Self as With<OffsetU>>::FORMAT),
        (OffsetV::NAME, <Self as With<OffsetV>>::FORMAT),
        (Depth::NAME, <Self as With<Depth>>::FORMAT),
        (Color::NAME, <Self as With<Color>>::FORMAT),
    ];
}

impl SpriteInstance {
    pub fn attributes() -> Attributes<'static> {
        <Self as Query<(DirX, DirY, Pos, OffsetU, OffsetV, Depth, Color)>>::QUERIED_ATTRIBUTES
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Uniform)]
pub struct TextureOffsetPod {
    pub u_offset: vec2,
    pub v_offset: vec2,
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Uniform)]
pub struct ViewArgs {
    proj: mat4,
    view: mat4,
}

impl ViewArgs {
    pub fn new(proj: mat4, view: mat4) -> Self {
        ViewArgs { proj, view }
    }
}
