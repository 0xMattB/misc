/***************************************************************************************************
* Initiative Tracker
* Version 0.4.0 (07/14/23)
* entity_list.rs
* 0xMattB
***************************************************************************************************/

mod entity;

use entity::Entity;

pub struct EntityList {
	list: Vec<Entity>,
}

impl EntityList {
	const MAX_NAME_LENGTH: usize = 24;
	
	pub fn new() -> EntityList {
		EntityList {
			list: Vec::new(),
		}
	}
	
	pub fn len(&self) -> usize {
		self.list.len()
	}
	
	pub fn add(&mut self, name: String, permanent: bool ) -> bool {
		if Self::is_name_unique(self, &name) {
			self.list.push(Entity::new(Self::truncate_name(&name), permanent));
			return true;
		}
		
		false
	}
	
	pub fn score_1_set(&mut self, name: &String, score: i32) -> bool {
		for e in &mut self.list {
			if &e.name == name {
				e.score_1_set(score);
				return true;
			}
		}
		
		false
	}
	
	pub fn score_2_set(&mut self, name: &String, score: i32) -> bool {
		for e in &mut self.list {
			if &e.name == name {
				e.score_2_set(score);
				return true;
			}
		}
		
		false
	}

	pub fn sort(&mut self) {
		self.list.sort_by_key(|n| n.score_2);
		self.list.sort_by_key(|n| n.score_1);
		self.list.reverse();
	}
	
	pub fn get_name_by_id(&self, id: usize) -> Option<&String> {
		if id < self.list.len() {
			Some(&self.list[id].name())
		} else {
			None
		}
	}
	
	pub fn get_id_by_name(&self, name: &str) -> Option<usize> {
		for i in 0..self.list.len() {
			if name == self.list[i].name {
				return Some(i);
			}
		}
		
		None
	}
	
	pub fn clear(&mut self) {
		self.list.retain(|e| e.permanent());
		
		for e in &mut self.list {
			e.clear();
		}
	}
	
	pub fn remove(&mut self, name: &String) -> bool {
		for (i, e) in self.list.iter().enumerate() {
			if e.name() == name {
				self.list.remove(i);
				return true;
			}
		}
		
		false
	}

	pub fn print(&self, id: usize, suffix: &str, show_init: bool) -> bool {
		let width_name = Self::get_longest_name_length(self) + 1;
		let width_number = 2;
		
		if self.list.len() > 0 && id < self.list.len() {
			print!("{}{:width_name$}", suffix, self.list[id].name());
			
			if show_init {
				print!("( {:width_number$} / {:width_number$} )", self.list[id].score_1(), self.list[id].score_2());
			}
			
			println!();
			return true;
		}
		
		false
	}
	
	fn is_name_unique(&self, name: &String) -> bool {
		for e in &self.list {
			if e.name() == name {
				return false;
			}
		}
		
		true
	}
	
	fn truncate_name(original: &String) -> String {
		let mut truncated_name = original.clone();
		truncated_name.truncate(Self::MAX_NAME_LENGTH);
		truncated_name
	}
	
	fn get_longest_name_length(&self) -> usize {
		let mut len = 0;
		
		for i in 0..self.list.len() {
			if self.list[i].name().len() > len {
				len = self.list[i].name.len();
			}
		}
		
		len
	}
}