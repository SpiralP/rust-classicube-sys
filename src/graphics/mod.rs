mod owned_gfx_texture;
mod owned_vertex_buffer;

pub use self::{owned_gfx_texture::*, owned_vertex_buffer::*};
use crate::{
    bindings::{
        GfxResourceID, Gfx_DrawVb_IndexedTris, Gfx_SetDynamicVbData, PackedCol, Texture,
        VertexTextured,
    },
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
    let x1: f32 = f32::from(tex.x);
    let x2: f32 = f32::from(tex.x) + f32::from(tex.width);
    let y1: f32 = f32::from(tex.y);
    let y2: f32 = f32::from(tex.y) + f32::from(tex.height);

    [
        VertexTextured {
            x: x1,
            y: y1,
            z: 0.0,
            Col: col,
            U: tex.uv.u1,
            V: tex.uv.v1,
        },
        VertexTextured {
            x: x2,
            y: y1,
            z: 0.0,
            Col: col,
            U: tex.uv.u2,
            V: tex.uv.v1,
        },
        VertexTextured {
            x: x2,
            y: y2,
            z: 0.0,
            Col: col,
            U: tex.uv.u2,
            V: tex.uv.v2,
        },
        VertexTextured {
            x: x1,
            y: y2,
            z: 0.0,
            Col: col,
            U: tex.uv.u1,
            V: tex.uv.v2,
        },
    ]
}
