# Horse Tab

Trigger Linux/Unix commands via morse code using your mouse.

## Overview

Similar to crontab, commands are configured in a file like this:

```sh
.-.-.--- /home/user/some_script.sh
...- /home/user/another_script.sh > /dev/null 2>&1
```

After saving, you are ready to start sending sequences using your mouse.

## Install Using Cargo

Install using Cargo from [crates.io](https://crates.io/crates/horsetab) using the following command:

```sh
cargo install horsetab
```

Then, verify installation:

```sh
horsetab --version
```

## How to Run

Run the main process:

```sh
horsetab serve
```

You need to manually turn it into a background process and manage stdout/stderr (e.g. redirect it to a file). You can daemonize a process this way:

```sh
nohup horsetab serve >> stdout.log 2>> stderr.log &
```

Edit commands (signals daemon to refresh the commands):

```sh
horsetab edit
```

Or show the help message to learn more:

```sh
horsetab --help
```

## Configuration File

Configuration can be done by executing:

```sh
horsetab edit
```

Here's a full example:

```sh
#!/bin/sh -c
# ^ Choose your shell (for bash/dash/sh/zsh, it must have the -c flag)
# Default is "sh -c" if shebang is removed.

SOME_VARIABLE=1
ANOTHER_VAR=2
VAR_SUBSTITUTION=$ANOTHER_VAR
MY_USER=$(whoami)

# Source a file (containing aliases, variables, etc)
. /home/user/some_file

# Define commands
# Start with a morse sequence (at least two characters)
# After the sequence, add the command to execute
# .   -->  short click
# -   -->  long click

.-.-.- some_sourced_alias
...---- another_command.sh > /dev/null 2>&1
..-..- echo "My user is $MY_USER and variable is $VAR_SUBSTITUTION"

# Note: Do NOT conditionally define commands like this:
# if CONDITION; then
#   .-.- my_command
# fi
#
# ^ BAD!
#
# Morse sequence commands are always parsed.
# This code would compile, but the morse command would be extracted out of
# the if statement anyway by the parser and made available to be triggered.
```

## Credits

Made by Chris Vilches using the Rust programming language.

```
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣦⠀⠀⠀⠀⠀⠀⠀⢠⡀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⡏⢻⣀⠀⠀⠀⠀⠀⢠⣾⢿⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣴⣿⠇⢸⣿⣿⣷⣄⠀⡰⢛⡇⢸⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣠⣤⣴⡶⠶⢋⣩⠴⢿⣿⣠⣾⠇⢹⢦⠻⣾⠁⣼⢀⡿⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣾⣿⣿⣿⠟⠉⣀⡴⠋⠀⣴⣾⠿⠛⣁⣴⡿⣼⢀⠘⢷⣿⡋⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣾⣿⢟⣿⡿⠋⠀⣠⡾⠋⠀⠀⢰⣿⣥⣶⡿⠟⢋⡴⢃⡎⡗⠀⠈⢿⠦⣄⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣤⣿⣿⠟⢡⣿⠟⠀⢠⡾⠋⠀⠀⢀⣴⢿⡟⠋⠁⠒⠚⠋⠀⣼⣻⠁⠀⠀⠈⣧⠙⢧⡀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣿⠟⠁⢰⡿⠃⠀⣰⡟⠁⠀⠀⢠⠟⠁⢸⡇⠀⠀⢤⡀⠀⠀⣏⠀⠀⠀⠀⠀⠈⠳⣤⠃
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⣿⢏⡄⢀⣿⠃⠀⢰⡿⠀⠀⠀⠀⡇⠀⠀⢀⡇⠀⣤⣤⡉⢢⡀⠻⠂⠀⠀⠀⠀⠀⢀⣾⡀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣿⢃⡞⠀⣼⡇⠀⢠⣿⠁⠀⠀⠀⠀⠃⠀⠀⠈⠀⠀⠿⣿⠿⡾⠃⠀⠀⠀⠀⠀⠀⢰⡿⠋⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⠏⣾⠀⠀⣿⠀⠀⣸⡇⠀⠀⠀⠀⠀⠀⠀⢠⡄⠀⠀⠀⠀⠀⠀⠀⠀⣀⠀⠀⠀⡆⠀⡇⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢀⡆⣾⣿⢀⡇⠀⠀⣿⡀⠀⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⣧⠀⠀⠀⠀⠀⠀⠀⠀⠈⣦⠀⠀⢧⠀⡇⠀⠀
⠀⠀⠀⠀⠀⠀⢀⡞⠁⢻⡇⠸⡇⠀⠀⠈⠃⠀⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⢻⡀⠀⠀⠀⠀⠀⠀⠀⠀⠸⡆⠀⠸⣦⠇⠀⠀
⠀⠀⠀⠀⠀⢀⣾⠀⠀⢸⣷⠀⡇⠀⠀⠀⠀⠀⣿⣇⠀⠀⠀⠀⠀⠀⠀⣰⠿⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⢣⠀⠀⢻⡀⠀⠀
⠀⠀⠀⢀⣠⢿⡟⠀⠀⠸⣿⡆⢣⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⠀⢀⣿⠀⠻⣆⡀⠀⢠⠀⠀⠀⠀⠀⢸⠀⠀⠀⢧⠀⠀
⠲⠴⠶⠟⢁⣼⡇⠀⠀⠀⢹⣿⡌⢧⠀⠀⠀⠀⢹⣿⣇⠀⠀⠀⠀⠀⢸⡇⠀⠀⠉⠛⠶⢾⡆⠀⠀⠀⠀⢸⠀⠀⠀⠸⡇⠀
⠀⢠⣤⡶⠛⢸⡇⠀⠀⠀⠀⢿⣿⣄⠀⠀⠀⠀⠘⣿⣿⡆⠀⠀⠀⠀⣼⣇⠀⠀⠀⠀⠀⠀⢳⡀⠀⠀⣠⣄⠀⠀⠀⠀⢽⣄
⠀⠀⠀⠀⠀⢸⣿⠀⠀⠀⠀⠀⢻⣿⣦⠀⠀⠀⠀⠸⣿⣿⣆⠀⠀⠀⣿⠻⣧⡀⠀⠀⠀⠀⠀⢳⠀⢰⢱⣦⠀⠀⠀⠀⢠⡿
⠀⠀⠀⠀⠀⠘⣿⡆⠀⠀⠀⠀⠀⠹⣿⣷⡄⠀⠀⠀⠹⣿⣿⣷⣄⢠⣿⠀⠈⢿⡀⠀⠀⠀⠀⠈⣇⠈⠺⣿⣷⡆⠀⢰⣿⠅
⠀⠀⠀⠀⠀⠀⠹⣧⠀⠀⠀⠀⠀⠀⠈⠻⣿⣧⡀⠀⠀⠘⢿⣿⣿⣿⣿⠀⠀⠀⢳⡀⠀⠀⠀⠀⢿⣇⠀⠈⠉⠀⠀⢨⡏⠀
⠀⠀⠀⠀⠀⠀⠀⢻⡄⠀⠀⠀⠀⠀⠀⠀⠈⠻⣿⣆⠀⠀⠀⠙⠿⣿⣿⣿⣦⡀⠈⣧⠀⠀⠀⠀⠘⢿⡦⣄⣀⣤⠴⠋⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠘⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢿⣷⣄⠀⠀⠀⣿⠟⠿⣿⣿⣦⣿⠀⠀⠀⠀⠀⠀⠉⠉⠉⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⣿⣦⡀⣼⡟⠀⠀⠈⠙⠻⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢠⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⣿⣷⡿⠁⠀⠀⠀⠀⠀⣿⠻⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢀⡞⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣿⣧⠀⠀⠀⠀⠀⢰⠇⠀⠙⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠠⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣴⡿⠋⠘⣿⡇⠀⠀⠀⠀⠞⠀⠀⠀⠸⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠀⠀⠀⠀⠘⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⢳⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⡇⠀⠀⠀⠀⠀⠀⠀⠀⠘⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
```

*But if you meet a friendly horse, will you communicate by... morse?*
