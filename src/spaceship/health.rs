use bevy::prelude::*;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use super::STARTING_HEALTH;

#[derive(Component, Debug)]
pub struct Health(u32);

impl std::fmt::Display for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.0))
    }
}

impl Add<u32> for Health {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<u32> for Health {
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs;
    }
}

impl Sub<u32> for Health {
    type Output = Self;

    fn sub(self, rhs: u32) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl SubAssign<u32> for Health {
    fn sub_assign(&mut self, rhs: u32) {
        self.0 -= rhs;
    }
}

impl PartialEq<u32> for Health {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u32> for Health {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(other))
    }
}

impl Default for Health {
    fn default() -> Self {
        Self(STARTING_HEALTH)
    }
}
