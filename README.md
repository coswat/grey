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
use grey::vars;

fn main() {
    let cmd: String = vars::get_cmd();
    // Create a new app
    let mut app: App = App::new();
    app.name("Test CLI");
    app.description("Testing this cli app");
    app.version("1.0.0");
    // Create new Commands
    let mut commands: Commands = Commands::new();
    // add a command
    commands.add("test", test_cmd , "test command");
    // run the commands
    app.run(cmd, commands);
}

fn test_cmd(_app: &App) -> u8 {
    println!("Hello from test");
    0
}
```

checkout the docs for the details, [Docs](https://docs.rs/grey/latest/grey/)

## License

The grey package is open-sourced software licensed under the [MIT license](LICENSE).