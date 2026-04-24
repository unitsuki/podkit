#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Language {
	#[default]
	En,
	Es,
}

//#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)] // considerated as too much
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UserSettings {
	pub two_factor_enabled: bool, // false by default
	pub preferred_language: Language,
}
