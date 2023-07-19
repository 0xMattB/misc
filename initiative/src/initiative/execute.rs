/***************************************************************************************************
* Initiative Tracker
* Version 0.4.0 (07/14/23)
* execute.rs
* 0xMattB
***************************************************************************************************/

mod tracker;

use super::command::{Command, CommandCode};
use super::entity_list::EntityList;
use super::options::Options;
use super::util;
use super::messages;

pub fn run(c: &Command, entity_list: &mut EntityList, options: &mut Options) -> bool {
	let response = match c.code {
		CommandCode::Null      => { execute_null()                             },
		CommandCode::Invalid   => { execute_invalid()                          },
		CommandCode::Add0      => { execute_add0(c, entity_list)               },
		CommandCode::Add1      => { execute_add1(c, entity_list)               },
		CommandCode::Set       => { execute_set(c, entity_list)                },
		CommandCode::Set2      => { execute_set2(c, entity_list)               },
		CommandCode::SetAll    => { execute_setall(entity_list)                },
		CommandCode::Remove    => { execute_remove(c, entity_list)             },
		CommandCode::Clear     => { execute_clear(entity_list)                 },
		CommandCode::Run       => { execute_run(entity_list, options)          },
		CommandCode::Print     => { execute_print(entity_list, options)        },
		CommandCode::PrintName => { execute_printname(c, entity_list, options) },
		CommandCode::Sort      => { execute_sort(entity_list)                  },
		CommandCode::Show      => { execute_show(options)                      },
		CommandCode::Hide      => { execute_hide(options)                      },
		CommandCode::Help      => { execute_help()                             },
		CommandCode::Example   => { execute_example()                          },
		CommandCode::Exit      => { return false;                              },
	};
	
	println!("{}", response);
	
	if response.len() > 0 {
		println!();
	}
	
	true
}

fn execute_null() -> &'static str {
	""
}

fn execute_invalid() -> &'static str {
	"! invalid command (check spelling, or confirm arguments)"
}

fn execute_add0(c: &Command, entity_list: &mut EntityList) -> &'static str {
	if entity_list.add(c.args[0].clone(), true) {
		"* entity added to list"
	} else {
		"! entity not added, name already in use"
	}
}

fn execute_add1(c: &Command, entity_list: &mut EntityList) -> &'static str {
	if entity_list.add(c.args[0].clone(), false) {
		"* entity added to list"
	} else {
		"! entity not added, name already in use"
	}
}

fn execute_set(c: &Command, entity_list: &mut EntityList) -> &'static str {
	let mut value = 0;

	if !util::get_i32_from_string(&c.args[1], &mut value) {
		return "! invalid score argument";
	}
	
	if !entity_list.score_1_set(&c.args[0], value) {
		return "! name not found";
	}
	
	"* score added"
}

fn execute_set2(c: &Command, entity_list: &mut EntityList) -> &'static str {
	let mut value = 0;

	if !util::get_i32_from_string(&c.args[1], &mut value) {
		return "! invalid score argument";
	}
	
	if !entity_list.score_2_set(&c.args[0], value) {
		return "! name not found";
	}
	
	"* score added"
}

fn execute_setall(entity_list: &mut EntityList) -> &'static str {
	if entity_list.len() == 0 {
		return "! entity list is empty";
	} else {
		println!();
		
		for id in 0..entity_list.len() {
			let name = entity_list.get_name_by_id(id).unwrap().clone();
			
			let s = format!["  {}: ", name];
			util::print_without_newline(&s);
			
			let mut value = 0;
			
			while !util::get_i32(&mut value) {
				util::print_without_newline(&s);
			}
			
			entity_list.score_1_set(&name, value);
		}
	}
	
	""
}

fn execute_remove(c: &Command, entity_list: &mut EntityList) -> &'static str {
	if entity_list.remove(&c.args[0]) {
		"* entity removed from list"
	} else {
		"! entity not removed, name not found"
	}
}

fn execute_clear(entity_list: &mut EntityList) -> &'static str {
	entity_list.clear();
	"* entity list has been cleared"
}

fn execute_run(entity_list: &mut EntityList, options: &Options) -> &'static str {
	if entity_list.len() == 0 {
		"! entity list is empty"
	} else {
		entity_list.sort();
		tracker::run(entity_list, options.show_initiative());
		""
	}
}

fn execute_print(entity_list: &EntityList, options: &Options) -> &'static str {
	if entity_list.len() == 0 {
		return "! entity list is empty";
	} else {
		println!();
		
		for id in 0..entity_list.len() {
			entity_list.print(id, "  ", options.show_initiative());
		}
	}
	
	""
}

fn execute_printname(c: &Command, entity_list: &EntityList, options: &Options) -> &'static str {
	if entity_list.len() == 0 {
		return "! entity list is empty";
	} else {
		match entity_list.get_id_by_name(&c.args[0]) {
			Some(id) => {
				println!();
				entity_list.print(id, "  ", options.show_initiative());
			},
			None => {
				return "! name not found on list";
			},
		}
	}
	
	""
}

fn execute_sort(entity_list: &mut EntityList) -> &'static str {
	entity_list.sort();
	"* entity list sorted"
}

fn execute_show(options: &mut Options) -> &'static str {
	options.show(true);
	"* initiative values will be shown"
}

fn execute_hide(options: &mut Options) -> &'static str {
	options.show(false);
	"* initiative values will be hidden"
}

fn execute_help() -> &'static str {
	messages::help();
	""
}

fn execute_example() -> &'static str  {
	messages::example();
	""
}