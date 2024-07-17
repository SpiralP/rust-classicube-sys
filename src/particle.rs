use crate::bindings::*;

pub fn Particle_DoRender(
    size: &Vec2,
    pos: &Vec3,
    rec: &TextureRec,
    col: PackedCol,
) -> [VertexTextured; 4] {
    let sX = size.x * 0.5;
    let sY = size.y * 0.5;
    let mut centre = *pos;
    centre.y += sY;
    let view = unsafe { &Gfx.View };

    let aX = view.row1.x * sX;
    let aY = view.row2.x * sX;
    let aZ = view.row3.x * sX; // right * size.x * 0.5f
    let bX = view.row1.y * sY;
    let bY = view.row2.y * sY;
    let bZ = view.row3.y * sY; // up    * size.y * 0.5f

    [
        VertexTextured {
            x: centre.x - aX - bX,
            y: centre.y - aY - bY,
            z: centre.z - aZ - bZ,
            Col: col,
            U: rec.u1,
            V: rec.v2,
        },
        VertexTextured {
            x: centre.x - aX + bX,
            y: centre.y - aY + bY,
            z: centre.z - aZ + bZ,
            Col: col,
            U: rec.u1,
            V: rec.v1,
        },
        VertexTextured {
            x: centre.x + aX + bX,
            y: centre.y + aY + bY,
            z: centre.z + aZ + bZ,
            Col: col,
            U: rec.u2,
            V: rec.v1,
        },
        VertexTextured {
            x: centre.x + aX - bX,
            y: centre.y + aY - bY,
            z: centre.z + aZ - bZ,
            Col: col,
            U: rec.u2,
            V: rec.v2,
        },
    ]
}
