//! # Builder
//!
//! CLI app builder

use std::collections::HashMap;
use std::process;

struct CommandInfo {
    handler: fn() -> u8,
    desc: String,
}

/// Commands struct which holds, command, command's description and its callback function
pub struct Commands {
    /// holds command name, description, callback
    values: HashMap<String, CommandInfo>,
}

#[derive(Debug)]
/// App struct which hold name, description and version of the app
pub struct App<'a> {
    /// name of the app
    pub name: &'a str,
    /// description of the app
    pub desc: &'a str,
    /// version of the app
    pub version: &'a str,
}

impl Commands {
    /// Create a empty Commands struct
    ///
    /// # Examples
    ///
    /// ```
    /// use grey::builder::Commands;
    ///
    /// let mut commands: Commands = Commands::new();
    /// ```
    pub fn new() -> Self {
        let command: Commands = Commands {
            values: HashMap::new(),
        };
        command
    }
    /// Add a command
    ///
    /// # Arguments
    ///
    /// * `cmd` - name of the command ( string slice )
    /// * `handler` - action ( callback function )
    /// * `desc` - description of the command ( string slice )
    ///
    /// # Examples
    ///
    /// ```
    /// use grey::builder::Commands;
    ///
    /// let mut commands: Commands = Commands::new();
    /// // add a command
    /// commands.add("test", || -> u8 {
    ///  println!("Hello World");
    ///  0
    /// }, "test command");
    /// ```
    pub fn add(&mut self, cmd: &str, handler: fn() -> u8, desc: &str) {
        let command_info: CommandInfo = CommandInfo {
            handler,
            desc: desc.to_string(),
        };
        self.values.insert(cmd.to_string(), command_info);
    }
}

impl<'a> App<'a> {
    /// Create a empty App struct
    ///
    /// # Examples
    ///
    /// ```
    /// use grey::builder::App;
    ///
    /// let mut app: App = App::new();
    /// ```
    pub fn new() -> Self {
        App {
            name: "",
            desc: "",
            version: "",
        }
    }
    /// Providing CLI app name
    ///
    /// # Arguments
    ///
    /// * `name` - name of the cli app ( a string slice )
    ///
    /// # Examples
    ///
    /// ```
    /// use grey::builder::App;
    ///
    /// let mut app: App = App::new();
    /// // add the name
    /// app.name("test name");
    /// ```
    pub fn name(&mut self, name: &'a str) {
        self.name = name;
    }
    /// Providing CLI app description
    ///
    /// # Arguments
    ///
    /// * `desc` - description of the cli app ( a string slice )
    ///
    /// # Examples
    ///
    /// ```
    /// use grey::builder::App;
    ///
    /// let mut app: App = App::new();
    /// app.name("test name");
    /// // add the description
    /// app.description("This is a test CLI app");
    /// ```
    pub fn description(&mut self, desc: &'a str) {
        self.desc = desc;
    }
    /// Providing CLI app version
    ///
    /// # Arguments
    ///
    /// * `version` - version of the cli app ( a string slice )
    ///
    /// # Examples
    ///
    /// ```
    /// use grey::builder::App;
    ///
    /// let mut app: App = App::new();
    /// app.name("test name");
    /// app.description("This is a test CLI app");
    /// // add version
    /// app.version("1.0.0");
    /// ```
    pub fn version(&mut self, version: &'a str) {
        self.version = version;
    }
    /// Run the app
    ///
    /// # Arguments
    ///
    /// * `cmd` - provide the command to run ( a string slice )
    /// * `commands` - provide the Commands struct ( Commands struct )
    ///
    /// # Examples
    ///
    /// ```
    /// use grey::builder::{App,Commands};
    ///
    /// let mut app: App = App::new();
    /// app.name("test name");
    /// app.description("This is a test CLI app");
    /// app.version("1.0.0");
    ///
    /// let mut commands: Commands = Commands::new();
    /// commands.add("test", || {
    ///     println!("hello");
    ///     0
    /// }, "test command");
    ///
    /// // Run the commands
    /// app.run("test", commands);
    /// ```
    pub fn run(&self, cmd: &'a str, commands: Commands) {
        if cmd.is_empty() {
            self.show_defaults(commands);
            process::exit(1);
        }
        match commands.values.get(cmd) {
            Some(cmd_info) => {
                let code: i32 = (cmd_info.handler)() as i32;
                process::exit(code);
            }
            None => {
                eprintln!("error: unknown command '{}'", cmd);
                process::exit(1);
            }
        }
    }
    // function to show the default commands and app contents
    fn show_defaults(&self, command: Commands) {
        println!(
            "\n{} {}\n\nUsage:\n command [options] [arguments]\n\nCommands:",
            self.name, self.version
        );

        for (cmd, cmd_info) in command.values {
            println!(" {:<15}{}", cmd, cmd_info.desc);
        }
    }
}
