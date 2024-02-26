#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Spectra {
    #[prost(message, repeated, tag = "1")]
    pub spectra: ::prost::alloc::vec::Vec<Spectrum>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Spectrum {
    #[prost(double, repeated, tag = "1")]
    pub mz_values: ::prost::alloc::vec::Vec<f64>,
    #[prost(double, repeated, tag = "2")]
    pub intensities: ::prost::alloc::vec::Vec<f64>,
    #[prost(message, optional, tag = "3")]
    pub precursor: ::core::option::Option<QuadrupoleEvent>,
    /// Note: Protobuf does not have a direct equivalent to Rust's usize, using uint32.
    #[prost(uint32, tag = "4")]
    pub index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Precursor {
    #[prost(double, tag = "1")]
    pub mz: f64,
    #[prost(double, tag = "2")]
    pub rt: f64,
    #[prost(double, tag = "3")]
    pub im: f64,
    /// Using uint32 as a practical equivalent to usize for charge.
    #[prost(uint32, tag = "4")]
    pub charge: u32,
    #[prost(double, tag = "5")]
    pub intensity: f64,
    #[prost(uint32, tag = "6")]
    pub index: u32,
    #[prost(uint32, tag = "7")]
    pub frame_index: u32,
    #[prost(double, tag = "8")]
    pub collision_energy: f64,
}
/// A type of quadrupole selection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuadrupoleEvent {
    #[prost(oneof = "quadrupole_event::Event", tags = "1")]
    pub event: ::core::option::Option<quadrupole_event::Event>,
}
/// Nested message and enum types in `QuadrupoleEvent`.
pub mod quadrupole_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        /// Define other events as needed.
        #[prost(message, tag = "1")]
        Precursor(super::Precursor),
    }
}
