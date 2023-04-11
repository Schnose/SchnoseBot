//! Library used by other repositories in the [Schnose](https://github.com/Schnose) organization.

/// Module to fetch global maps from the `GlobalAPI` and `SchnoseAPI`
pub mod global_map;

/// Utility functions to extend `serde` (like parsing dates)
pub mod serde;

/// Utility function to format seconds to `HH:MM:SS.MS`.
pub mod time;

pub type Result<T> = std::result::Result<T, Error>;

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
	/// An error occured when talking to the `GlobalAPI`.
	#[error("GlobalAPI request failed: {message}")]
	GlobalAPI { message: String, error: gokz_rs::Error },
}

impl From<gokz_rs::Error> for Error {
	fn from(error: gokz_rs::Error) -> Error {
		Error::GlobalAPI {
			message: error.to_string(),
			error,
		}
	}
}
