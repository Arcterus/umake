#![feature(globs)]

use std::collections::HashMap;
use std::fmt::{Show, FormatError, Formatter};
use std::os;
use std::str;

pub use self::gnu::GnuMakefile;

// XXX: no idea why this is necessary
#[path = "gnu/mod.rs"]
pub mod gnu;

pub type TokenResult<T> = Result<T, TokenError>;
pub type ParseResult<T> = Result<T, ParseError>;

pub trait Makefile<'a> {
   fn vars(&mut self) -> &mut MutableMap<&'a [u8], Vec<u8>>;
   
   fn tokenize<'a>(&mut self, data: &[u8]) -> TokenResult<Vec<Token<'a>>>;
   fn parse<'a, 'b>(&mut self, tokens: Vec<Token<'b>>) -> ParseResult<MakefileDag<'a>>;

   fn find_var<'b>(&'b mut self, name: &'a [u8]) -> Option<Vec<u8>> {
      let vars = self.vars();
      match vars.find(&name).and_then(|val| Some(val.clone())) {  // get the borrow checker to shut up
         None => {
            if let Some(utf8_name) = str::from_utf8(name) {
               match os::getenv_as_bytes(utf8_name) {
                  Some(val) => {
                     vars.insert(name, val.clone());
                     Some(val)
                  }
                  None => None
               }
            } else {
               None
            }
         }
         val => val
      }
   }
}

pub enum Token<'a> {
   Ident(&'a [u8]),
   Var(&'a [u8]),       // NOTE: both Var and FuncCall have the same syntax
   FuncCall(&'a [u8]),
   Command(&'a Token<'a>),  // i.e. shell commands in a rule (Token should be Other)
   Colon,
   Equal,
   Comma,
   Dollar,
   Question,
   Plus,
   Tab,
   IfEq,
   IfNeq,
   Other(&'a [u8])
}

pub struct MakefileDag<'a> {
   rules: HashMap<&'a [u8], MakefileRule<'a>>,
   funcs: HashMap<&'a [u8], &'a [u8]>
}

pub struct MakefileRule<'a> {
   name: &'a [u8],
   deps: Vec<&'a MakefileRule<'a>>,
   body: &'a [u8]
}

pub struct ParseError {
   pub message: String,
   pub code: int
}

pub struct TokenError {
   pub message: String,
   pub code: int
}

impl<'a> MakefileDag<'a> {
   #[inline]
   pub fn new() -> MakefileDag<'a> {
      MakefileDag {
         rules: HashMap::new(),
         funcs: HashMap::new(),
      }
   }
}

impl<'a> MakefileRule<'a> {
   #[inline]
   pub fn new(name: &'a [u8], deps: Vec<&'a MakefileRule<'a>>, body: &'a [u8]) -> MakefileRule<'a> {
      MakefileRule {
         name: name,
         deps: deps,
         body: body
      }
   }
}

impl ParseError {
   #[inline]
   pub fn new(msg: String, code: int) -> ParseError {
      ParseError {
         message: msg,
         code: code
      }
   }
}

impl Show for ParseError {
   fn fmt(&self, formatter: &mut Formatter) -> Result<(), FormatError> {
      formatter.write(self.message.as_bytes())
   }
}

impl TokenError {
   #[inline]
   pub fn new(msg: String, code: int) -> TokenError {
      TokenError {
         message: msg,
         code: code
      }
   }
}

impl Show for TokenError {
   fn fmt(&self, formatter: &mut Formatter) -> Result<(), FormatError> {
      formatter.write(self.message.as_bytes())
   }
}
