#![recursion_limit = "512"]
#![feature(bool_to_option)]

pub mod classes;
#[cfg(feature = "full")]
pub mod components;
#[cfg(feature = "full")]
pub mod elements;
#[cfg(feature = "full")]
pub mod forms;
#[cfg(feature = "full")]
pub mod layout;
#[cfg(feature = "full")]
pub mod toast;
#[cfg(feature = "full")]
pub mod utils;

#[cfg(feature = "full")]
pub mod prelude {
    pub use super::classes::*;
    pub use super::components::*;
    pub use super::elements::*;
    pub use super::forms::*;
    pub use super::layout::*;
    pub use super::toast::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
