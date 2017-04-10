#[cfg(test)]
#[macro_use]
extern crate rulinalg;

#[cfg(not(test))]
extern crate rulinalg;

extern crate specs;

pub mod equation;
pub mod elements;
pub mod solver;

pub type Delta = f32;
