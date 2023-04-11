use {
	chrono::NaiveDateTime,
	serde::{de, Deserialize, Deserializer, Serialize, Serializer},
};

pub fn ser_naive_date_time<S: Serializer>(
	date: &NaiveDateTime,
	serializer: S,
) -> Result<S::Ok, S::Error> {
	let formatted = date.format("%Y-%m-%dT%H:%M:%S");
	formatted
		.to_string()
		.serialize(serializer)
}

pub fn ser_opt_naive_date_time<S: Serializer>(
	date: &Option<NaiveDateTime>,
	serializer: S,
) -> Result<S::Ok, S::Error> {
	let formatted = date.map(|date| date.format("%Y-%m-%dT%H:%M:%S"));
	formatted
		.map(|formatted| formatted.to_string())
		.serialize(serializer)
}

pub fn deser_naive_date_time<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
	D: Deserializer<'de>,
{
	let date = String::deserialize(deserializer)?;
	NaiveDateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M:%S").map_err(|_| {
		de::Error::invalid_value(
			de::Unexpected::Other(&date),
			&"Date with format `%Y-%m-%dT%H:%M:%S`",
		)
	})
}

pub fn deser_opt_naive_date_time<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
	D: Deserializer<'de>,
{
	match Option::<String>::deserialize(deserializer)? {
		None => Ok(None),
		Some(date) => NaiveDateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M:%S")
			.map(|date| Some(date))
			.map_err(|_| {
				de::Error::invalid_value(
					de::Unexpected::Other(&date),
					&"Date with format `%Y-%m-%dT%H:%M:%S`",
				)
			}),
	}
}
