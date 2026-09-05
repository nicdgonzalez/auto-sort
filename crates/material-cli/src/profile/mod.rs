use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use heck::ToShoutySnakeCase as _;
use material_core::{Category, Family, Modifier, Shape};

pub use crate::profile::model::Precedence;
use crate::profile::model::TomlProfile;

mod model;

pub struct Profile {
    inner: TomlProfile,
}

#[derive(Debug, thiserror::Error)]
pub enum ProfileError {
    #[error("I/O error occurred")]
    Io(#[source] io::Error),

    #[error("failed to parse profile")]
    Parse(#[source] toml::de::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum GetModifierError {
    #[error("family not found")]
    FamilyNotFound,
    // #[error("shape not found")]
    // ShapeNotFound,
}

impl Profile {
    /// Reads a profile at the given `path`.
    pub fn open(path: &Path) -> Result<Self, ProfileError> {
        File::open(path)
            .map_err(ProfileError::Io)
            .and_then(Self::from_reader)
    }

    /// Reads a profile from a given `reader`.
    pub fn from_reader<R>(mut reader: R) -> Result<Self, ProfileError>
    where
        R: Read,
    {
        let mut buffer = String::new();
        reader
            .read_to_string(&mut buffer)
            .map_err(ProfileError::Io)?;

        Ok(Self {
            inner: buffer.parse().map_err(ProfileError::Parse)?,
        })
    }

    /// Returns the user's preferred category ordering.
    #[must_use]
    pub fn category(&self) -> &[Category] {
        &self.inner.order
    }

    /// Helper for returning the user's preferred position for a given [`Category`].
    #[must_use]
    pub fn category_position(&self, category: Category) -> Option<u16> {
        let index = self.category().iter().position(|c| c == &category)?;
        Some(u16::try_from(index).expect("usize overflowed u16"))
    }

    #[must_use]
    pub fn precedence(&self, category: Category) -> Option<&[Precedence]> {
        self.inner
            .category
            .get(&category)
            .map(|v| v.precedence.as_ref())
    }

    /// Returns the user's preferred family ordering for a given [`Category`].
    #[must_use]
    pub fn family(&self, category: Category) -> Option<&[Family]> {
        self.inner.category.get(&category).map(|v| v.order.as_ref())
    }

    #[must_use]
    // TODO: This should be a Result.
    pub fn family_position(&self, category: Category, family: Family) -> Option<u16> {
        let ordering = self.family(category)?;
        let index = ordering.iter().position(|f| family == *f)?;
        Some(u16::try_from(index).expect("usize overflowed u16"))
    }

    /// Returns the user's preferred shape ordering for a given [`Family`].
    #[must_use]
    pub fn shape(&self, family: Family) -> Option<&[Shape]> {
        self.inner.family.get(&family).map(|v| v.order.as_ref())
    }

    #[must_use]
    // TODO: This should be a Result.
    pub fn shape_position(&self, family: Family, shape: Shape) -> Option<u16> {
        let ordering = self.shape(family)?;
        let index = ordering.iter().position(|s| shape == *s)?;
        Some(u16::try_from(index).expect("usize overflowed u16"))
    }

    /// Returns the user's preferred modifier ordering for a given [`Family`] and [`Shape`].
    pub fn modifiers(&self, family: Family, shape: Shape) -> Result<Vec<String>, GetModifierError> {
        let family = self
            .inner
            .family
            .get(&family)
            .ok_or(GetModifierError::FamilyNotFound)?;
        let modifiers = family.shape.get(&shape).cloned().unwrap_or_default().order;
        Ok(modifiers)
    }

    #[must_use]
    // TODO: This should be a Result.
    pub fn modifier_position(
        &self,
        family: Family,
        shape: Shape,
        modifiers: &[Modifier],
    ) -> Option<u16> {
        let ordering = self
            .modifiers(family, shape)
            .expect("failed to get modifier ordering");

        let modifier_id = if modifiers.is_empty() {
            "BASE".to_owned()
        } else {
            modifiers
                .iter()
                .map(|m| m.to_string().to_shouty_snake_case())
                .collect::<Vec<_>>()
                .join("_")
        };

        let index = ordering.iter().position(|m| modifier_id == *m)?;
        Some(u16::try_from(index).expect("usize overflowed u16"))
    }
}
