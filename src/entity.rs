use super::OwnedString;
use crate::bindings::*;

#[allow(clippy::missing_safety_doc)]
pub unsafe fn Entity_Init(e: &mut Entity) {
    let model = OwnedString::new("humanoid");
    e.ModelScale.set(1.0, 1.0, 1.0);
    e.uScale = 1.0;
    e.vScale = 1.0;
    e._skinReqID = 0;
    e.SkinRaw[0] = 0;
    e.NameRaw[0] = 0;
    unsafe {
        Entity_SetModel(e, model.as_cc_string());
    }
}
