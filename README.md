# Grey

Grey is a minimal rust package to build console applications quickly 

## Installation

Run the following Cargo command in your project directory:

```bash 
cargo add grey
```

Or add the following line to your Cargo.toml:

```toml
grey = "0.1.0"
```

## Usage

```rust
use grey::builder::{App, Commands};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd: &str;
    if args.len() < 2 {
        cmd = "";
    } else {
        cmd = &args[1];
    }
    // Create a new app
    let mut app: App = App::new();
    app.name("Test CLI");
    app.description("Testing this cli app");
    app.version("1.0.0");
    // Create new Commands
    let mut commands: Commands = Commands::new();
    // add a command
    commands.add("coswat", || -> u8 {
       println!("Hello World");
       0
    }, "coswat admin command");
    // run the commands
    app.run(cmd, commands);
}
```

## License

The grey package is open-sourced software licensed under the [MIT license](LICENSE.md).