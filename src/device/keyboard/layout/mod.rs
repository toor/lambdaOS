#[cfg(feature = "uk")]
pub mod uk_std;
#[cfg(feature = "us")]
pub mod us_std;

#[cfg(feature = "uk")]
pub use self::uk_std::map_to_upper;
#[cfg(feature = "us")]
pub use self::us_std::map_to_upper;
