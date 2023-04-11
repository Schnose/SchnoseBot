/// Utility function to format seconds to `HH:MM:SS.MS`.
pub fn format(seconds: f64) -> String {
	let hours = (seconds / 3600.0) as u8;
	let minutes = ((seconds % 3600.0) / 60.0) as u8;
	let seconds = seconds % 60.0;

	let mut formatted = format!("{minutes:02}:{seconds:06.3}");

	if hours > 0 {
		formatted = format!("{hours:02}:{formatted}");
	}

	formatted
}

#[cfg(test)]
mod tests {
	use super::format;

	#[test]
	fn zero() {
		let time = 0.0;
		let formatted = format(time);
		assert_eq!(formatted, "00:00.000");
	}

	#[test]
	fn only_seconds() {
		let time = 53.727069;
		let formatted = format(time);
		assert_eq!(formatted, "00:53.727");
	}

	#[test]
	fn minutes() {
		let time = 153.727069;
		let formatted = format(time);
		assert_eq!(formatted, "02:33.727");
	}

	#[test]
	fn hours() {
		let time = 11153.727069;
		let formatted = format(time);
		assert_eq!(formatted, "03:05:53.727");
	}
}
