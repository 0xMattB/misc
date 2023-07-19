/***************************************************************************************************
* Initiative Tracker
* Version 0.4.0 (07/14/23)
* command.rs
* 0xMattB
***************************************************************************************************/

use super::util;

#[derive(Copy, Clone)]
pub enum CommandCode {
	Null,
	Invalid,
	Add0,
	Add1,
	Set,
	Set2,
	SetAll,
	Remove,
	Clear,
	Run,
	Print,
	PrintName,
	Sort,
	Show,
	Hide,
	Help,
	Example,
	Exit,
}

pub struct Command {
	pub code: CommandCode,
	pub args: Vec<String>,
}

impl Command {
	pub fn new() -> Command {
		Command {
			code: CommandCode::Null,
			args: Vec::new(),
		}
	}
	
	pub fn get(&mut self) {
		self.clear();
		Self::prompt();
		let input_read = util::get_string();
		let input_data: Vec<&str> = input_read.split_whitespace().collect();
		Self::process(self, input_data);
	}
	
	fn process(&mut self, data: Vec<&str>) {
		let command_list = vec![
			CommandType { text: "".to_string()         , code: CommandCode::Null     , min_args: 0, max_args: 0 },
			CommandType { text: "".to_string()         , code: CommandCode::Invalid  , min_args: 0, max_args: 0 },
			CommandType { text: "add0".to_string()     , code: CommandCode::Add0     , min_args: 1, max_args: 1 },
			CommandType { text: "add1".to_string()     , code: CommandCode::Add1     , min_args: 1, max_args: 1 },
			CommandType { text: "set".to_string()      , code: CommandCode::Set      , min_args: 2, max_args: 2 },
			CommandType { text: "set2".to_string()     , code: CommandCode::Set2     , min_args: 2, max_args: 2 },
			CommandType { text: "setall".to_string()   , code: CommandCode::SetAll   , min_args: 0, max_args: 0 },
			CommandType { text: "remove".to_string()   , code: CommandCode::Remove   , min_args: 1, max_args: 1 },
			CommandType { text: "clear".to_string()    , code: CommandCode::Clear    , min_args: 0, max_args: 0 },
			CommandType { text: "run".to_string()      , code: CommandCode::Run      , min_args: 0, max_args: 0 },
			CommandType { text: "print".to_string()    , code: CommandCode::Print    , min_args: 0, max_args: 0 },
			CommandType { text: "printname".to_string(), code: CommandCode::PrintName, min_args: 1, max_args: 1 },
			CommandType { text: "sort".to_string()     , code: CommandCode::Sort     , min_args: 0, max_args: 0 },
			CommandType { text: "show".to_string()     , code: CommandCode::Show     , min_args: 0, max_args: 0 },
			CommandType { text: "hide".to_string()     , code: CommandCode::Hide     , min_args: 0, max_args: 0 },
			CommandType { text: "help".to_string()     , code: CommandCode::Help     , min_args: 0, max_args: 0 },
			CommandType { text: "example".to_string()  , code: CommandCode::Example  , min_args: 0, max_args: 0 },
			CommandType { text: "exit".to_string()     , code: CommandCode::Exit     , min_args: 0, max_args: 0 },
		];
		
		let mut valid = false;
		
		if data.len() != 0 {
			for c in &command_list {
				if data[0] == c.text {
					let args = data.len() - 1;
					
					if args >= c.min_args && args <= c.max_args {
						self.code = c.code;
						
						let mut iter = data.iter();
						iter.next();
						
						for i in iter {
							self.args.push(i.to_string());
						}
						
						valid = true;
					}
				} 
			}
			
			if !valid {
				self.code = CommandCode::Invalid;
			}
		}
	}
	
	fn clear(&mut self) {
		self.code = CommandCode::Null;
		self.args.clear();
	}
	
	fn prompt() {
		util::print_without_newline("> ");
	}
}

struct CommandType {
	text: String,
	code: CommandCode,
	min_args: usize,
	max_args: usize,
}