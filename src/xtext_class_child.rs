
// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use syn::parse::{Parse, ParseStream, Result};
use syn::{Token};
use crate::xtext::{XtextClass,XtextDisplayExpr};

pub enum XtextClassChild {
   C(XtextClass),
   D(XtextDisplayExpr)
}
impl Parse for XtextClassChild {
    fn parse(input: ParseStream) -> Result<Self> {
       if input.peek(Token![<]) && input.peek2(Token![?]) {
          let d: XtextDisplayExpr = input.parse()?;
          Ok(XtextClassChild::D(d))
       } else {
          let c: XtextClass = input.parse()?;
          Ok(XtextClassChild::C(c))
       }
    }
}

