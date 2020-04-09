mod owned_texture;
mod owned_vertex_buffer;

pub use self::{owned_texture::*, owned_vertex_buffer::*};
use crate::bindings::*;
use std::os::raw::{c_int, c_void};

pub unsafe fn Gfx_UpdateDynamicVb_IndexedTris(
    vb: GfxResourceID,
    vertices: *mut c_void,
    vCount: c_int,
) {
    Gfx_SetDynamicVbData(vb, vertices, vCount);
    Gfx_DrawVb_IndexedTris(vCount);
}

pub unsafe fn Gfx_Make2DQuad(tex: &mut Texture, col: PackedCol) -> [VertexP3fT2fC4b; 4] {
    let mut x1: f32 = tex.X as _;
    let mut x2: f32 = (tex.X as f32 + tex.Width as f32) as _;
    let mut y1: f32 = tex.Y as _;
    let mut y2: f32 = (tex.Y as f32 + tex.Height as f32) as _;
    // VertexP3fT2fC4b* v = *vertices;

    // #ifdef CC_BUILD_D3D9
    // NOTE: see "https://msdn.microsoft.com/en-us/library/windows/desktop/bb219690(v=vs.85).aspx",
    // i.e. the msdn article called "Directly Mapping Texels to Pixels (Direct3D 9)" for why we have to do this.
    x1 -= 0.5;
    x2 -= 0.5;
    y1 -= 0.5;
    y2 -= 0.5;
    // #endif

    [
        VertexP3fT2fC4b {
            X: x1,
            Y: y1,
            Z: 0 as _,
            Col: col,
            U: tex.uv.U1,
            V: tex.uv.V1,
        },
        VertexP3fT2fC4b {
            X: x2,
            Y: y1,
            Z: 0 as _,
            Col: col,
            U: tex.uv.U2,
            V: tex.uv.V1,
        },
        VertexP3fT2fC4b {
            X: x2,
            Y: y2,
            Z: 0 as _,
            Col: col,
            U: tex.uv.U2,
            V: tex.uv.V2,
        },
        VertexP3fT2fC4b {
            X: x1,
            Y: y2,
            Z: 0 as _,
            Col: col,
            U: tex.uv.U1,
            V: tex.uv.V2,
        },
    ]
}
