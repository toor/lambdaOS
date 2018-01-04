#[cfg(feature = "uk_std")]
pub mod uk_std;
// pub mod us_std;

#[cfg(feature = "uk_std")]
pub use self::uk_std::map_to_upper;
