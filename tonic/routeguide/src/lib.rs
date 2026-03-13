pub mod data;

pub mod routeguide {
    tonic::include_proto!("route_guide");
}

pub use routeguide::*;
