
// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use syn::parse::{ParseStream, Result};
use syn::{LitChar, LitBool, LitStr, LitInt};
use syn::token::{Bracket,Brace};
use crate::xhtml::{XhtmlExpr,BracketedExpr};

pub enum XhtmlAttr {
   S(String),
   F(BracketedExpr),
   E(XhtmlExpr)
}
impl XhtmlAttr {
   pub fn parse(input: ParseStream, key: String) -> Result<Self> {
      if input.peek(Bracket) {
         let f: BracketedExpr = BracketedExpr::parse(key.clone(),input)?;
         Ok(XhtmlAttr::F(f))
      } else if input.peek(Brace) {
         let e: XhtmlExpr = input.parse()?;
         Ok(XhtmlAttr::E(e))
      } else if input.peek(LitBool) {
         let b: LitBool = input.parse()?;
         Ok(XhtmlAttr::S(format!("{}", b.value)))
      } else if input.peek(LitInt) {
         let b: LitInt = input.parse()?;
         Ok(XhtmlAttr::S(format!("{}", b.base10_digits())))
      } else if input.peek(LitChar) {
         let b: LitChar = input.parse()?;
         Ok(XhtmlAttr::S(format!("'{}'", b.value())))
      } else {
         let val: LitStr = input.parse()?;
         Ok(XhtmlAttr::S(format!("{:?}",val.value())))
      }
   }
}
