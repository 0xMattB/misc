/***************************************************************************************************
* Initiative Tracker
* Version 0.4.0 (07/14/23)
* tracker.rs
* 0xMattB
***************************************************************************************************/

use crate::initiative::entity_list::EntityList;
use super::util;

pub fn run(entity_list: &EntityList, show_initiative: bool) {
	let mut running = true;
	let mut position = 0;
	let mut round = 1;
	
	println!("\n  *** Running initiative tracker - press enter to advance, or 'x' to stop ***\n");
	println!("  Round {round}\n");
	
	while running {
		for i in 0..entity_list.len() {
			if i == position {
				entity_list.print(i, "  * ", show_initiative);
			} else {
				entity_list.print(i, "    ", show_initiative);
			}
		}
		
		util::print_without_newline("  > ");
		
		let input = util::get_string();
		
		if input.len() > 0 {
			running = false;
		}
		
		position += 1;
		
		if position >= entity_list.len() {
			position = 0;
			round += 1;
			println!("\n  Round {round}");
		}
		
		println!();
	}
	
	println!("  *** Ending initiative tracker ***");
}



