# Horse Tab

Trigger Linux/Unix commands via morse code using your mouse.

## Example Usage

Create a configuration file similar to crontab:

```
.-.-.--- /home/user/some_script.sh
...- /home/user/another_script.sh > /dev/null 2>&1
```

## Install Using Cargo

Install using Cargo from [crates.io](https://crates.io/crates/horsetab) using the following command.

```sh
cargo install horsetab
```

Then, verify installation.

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
