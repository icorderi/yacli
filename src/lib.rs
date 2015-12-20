
// Copyright (c) 2015 Ignacio Corderi

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

// author: Ignacio Corderi

extern crate shell;
extern crate docopt;
extern crate rustc_serialize;

pub use ::shell::MultiShell; // re-export
use ::docopt::Docopt;
use ::rustc_serialize::Decodable;
use ::std::env;
use ::std::error::Error;
use ::std::marker::PhantomData;

pub trait CliCommand<E> : Sized + Decodable {
    fn from_argv(argv: Vec<String>) -> Self {
        ::docopt::Docopt::new(Self::usage())
            .and_then(|d| d.argv(argv.clone().into_iter()).decode() )
            .unwrap_or_else(|e| e.exit())
    }

    fn execute(&self, &mut MultiShell) -> Result<(), E>;

    fn usage() -> &'static str;
}

pub trait CliDispatcher<E> {
    fn dispatch(&self, Vec<String>, &mut MultiShell) -> Result<(), E>;
}

pub trait CliArgs<E, D>
where D: CliDispatcher<E>
{
    fn get_verbose(&self) -> bool { false }

    fn get_show_list(&self) -> bool { false }

    fn get_list(&self) -> Vec<String> { vec![] }

    fn get_dispatcher<'a>(&'a self) -> &'a Option<D>;

    fn get_args<'a>(&'a self) -> &'a Vec<String>;
}

pub struct CliApplication<E, D, A>
{
    _error_type: PhantomData<E>,      // Marker
    _dispatcher_type: PhantomData<D>, // Marker
    _args_type: PhantomData<A>,       // Marker
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
                    None    => None,
                };
         CliApplication { usage: usage.into(), version: v,
                          _error_type: PhantomData,
                          _dispatcher_type: PhantomData,
                          _args_type: PhantomData }
    }

    fn main_with_args(&self, args: &[String], shell: &mut MultiShell) -> Result<(),E> {
        let docopt = Docopt::new(self.usage.clone()).unwrap()
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
            },
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