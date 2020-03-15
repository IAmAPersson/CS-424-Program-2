/*
	Phil Lane
	03/14/2020
	CS-424-02
	Mary Allen

	Programming Assignment 2 (Rust)
*/

//Type: Inputted batter information
pub struct BatterInfo {
	pub firstName: String,
	pub lastName: String,
	pub plateAppearances: u64,
	pub atBats: u64,
	pub singles: u64,
	pub doubles: u64,
	pub triples: u64,
	pub homeRuns: u64,
	pub walks: u64,
	pub hitByPitch: u64,
}

//Type: Calculated batter information
pub struct CalculatedBatterInfo {
	pub firstName: String,
	pub lastName: String,
	pub average: f64,
	pub slugging: f64,
	pub onBase: f64,
}
