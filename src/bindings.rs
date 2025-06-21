#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    deref_nullptr
)]
#![allow(
    clippy::approx_constant,
    clippy::cognitive_complexity,
    clippy::missing_safety_doc,
    clippy::ptr_offset_with_cast,
    clippy::redundant_static_lifetimes,
    clippy::too_many_arguments,
    clippy::unreadable_literal,
    clippy::useless_transmute
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
