# Horse Tab

Trigger Linux/Unix commands via morse code using your mouse.

## Overview

Similar to crontab, you need to configure the commands like this:

```
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

See an [example](./assets/default_config.conf).

## Troubleshooting

### Commands that hang the main process

If a command runs in the foreground for a long time, and/or keeps outputting data to stdout/stderr, new commands may not be executed even after a matching sequence.

In order to fix this issue, find the command that hangs the main process, then modify it so it redirects its output somewhere else (e.g. a file, `/dev/null`, etc), and make it run in the background:

```sh
..---.- my_command > /dev/null 2>&1 &
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
