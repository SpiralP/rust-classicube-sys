mod owned_gfx_texture;
mod owned_vertex_buffer;

pub use self::{owned_gfx_texture::*, owned_vertex_buffer::*};
use crate::{
    bindings::*,
    std_types::{c_int, c_void},
};

#[allow(clippy::missing_safety_doc)]
pub unsafe fn Gfx_UpdateDynamicVb_IndexedTris(
    vb: GfxResourceID,
    vertices: *mut c_void,
    vCount: c_int,
) {
    Gfx_SetDynamicVbData(vb, vertices, vCount);
    Gfx_DrawVb_IndexedTris(vCount);
}

pub fn Gfx_Make2DQuad(tex: &mut Texture, col: PackedCol) -> [VertexTextured; 4] {
    let x1: f32 = tex.X as _;
    let x2: f32 = (tex.X as f32 + tex.Width as f32) as _;
    let y1: f32 = tex.Y as _;
    let y2: f32 = (tex.Y as f32 + tex.Height as f32) as _;

    [
        VertexTextured {
            X: x1,
            Y: y1,
            Z: 0 as _,
            Col: col,
            U: tex.uv.U1,
            V: tex.uv.V1,
        },
        VertexTextured {
            X: x2,
            Y: y1,
            Z: 0 as _,
            Col: col,
            U: tex.uv.U2,
            V: tex.uv.V1,
        },
        VertexTextured {
            X: x2,
            Y: y2,
            Z: 0 as _,
            Col: col,
            U: tex.uv.U2,
            V: tex.uv.V2,
        },
        VertexTextured {
            X: x1,
            Y: y2,
            Z: 0 as _,
            Col: col,
            U: tex.uv.U1,
            V: tex.uv.V2,
        },
    ]
}
