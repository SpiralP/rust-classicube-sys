use super::OwnedString;
use crate::bindings::*;

pub unsafe fn Entity_Init(e: &mut Entity) {
    let model = OwnedString::new("humanoid");
    e.ModelScale.set(1.0, 1.0, 1.0);
    e.uScale = 1.0;
    e.vScale = 1.0;
    e.StepSize = 0.5;
    e.SkinNameRaw[0] = 0;
    e.DisplayNameRaw[0] = 0;
    Entity_SetModel(e, model.as_cc_string());
}
