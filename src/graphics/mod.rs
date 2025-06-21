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
    unsafe {
        Gfx_SetDynamicVbData(vb, vertices, vCount);
        Gfx_DrawVb_IndexedTris(vCount);
    }
}

pub fn Gfx_Make2DQuad(tex: &mut Texture, col: PackedCol) -> [VertexTextured; 4] {
    let x1: f32 = tex.x as _;
    let x2: f32 = (tex.x as f32 + tex.width as f32) as _;
    let y1: f32 = tex.y as _;
    let y2: f32 = (tex.y as f32 + tex.height as f32) as _;

    [
        VertexTextured {
            x: x1,
            y: y1,
            z: 0 as _,
            Col: col,
            U: tex.uv.u1,
            V: tex.uv.v1,
        },
        VertexTextured {
            x: x2,
            y: y1,
            z: 0 as _,
            Col: col,
            U: tex.uv.u2,
            V: tex.uv.v1,
        },
        VertexTextured {
            x: x2,
            y: y2,
            z: 0 as _,
            Col: col,
            U: tex.uv.u2,
            V: tex.uv.v2,
        },
        VertexTextured {
            x: x1,
            y: y2,
            z: 0 as _,
            Col: col,
            U: tex.uv.u1,
            V: tex.uv.v2,
        },
    ]
}
