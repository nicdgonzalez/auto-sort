use std::fmt;

use material_core::Material;

use crate::profile::Profile;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SortKey {
    pub category: u16,
    pub family: u16,
    pub shape: u16,
    pub modifier: u16,
}

#[must_use]
// TODO: This should be a Result.
pub fn get_sort_key(material: &Material, profile: &Profile) -> SortKey {
    let category = profile
        .category_position(material.category)
        .unwrap_or(u16::MAX);

    let family = profile
        .family_position(material.category, material.family)
        .unwrap_or(u16::MAX);

    let shape = profile
        .shape_position(material.family, material.shape)
        .unwrap_or(u16::MAX);

    let modifier = profile
        .modifier_position(material.family, material.shape, material.modifiers)
        .unwrap_or(u16::MAX);

    SortKey {
        category,
        family,
        shape,
        modifier,
    }
}

impl fmt::Display for SortKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{},{},{}",
            self.category, self.family, self.shape, self.modifier
        )
    }
}
