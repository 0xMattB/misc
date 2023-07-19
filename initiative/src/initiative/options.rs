/***************************************************************************************************
* Initiative Tracker
* Version 0.4.0 (07/14/23)
* options.rs
* 0xMattB
***************************************************************************************************/

pub struct Options {
	show_initiative: bool,
}

impl Options {
	pub fn new() -> Options {
		Options {
			show_initiative: true,
		}
	}
	
	pub fn show_initiative(&self) -> bool {
		self.show_initiative
	}
	
	pub fn show(&mut self, show: bool) {
		self.show_initiative = show;
	}
}