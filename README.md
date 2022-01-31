# `cosh` Comfortably Shell
> A *minimal* and *simple* shell written in Rust.

## Commands & Syntax
`cosh`'s syntax is a mix between two shells: Windows' `cmd.exe` and the classic UNIX `sh`.

The hash character (`#`) will be used as comments - who likes typing three more characters (`@REM`) ?

Control statements have not been implemented yet, but for now we just have the following *built-in* commands:
- `ls [dir]` - Lists a given directory, or by default the current working directory. No command-line flags are supported.
- `pwd` - Prints the current working directory, which is already displayed in the prompt.
- `history` - Displays the current command history. This persists through system power management. Typing `history clear` will clear the current history.
- `cd <dir>` - Changes directory.
- `help` - Displays help command
- `cls` - Clears the current screen. `Ctrl + L` functions the same way.

`cosh` handles these interrupts in a way denoted below:
- `Ctrl + L` - Clear the screen, as mentioned above.
- `Ctrl + C` & `Ctrl + D` - Do **literally nothing**. Who uses these in shells?

*note: `[...]` denote optional parameters, while `<...>` denote required parameters.*

## To-do
- Replace `$XXX` with its respective environment variable