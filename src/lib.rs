extern crate shell;
extern crate docopt;
extern crate rustc_serialize;

pub use ::shell::MultiShell; // re-export

use ::std::env;
use ::std::error::Error;
use ::std::marker::PhantomData;

use ::docopt::Docopt;
use ::rustc_serialize::Decodable;

/// Represents CLI commands
pub trait CliCommand<E>: Decodable {
    fn from_argv(argv: Vec<String>) -> Self {
        ::docopt::Docopt::new(Self::usage())
            .and_then(|d| d.argv(argv.clone().into_iter()).decode())
            .unwrap_or_else(|e| e.exit())
    }

    fn execute(&self, &mut MultiShell) -> Result<(), E>;

    fn usage() -> &'static str;
}

/// Represents a CLI command dispatcher
pub trait CliDispatcher<E> {
    fn dispatch(&self, Vec<String>, &mut MultiShell) -> Result<(), E>;
}

/// Represents a cli args
pub trait CliArgs<E, D>: Decodable
    where D: CliDispatcher<E>
{
    fn usage() -> &'static str;

    fn get_verbose(&self) -> bool {
        false
    }

    fn get_show_list(&self) -> bool {
        false
    }

    fn get_list(&self) -> Vec<String> {
        vec![]
    }

    fn get_dispatcher<'a>(&'a self) -> &'a Option<D>;

    fn get_args<'a>(&'a self) -> &'a Vec<String>;

    fn try_dispatch(&self, shell: &mut MultiShell) -> Result<(), E> {
        match *self.get_dispatcher() {
            Some(ref cmd) => try!(cmd.dispatch(self.get_args().clone(), shell)),
            None => println!("{}", Self::usage()),
        };
        Ok(())
    }
}

#[derive(Debug, RustcDecodable)]
pub struct GenericArgs<C> {
    pub arg_command: Option<C>,
    pub arg_args: Vec<String>,
    pub flag_list: bool,
    pub flag_verbose: bool,
}

impl<C: Decodable> GenericArgs<C> {
    pub fn from_argv(usage: &str, argv: Vec<String>) -> Self {
        let docopt = Docopt::new(usage)
            .unwrap()
            .options_first(true)
            .argv(argv.iter().map(|s| &s[..]))
            .help(true);

        docopt.decode().unwrap_or_else(|e| e.exit())
    }
}

pub struct CliApplication<E, D, A> {
    _error_type: PhantomData<E>, // Marker
    _dispatcher_type: PhantomData<D>, // Marker
    _args_type: PhantomData<A>, // Marker
    usage: String,
    version: Option<String>,
}

impl<E, D, A> CliApplication<E, D, A>
    where E: Error,
          D: CliDispatcher<E>,
          A: CliArgs<E, D> + Decodable
{
    pub fn new<U: Into<String>, V: Into<String>>(usage: U, version: Option<V>) -> Self {
        let v = match version {
            Some(v) => Some(v.into()),
            None => None,
        };
        CliApplication {
            usage: usage.into(),
            version: v,
            _error_type: PhantomData,
            _dispatcher_type: PhantomData,
            _args_type: PhantomData,
        }
    }

    fn main_with_args(&self, args: &[String], shell: &mut MultiShell) -> Result<(), E> {
        let docopt = Docopt::new(self.usage.clone())
            .unwrap()
            .options_first(true)
            .argv(args.iter().map(|s| &s[..]))
            .help(true)
            .version(self.version.clone());

        let args: A = docopt.decode().unwrap_or_else(|e| e.exit());
        shell.set_verbose(args.get_verbose());

        if args.get_show_list() {
            println!("Installed Commands:");
            for x in args.get_list() {
                println!("    {}", x);
            }
            return Ok(());
        }

        match *args.get_dispatcher() {
            Some(ref cmd) => cmd.dispatch(args.get_args().clone(), shell),
            None => {
                println!("{}", &self.usage);
                Ok(())
            }
        }
    }

    pub fn main(&self) {
        let mut shell = MultiShell::new_stdio(false);
        let args: Vec<_> = env::args().collect();
        let r = self.main_with_args(args.as_ref(), &mut shell);
        match r {
            Ok(_) => (),
            Err(e) => shell.error_full(&e, true).unwrap(),
        }
    }
}

pub fn main<C: CliCommand<E>, E: Error>() {
    let mut shell = MultiShell::new_stdio(false);
    let args: Vec<_> = env::args().collect();
    let cmd = C::from_argv(args);
    let r = cmd.execute(&mut shell);
    match r {
        Ok(_) => (),
        Err(e) => shell.error_full(&e, true).unwrap(),
    }
}
