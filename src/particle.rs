use crate::bindings::*;

pub fn Particle_DoRender(
    size: &Vec2,
    pos: &Vec3,
    rec: &TextureRec,
    col: PackedCol,
) -> [VertexTextured; 4] {
    let sX = size.X * 0.5;
    let sY = size.Y * 0.5;
    let mut centre = *pos;
    centre.Y += sY;
    let view = unsafe { &Gfx.View };

    let aX = view.Row0.X * sX;
    let aY = view.Row1.X * sX;
    let aZ = view.Row2.X * sX; // right * size.X * 0.5f
    let bX = view.Row0.Y * sY;
    let bY = view.Row1.Y * sY;
    let bZ = view.Row2.Y * sY; // up    * size.Y * 0.5f

    [
        VertexTextured {
            X: centre.X - aX - bX,
            Y: centre.Y - aY - bY,
            Z: centre.Z - aZ - bZ,
            Col: col,
            U: rec.U1,
            V: rec.V2,
        },
        VertexTextured {
            X: centre.X - aX + bX,
            Y: centre.Y - aY + bY,
            Z: centre.Z - aZ + bZ,
            Col: col,
            U: rec.U1,
            V: rec.V1,
        },
        VertexTextured {
            X: centre.X + aX + bX,
            Y: centre.Y + aY + bY,
            Z: centre.Z + aZ + bZ,
            Col: col,
            U: rec.U2,
            V: rec.V1,
        },
        VertexTextured {
            X: centre.X + aX - bX,
            Y: centre.Y + aY - bY,
            Z: centre.Z + aZ - bZ,
            Col: col,
            U: rec.U2,
            V: rec.V2,
        },
    ]
}
