#![feature(if_let, phase)]

extern crate getopts;
#[phase(plugin, link)]
extern crate log;
extern crate umake;

use getopts::{optflag, usage};
use umake::Makefile;
use std::collections::HashMap;
use std::os;

const NAME: &'static str = "umake";
const VERSION: &'static str = "0.0.1";

fn main() {
   let args = os::args_as_bytes();
   let args: Vec<&[u8]> = args.iter().map(|x| x.as_slice()).collect();
   let args = args.as_slice();
   
   if let Some(options) = parse_args(args) {
      let mut makefile = umake::GnuMakefile::new();  // TODO: check for BSD-style file
      let mut file = std::io::File::open(&Path::new(args[1])).unwrap();  // FIXME: check for errors
      let tokens = match makefile.tokenize(file.read_to_end().unwrap().as_slice()) {  // FIXME: check for errors
         Ok(tokens) => tokens,
         Err(f) => {
            println!("{}", f);
            println!("{}", f.code);
            return;
         }
      };
      match makefile.parse(tokens) {
         Ok(_) => { /* ignore dag for now */ }
         Err(f) => {
            println!("{}", f);
            println!("{}", f.code);
         }
      }
   }
}

struct CmdOptions<'a> {
   vars: HashMap<&'a [u8], &'a [u8]>,
   other: Vec<&'a [u8]>
}

fn parse_args<'a>(args: &[&'a [u8]]) -> Option<CmdOptions<'a>> {
   let mut options = CmdOptions {
      vars: HashMap::new(),
      other: vec!()
   };
   for &arg in args.iter() {
      match arg {
         b"-h" | b"--help" => {
            print_help();
            return None;
         }
         b"-V" | b"--version" => {
            print_version();
            return None;
         }
         _ => {
            if arg.len() > 0 && arg[0] == b'-' {
               error!("Unrecognized option (\"{}\").  Try \"{} --help\" for help", String::from_utf8_lossy(arg), NAME);
               return None;
            } else {
               options.other.push(arg);
            }
         }
      }
   }
   Some(options)
}

fn print_help() {
   let opts = [
      optflag("h", "help", "print this help menu"),
      optflag("V", "version", "print the version of this program")
   ];
   
   print!("{}", usage("An implementation of make supporting both GNU and BSD syntaxes.", opts));
}

fn print_version() {
   println!("{} v{}", NAME, VERSION);
}
