
// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use syn::parse::{Parse, ParseStream, Result};
use syn::{Token};
use crate::xhtml::{XhtmlClass,XhtmlDisplayExpr};

pub enum XhtmlClassChild {
   C(XhtmlClass),
   D(XhtmlDisplayExpr)
}
impl Parse for XhtmlClassChild {
    fn parse(input: ParseStream) -> Result<Self> {
       if input.peek(Token![<]) && input.peek2(Token![?]) {
          let d: XhtmlDisplayExpr = input.parse()?;
          Ok(XhtmlClassChild::D(d))
       } else {
          let c: XhtmlClass = input.parse()?;
          Ok(XhtmlClassChild::C(c))
       }
    }
}

