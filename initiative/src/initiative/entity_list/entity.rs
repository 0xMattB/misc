/***************************************************************************************************
* Initiative Tracker
* Version 0.4.0 (07/14/23)
* entity.rs
* 0xMattB
***************************************************************************************************/

pub struct Entity {
	pub name: String,
	pub score_1: i32,
	pub score_2: i32,
	pub permanent: bool
}

impl Entity {
	pub fn new(name: String, permanent: bool) -> Entity {
		Entity {
			name,
			score_1: 0,
			score_2: 0,
			permanent,
		}
	}
	
	pub fn name(&self) -> &String {
		&self.name
	}
	
	pub fn score_1(&self) -> i32 {
		self.score_1
	}
	
	pub fn score_2(&self) -> i32 {
		self.score_2
	}
	
	pub fn permanent(&self) -> bool {
		self.permanent
	}
	
	pub fn score_1_set(&mut self, score: i32) {
		self.score_1 = score;
	}
	
	pub fn score_2_set(&mut self, score: i32) {
		self.score_2 = score;
	}
	
	pub fn clear(&mut self) {
		self.score_1 = 0;
		self.score_2 = 0;
	}
}