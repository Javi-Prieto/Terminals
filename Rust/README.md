# Rust nooby Term

Inside /rTerm there is a small project done  Rust to use some terminal commands. It has two implementations uncomment one and comment the other to change between those.

## Commands
- `grep [query] [paths]`: It shows every word that is the same as the query word in the file that is on the path provided.
- `cat [path]`: shows all the content of the file on the path provided
- `echo `: This command is the most complex it has 2 options `echo [message]` shows the message in the terminal. `echo [message] > [path]` Write message into the file that is on the path provided, IT DELETES THE PREVIOUS INFORMATION.
- `exit`: This will stop the terminal execution.

## Modes

### Default
The default uncommented mode is the multi command option. You can build and run the project with `cargo run` and start executing different commands until you use the exit command.

### CLI Option
This option allows you to execute just one command per execution using the arguments provided on cargo run, first you should comment all the lines uncommented by default and uncomment the commented ones. Then you can execute `cargo run -- [command] [args]`, the commands and the arguments are the same except from exit.
