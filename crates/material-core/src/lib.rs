#![warn(
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic
)]

pub use category::Category;
pub use family::{Family, MATERIALS};
pub use material::Material;
pub use modifier::Modifier;
pub use shape::Shape;

mod category;
mod family;
mod material;
mod modifier;
mod shape;
