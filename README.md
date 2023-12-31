# Horse Tab

[![Crates.io](https://img.shields.io/crates/v/horsetab)](https://crates.io/crates/horsetab "horsetab on Crates.io")

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

Show the help message to learn more:

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
####################
#   Example File   #
####################

SOME_VARIABLE=1
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
..-..- echo "My user is $MY_USER and variable is $SOME_VARIABLE"

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

## Windows Support

On Windows, install using Cargo, but run using [Cygwin](https://en.wikipedia.org/wiki/Cygwin) (Unix-like environment).

Currently, it seems to work properly only on Cygwin. There are several issues when running it with CMD or Powershell. No alternatives to Cygwin have been reported to work correctly.

### Kill a Process

Processes in Cygwin have both a PID and a WINPID, so the PID shown by `horsetab ps` may not work if you want to kill the process. Use `ps aux` to find the correct PID.

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
