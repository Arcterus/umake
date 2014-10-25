use super::GnuMakefile;
use {TokenError, TokenResult, Token, Comma, Dollar};

enum GnuTokenError {
   Unimplemented = -1,
   Unexpected = -2,
}

pub struct Tokenizer;

impl Tokenizer {
   #[inline]
   pub fn new() -> Tokenizer {
      Tokenizer
   }
   
   pub fn tokenize<'a, 'b>(&mut self, makefile: &mut GnuMakefile<'a>, data: &[u8]) -> TokenResult<Vec<Token<'b>>> {
      let mut tokens = vec!();
      
      let len = data.len();
      let mut idx = 0;
      while idx < len {
         let result = match data[idx] {
            b'$' => self.tokenize_var(makefile, data).or_else(|_| self.tokenize_func(makefile, data)).or(Ok((Dollar, 1))),
            b',' => Ok((Comma, 1)),
            // TODO: more
            tok => Err(byte_unexpected_error(tok, "an identifier"))
         };
         let (token, count) = try!(result);
         tokens.push(token);
         idx += count;
      }
      
      Ok(tokens)
   }
   
   fn tokenize_var<'a, 'b>(&mut self, makefile: &mut GnuMakefile<'a>, data: &[u8]) -> TokenResult<(Token<'b>, uint)> {
      /*let mut count = 0;
      let lhs = try!(self.parse_ident(data));
      if data.len() > 0 && data[0] == '='
      }*/
      Err(unimplemented_error("tokenize_var"))
   }
   
   fn tokenize_rule<'a, 'b>(&mut self, makefile: &mut GnuMakefile<'a>, data: &[u8]) -> TokenResult<(Token<'b>, uint)> {
      Err(unimplemented_error("tokenize_rule"))
   }
   
   fn tokenize_func<'a, 'b>(&mut self, makefile: &mut GnuMakefile<'a>, data: &[u8]) -> TokenResult<(Token<'b>, uint)> {
      Err(unimplemented_error("tokenize_func"))
   }
}

#[inline]
fn unimplemented_error(name: &str) -> TokenError {
   TokenError::new(format!("{} not yet implemented.", name), Unimplemented as int)
}

#[inline]
fn unexpected_error(token: &str, expected: &str) -> TokenError {
   phrase_unexpected_error(format!("\"{}\"", token).as_slice(), format!("\"{}\"", expected).as_slice())
}

#[inline]
fn byte_unexpected_error(token: u8, expected: &str) -> TokenError {
   phrase_unexpected_error(format!("\"{}\"", token as char).as_slice(), expected)
}

#[inline]
fn phrase_unexpected_error(token: &str, expected: &str) -> TokenError {
   TokenError::new(format!("Expected {} but found {}.", expected, token), Unexpected as int)
}
