use std::collections::{HashMap};
use super::{MakefileDag, Makefile, ParseResult, Token, TokenResult};

mod parser;
mod lexer;

pub struct GnuMakefile<'a> {
   vars: HashMap<&'a [u8], Vec<u8>>,
}

impl<'a> GnuMakefile<'a> {
   #[inline]
   pub fn new() -> GnuMakefile<'a> {
      GnuMakefile {
         vars: HashMap::new()
      }
   }
}

impl<'a> Makefile<'a> for GnuMakefile<'a> {
   #[inline]
   fn vars(&mut self) -> &mut MutableMap<&'a [u8], Vec<u8>> {
      &mut self.vars as &mut MutableMap<&'a [u8], Vec<u8>>
   }
   
   fn parse<'b, 'c>(&mut self, data: Vec<Token<'c>>) -> ParseResult<MakefileDag<'b>> {
      parser::Parser::new().parse(self, data)
   }
   
   fn tokenize<'b>(&mut self, data: &[u8]) -> TokenResult<Vec<Token<'b>>> {
      lexer::Tokenizer::new().tokenize(self, data)
   }
}
