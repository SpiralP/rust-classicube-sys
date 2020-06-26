use std::os::raw::c_int;

pub fn Math_NextPowOf2(value: c_int) -> c_int {
    let mut next = 1;
    while value > next {
        next <<= 1;
    }
    next
}

pub fn Math_IsPowOf2(value: c_int) -> bool {
    value != 0 && (value & (value - 1)) == 0
}
