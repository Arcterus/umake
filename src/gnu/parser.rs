use super::GnuMakefile;
use {MakefileRule, MakefileDag, ParseError, ParseResult, Token};

enum GnuParseError {
   Unimplemented = -1,
}

pub struct Parser;

impl Parser {
   #[inline]
   pub fn new() -> Parser {
      Parser
   }
   
   pub fn parse<'a, 'b, 'c>(&mut self, makefile: &mut GnuMakefile<'a>, data: Vec<Token<'b>>) -> ParseResult<MakefileDag<'c>> {
      let mut dag = MakefileDag::new();
      
      let slice = data.as_slice();
      let len = slice.len();
      let mut idx = 0;
      while idx < len {
         let result = self.parse_var(makefile, slice).or_else(|_| self.parse_rule(makefile, slice)).or_else(|_| self.parse_func(makefile, slice));
         idx += try!(result);
      }
      
      Ok(dag)
   }
   
   fn parse_var<'a, 'b>(&mut self, makefile: &mut GnuMakefile<'a>, tokens: &[Token<'b>]) -> ParseResult<uint> {
      /*let mut count = 0;
      let lhs = try!(self.parse_ident(data));
      if data.len() > 0 && data[0] == '='
      }*/
      Err(unimplemented_error("parse_var"))
   }
   
   fn parse_rule<'a, 'b>(&mut self, makefile: &mut GnuMakefile<'a>, tokens: &[Token<'b>]) -> ParseResult<uint> {
      Err(unimplemented_error("parse_rule"))
   }
   
   fn parse_func<'a, 'b>(&mut self, makefile: &mut GnuMakefile<'a>, data: &[Token<'b>]) -> ParseResult<uint> {
      Err(unimplemented_error("parse_func"))
   }
}

#[inline]
fn unimplemented_error(name: &str) -> ParseError {
   ParseError::new(format!("{} not yet implemented.", name), Unimplemented as int)
}
