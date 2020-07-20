
// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{quote_spanned, ToTokens};
use proc_macro2::{Span};
use syn::parse::{Parse, ParseStream, Result};
use syn::{Token};
use crate::xhtml::Xhtml;

pub enum XhtmlDisplay {
   X(Xhtml)
}
impl ToTokens for XhtmlDisplay {
   fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
      match self {
         XhtmlDisplay::X(xhtmls) => {
            let expanded = quote_spanned! { xhtmls.span() =>
               {
                  let mut stream = String::new();
                  #xhtmls
                  stream
               }
            };
            expanded.to_tokens(tokens);
         }
      }
   }
}

pub struct XhtmlDisplayExpr {
   pub open: Token![<],
   pub expr: XhtmlDisplay,
   pub close: Token![>],
}
impl XhtmlDisplayExpr {
    pub fn span(&self) -> Span {
       self.open.span.join(self.close.span).unwrap_or(self.open.span)
    }
}
impl Parse for XhtmlDisplayExpr {
    fn parse(input: ParseStream) -> Result<Self> {
       let open: Token![<] = input.parse()?;
       let _: Token![?] = input.parse()?;
       let _: Token![>] = input.parse()?;

       let xhtml: Xhtml = input.parse()?;
       let expr = XhtmlDisplay::X(xhtml);

       let _: Token![<] = input.parse()?;
       let _: Token![/] = input.parse()?;
       let _: Token![?] = input.parse()?;
       let close: Token![>] = input.parse()?;

       Ok(XhtmlDisplayExpr {
          open: open,
          expr: expr,
          close: close,
       })
    }
}
impl ToTokens for XhtmlDisplayExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
       self.expr.to_tokens(tokens);
    }
}

