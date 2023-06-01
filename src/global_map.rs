use gokz_rs::schnose_api::maps::index::Mapper;

use {
	crate::serde::{deser_naive_date_time, ser_naive_date_time},
	chrono::NaiveDateTime,
	fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher},
	gokz_rs::{
		global_api,
		schnose_api::{
			self,
			maps::{Course, Map},
		},
		MapIdentifier, Mode, SteamID, Tier,
	},
	serde::{Deserialize, Serialize},
};

/// Information about a global KZ map.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalMap {
	pub id: u16,
	pub name: String,
	pub tier: Tier,
	pub global: bool,
	pub courses: Vec<Course>,
	/// Whether the map's main course has a KZT filter
	pub kzt: bool,
	/// Whether the map's main course has a SKZ filter
	pub skz: bool,
	/// Whether the map's main course has a VNL filter
	pub vnl: bool,
	pub mappers: Vec<Mapper>,
	pub approver_steam_id: Option<SteamID>,
	pub filesize: u32,
	#[serde(serialize_with = "ser_naive_date_time")]
	#[serde(deserialize_with = "deser_naive_date_time")]
	pub created_on: NaiveDateTime,
	#[serde(serialize_with = "ser_naive_date_time")]
	#[serde(deserialize_with = "deser_naive_date_time")]
	pub updated_on: NaiveDateTime,
	pub workshop_link: Option<String>,
}

impl GlobalMap {
	/// A link to the map's leaderboard on KZ:GO.
	pub fn kzgo_link(&self) -> String {
		format!("https://kzgo.eu/maps/{}", self.name)
	}

	/// A link to the map's thumbnail, hosted by the Global Team on GitHub.
	pub fn thumbnail(&self) -> String {
		format!(
			"https://raw.githubusercontent.com/KZGlobalTeam/map-images/master/images/{}.jpg",
			self.name
		)
	}

	/// A link to the mapper's Steam profile.
	pub fn mapper_steamids(&self) -> impl Iterator + '_ {
		self.mappers.iter().map(|mapper| {
			format!("https://steamcommunity.com/profiles/{}", mapper.steam_id.as_id64())
		})
	}

	/// A link to the approver's Steam profile.
	pub fn approver_steam(&self) -> Option<String> {
		self.approver_steam_id
			.map(|steam_id| format!("https://steamcommunity.com/profiles/{}", steam_id.as_id64()))
	}

	pub async fn fetch(
		validated_only: bool,
		gokz_client: &gokz_rs::Client,
	) -> crate::Result<Vec<GlobalMap>> {
		let mut maps = Vec::new();

		let filters = global_api::record_filters::get_filters(
			global_api::record_filters::index::Params {
				stages: Some(0),
				tickrates: Some(128),
				limit: Some(99999),
				..Default::default()
			},
			gokz_client,
		)
		.await?;

		let fetched_maps = match validated_only {
			true => schnose_api::get_global_maps(gokz_client).await?,
			false => schnose_api::get_maps(gokz_client).await?,
		};

		let global_api_maps = match validated_only {
			true => global_api::get_global_maps(gokz_client).await?,
			false => global_api::get_maps(gokz_client).await?,
		};

		for Map {
			id,
			name,
			global,
			filesize,
			courses,
			mappers,
			approved_by,
			created_on,
			updated_on,
		} in fetched_maps
		{
			let kzt = filters
				.iter()
				.any(|filter| filter.map_id == id && filter.mode == Mode::KZTimer);
			let skz = filters
				.iter()
				.any(|filter| filter.map_id == id && filter.mode == Mode::SimpleKZ);
			let vnl = filters
				.iter()
				.any(|filter| filter.map_id == id && filter.mode == Mode::Vanilla);

			let workshop_link = global_api_maps.iter().find_map(|map| {
				if map.id == id && !map.workshop_url.is_empty() {
					Some(map.workshop_url.clone())
				} else {
					None
				}
			});

			maps.push(GlobalMap {
				id,
				name,
				tier: courses[0].tier,
				global,
				courses,
				kzt,
				skz,
				vnl,
				mappers,
				approver_steam_id: approved_by,
				filesize,
				created_on,
				updated_on,
				workshop_link,
			})
		}

		maps.sort_unstable_by(|a, b| a.name.cmp(&b.name));

		Ok(maps)
	}

	/// Fuzzy find a [`GlobalMap`] by its identifier in a collection of maps.
	/// If `map_identifier` is an id, it will search for an exact match.
	/// If `map_identifier` is a name, it will perform a fuzzy search.
	pub fn fuzzy_search(maps: &[Self], map_identifier: impl Into<MapIdentifier>) -> Option<Self> {
		match map_identifier.into() {
			MapIdentifier::ID(map_id) => maps
				.iter()
				.find_map(|map| (map.id == map_id).then_some(map.clone())),
			MapIdentifier::Name(map_name) => Self::fuzzy_match(&map_name, maps)
				.first()
				.cloned(),
		}
	}

	pub fn fuzzy_match(map_name: &str, maps: &[Self]) -> Vec<Self> {
		let fzf = SkimMatcherV2::default();
		let map_name = map_name.to_lowercase();

		let mut maps = maps
			.iter()
			.filter_map(|map| {
				if map_name.is_empty() {
					return Some((100, map.clone()));
				}

				let score = fzf.fuzzy_match(&map.name.to_lowercase(), &map_name)?;

				if score >= 50 {
					Some((score, map.clone()))
				} else {
					None
				}
			})
			.collect::<Vec<_>>();

		maps.sort_unstable_by(|(a_score, _), (b_score, _)| a_score.cmp(b_score));

		maps.into_iter()
			.map(|(_, map)| map)
			.collect()
	}
}
