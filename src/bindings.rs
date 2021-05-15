#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    deref_nullptr
)]
#![allow(
    clippy::missing_safety_doc,
    clippy::unreadable_literal,
    clippy::cognitive_complexity,
    clippy::redundant_static_lifetimes,
    clippy::approx_constant,
    clippy::too_many_arguments,
    clippy::useless_transmute
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
