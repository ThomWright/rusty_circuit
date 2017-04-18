
pub mod equation;
mod stamp_static;

pub mod solve;
pub use self::stamp_static::create_static_equation;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod test_static_circuits;
