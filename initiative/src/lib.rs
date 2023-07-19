/***************************************************************************************************
* Initiative Tracker
* Version 0.4.0 (07/14/23)
* lib.rs
* 0xMattB
***************************************************************************************************/

mod initiative;

use initiative::entity_list::EntityList;
use initiative::command::Command;
use initiative::options::Options;
use initiative::messages;
use initiative::execute;

pub fn run() {
	let mut entity_list = EntityList::new();
	let mut command = Command::new();
	let mut options = Options::new();
	
	messages::introduction();
	
	let mut running = true;
	
	while running {
		command.get();
		running = execute::run(&command, &mut entity_list, &mut options);
	}
}