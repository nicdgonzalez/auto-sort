use crate::category::Category;
use crate::family::Family;
use crate::modifier::Modifier;
use crate::shape::Shape;

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct MaterialId<'a>(pub &'a str);
//
// impl<'a> From<&'a str> for MaterialId<'a> {
//     fn from(value: &'a str) -> Self {
//         Self(value)
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Material {
    pub category: Category,
    pub family: Family,
    pub shape: Shape,
    pub modifiers: &'static [Modifier],
}
