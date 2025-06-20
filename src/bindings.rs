#![allow(
    clippy::approx_constant,
    clippy::cast_lossless,
    clippy::cognitive_complexity,
    clippy::default_trait_access,
    clippy::missing_safety_doc,
    clippy::must_use_candidate,
    clippy::pub_underscore_fields,
    clippy::redundant_static_lifetimes,
    clippy::semicolon_if_nothing_returned,
    clippy::too_many_arguments,
    clippy::unreadable_literal,
    clippy::used_underscore_binding,
    clippy::useless_transmute,
    deref_nullptr,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
