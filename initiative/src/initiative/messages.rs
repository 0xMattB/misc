/***************************************************************************************************
* Initiative Tracker
* Version 0.4.0 (07/14/23)
* messages.rs
* 0xMattB
***************************************************************************************************/

use super::info::PROGRAM_NAME;
use super::info::PROGRAM_VERS;
use super::info::PROGRAM_DATE;

pub fn introduction() {
	println!();
	println!("{}", PROGRAM_NAME);
	println!("{} ({})", PROGRAM_VERS, PROGRAM_DATE);
	println!();
	println!("Type 'help' for a list of commands.");
	println!("Type 'example' to see an example of program usage.");
	println!();
}

pub fn help() {
	println!();
	println!("| Command    Arg1    Arg2     Description");
	println!("| -------    ----    ----     -----------");
	println!("| add0       [name]  -        Add a permanent entity to list");
	println!("| add1       [name]  -        Add a temporary entity to list");
	println!("| set        [name]  [score]  Set primary initiative score for [name]");
	println!("| set2       [name]  [score]  Set secondary initiative score for [name] (use for ties)");
	println!("| setall     -       -        Set all primary initiative scores");
	println!("| remove     [name]  -        Remove entity from list");
	println!("| clear      -       -        Remove temporary entities, reset scores for permanent entities");
	println!("| run        -       -        Run initiative tracker (list is automatically sorted)");
	println!("| print      -       -        Print all entities on list");
	println!("| printname  [name]  -        Print entity with [name]");
	println!("| sort       -       -        Sort entities in initiative order");
	println!("| show       -       -        Show initiative scores when printing lists");
	println!("| hide       -       -        Hide initiative scores when printing lists");
	println!("| help       -       -        Print this help menu");
	println!("| exit       -       -        Exit program");
	println!("|");
	println!("| Note: Names cannot contain spaces, and will be converted to lowercase.");
}

pub fn example() {
	println!("
| >>> First we create entities:
|
| > add0 william       << create a permanent entity named 'william'
| > add0 betty         << create a permanent entity named 'betty'
| > add1 goblin        << create a temporary entity named 'goblin'
|
| >>> Then we set initiative scores:
|
| > setall             << runs loop that sets primary initiative for all entities
|   william: 16        << ... william rolls a 16 
|   betty: 9           << ... betty rolls a 9
|   goblin: 16         << ... goblin rolls a 16
|                      << uh oh, william and goblin tied!
|                      << they each roll again, and we enter these as secondary scores
| > set2 william 12    << ... william rolls a 12
| > set2 goblin 14     << ... goblin rolls a 14
|
| >>> List tasks (optional):
|
| > sort              << we can sort the list, but this not required to be done manually
| > print             << we can also print the list to the screen
|   goblin  ( 16 / 14 )
|   william ( 16 / 12 )
|   betty   (  9 /  0 )
|
| >>> We are now ready to run initiative
|
| > run                   << this sorts the entities in initiative order and starts the tracker
| (*** initiative tracker not shown here ***)
|
| >>> Cleaning up the list:
|
| > clear                << this command: (1) removes any temporary entities from the list ...
|                        << ... and (2) resets all initiative scores back to zero
| > print                << we can print the list to verify this
|   william (  0 /  0 )
|   betty   (  0 /  0 )
|
| > exit                 << when we are done, we can exit the program");
}