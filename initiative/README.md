"Initiative"

This is a Rust program that performs initiative tracking for table-top role playing games.
It is a simple practice project written to supplement the learning process.

The command-line interface allows commands and data to be entered.
Enter 'help' for a list of all commands and their descriptions.
Enter 'example' to print a sample session.

The user can add "permanent" entities with the "add0 [name]" command.
The user can add "temporary" entities with the "add1 [name]" command.
Typically, the PCs are added with "add0" while NPCs/foes are added with "add1".
When a "clear" command is entered, all temporary entities are deleted, and the initiative scores for
all permanent entities are reset to zero.

Initiative scores can be entered with the "set [name] [score]" command or the "set" command (the latter
loops through all entities and prompts for their initiative score).

There is also a "set2 [name] [score]" command that allows the user to enter a secondary initiative for
a given entity. This is for when there is a tie and secondary initiave rolls are made.

The "print" command prints a list of all entities.
The "sort" command sorts all entities in descending order.
The "hide" command hides the initiative scores when printing the entity list.
The "show" command shows the initiative scores when printing the entity list (on by default).

When the "run" command is entered, the program enters run mode. This sorts then prints the entity
list, indicating the turn. Pressing enter advances the turn (and keeps track of rounds). Entering
any character terminates the run mode.