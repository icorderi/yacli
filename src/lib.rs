extern crate shell;
extern crate docopt;
extern crate rustc_serialize;
pub extern crate pad; // re-export

pub use ::shell::MultiShell; // re-export

use ::std::error::Error;

use ::docopt::Docopt;
use ::rustc_serialize::Decodable;

/// Represents a CLI command
pub trait Command {
    type Error: Error;

    /// Name of the command as used in the docopt string
    #[inline]
    fn name() -> Option<&'static str> {
        None
    }

    /// Docopt usage string
    #[inline]
    fn usage() -> &'static str;

    /// Run the command
    fn execute(&self, shell: &mut MultiShell) -> Result<(), Self::Error>;
}

pub trait Decode {
    fn decode(args: Vec<String>) -> Self;
    fn decode_with_version<S: Into<String>>(args: Vec<String>, version: S) -> Self;
}

impl<T: Command + Decodable> Decode for T {
    fn decode(args: Vec<String>) -> Self {
        let docopt = Docopt::new(T::usage())
            .unwrap()
            .options_first(true)
            .argv(args)
            .help(true);
        docopt.decode().unwrap_or_else(|e| e.exit())
    }

    fn decode_with_version<S: Into<String>>(args: Vec<String>, version: S) -> Self {
        let docopt = Docopt::new(T::usage())
            .unwrap()
            .options_first(true)
            .argv(args)
            .help(true)
            .version(Some(version.into()));
        docopt.decode().unwrap_or_else(|e| e.exit())
    }
}

#[inline]
pub fn main_with_command<C: Command>(cmd: C) {
    let mut shell = MultiShell::new_stdio(false);
    let r = cmd.execute(&mut shell);
    match r {
        Ok(_) => (),
        Err(e) => shell.error_full(&e, true).unwrap(),
    }
}

#[macro_export]
macro_rules! route {
    (   name  = $name:expr;
        error = $error:path;
        usage = $usage:expr;
        match $cmd:path {
            $( $left:pat => $right:path, )*
        }
    ) => {
        impl ::yacli::Command for $cmd {
            type Error = $error;

            fn name() ->  Option<&'static str> { Some($name) }

            fn usage() -> &'static str { $usage }

            fn execute(&self, shell: &mut MultiShell) -> Result<(), Self::Error> {
                if self.get_help() {
                    // Print the help
                    if let &Some(ref cmd) = self.get_command() {
                        match *cmd {
                            $( $left => {
                                println!("{}", <$right as ::yacli::Command>::usage().trim());
                            }),*
                        };
                    } else {
                        println!("{}", Self::usage().trim());
                        ::std::process::exit(-1);
                    }
                } else if self.get_list() {
                    let commands = &[ $( (<$right as ::yacli::Command>::name().unwrap().trim(),
                                          <$right as ::yacli::Command>::usage().trim()
                                                                               .lines()
                                                                               .next()
                                                                               .unwrap()) ),* ];

                    let title = "Command";
                    let mut max_cmd = commands.iter().map(|&(ref x,_)| x.len()).max().unwrap();
                    // Make sure it is at least as long as the title "Command"
                    if max_cmd < title.len() {
                        max_cmd = title.len();
                    }
                    let max_help = commands.iter().map(|&(_,ref y)| y.len()).max().unwrap();

                    println!("{} | {}", ::yacli::pad::PadStr::pad_to_width(title, max_cmd), "Help");
                    println!("{}", String::from_utf8(vec![b'-'; max_cmd + 3 + max_help]).unwrap());
                    for &(ref name, ref usage) in commands {
                        println!("{} | {}", ::yacli::pad::PadStr::pad_to_width(*name, max_cmd), usage);
                    }
                    println!("");
                    println!("Use `help <command>` for more information on a specific command.");
                    ::std::process::exit(-1);
                } else {
                    // Execute the command
                    if let &Some(ref cmd) = self.get_command() {
                        match *cmd {
                            $( $left => {
                                let mut args = self.get_args().clone();
                                if let Some(n) = <$right as ::yacli::Command>::name() {
                                    args.insert(0, n.to_string());
                                }
                                let x = <$right as ::yacli::Decode>::decode(args);
                                try!(x.execute(shell));
                            }),*
                        };
                    } else {
                        println!("{}", Self::usage().trim());
                    }
                }
                Ok(())
            }
        }
    };
}

#[macro_export]
macro_rules! main {
    ($t:path) => {
        {
            let args: Vec<_> = ::std::env::args().collect();
            let cmd = <$t as ::yacli::Decode>::decode(args);
            ::yacli::main_with_command(cmd);
        }
    };
    ($t:path, $version:expr) => {
        {
            let args: Vec<_> = ::std::env::args().collect();
            let cmd = <$t as ::yacli::Decode>::decode_with_version(args, $version);
            ::yacli::main_with_command(cmd);
        }
    };
}

#[macro_export]
macro_rules! generic_args {
    ($id:ident, $t:ty) => {
        #[derive(Debug, RustcDecodable)]
        pub struct $id {
            cmd_help: Option<bool>,
            arg_command: Option<$t>,
            arg_args: Vec<String>,
            flag_list: Option<bool>,
            flag_verbose: Option<bool>,
        }

        #[allow(dead_code)]
        impl $id {
            pub fn get_args(&self) -> &Vec<String> {
                &self.arg_args
            }

            pub fn get_command(&self) -> &Option<$t> {
                &self.arg_command
            }

            pub fn get_help(&self) -> bool {
                self.cmd_help.unwrap_or(false)
            }

            pub fn get_verbose(&self) -> bool {
                self.flag_verbose.unwrap_or(false)
            }

            pub fn get_list(&self) -> bool {
                self.flag_list.unwrap_or(false)
            }
        }
    }
}
