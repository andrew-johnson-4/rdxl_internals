
// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{quote_spanned, ToTokens};
use proc_macro2::{Span};
use syn::parse::{Parse, ParseStream, Result};
use syn::{Token};
use crate::xtext::Xtext;

pub enum XtextDisplay {
   X(Xtext)
}
impl ToTokens for XtextDisplay {
   fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
      match self {
         XtextDisplay::X(Xtexts) => {
            let expanded = quote_spanned! { Xtexts.span() =>
               {
                  let mut stream = String::new();
                  #Xtexts
                  stream
               }
            };
            expanded.to_tokens(tokens);
         }
      }
   }
}

pub struct XtextDisplayExpr {
   pub open: Token![<],
   pub expr: XtextDisplay,
   pub close: Token![>],
}
impl XtextDisplayExpr {
    pub fn span(&self) -> Span {
       self.open.span.join(self.close.span).unwrap_or(self.open.span)
    }
}
impl Parse for XtextDisplayExpr {
    fn parse(input: ParseStream) -> Result<Self> {
       let open: Token![<] = input.parse()?;
       let _: Token![?] = input.parse()?;
       let _: Token![>] = input.parse()?;

       let Xtext: Xtext = input.parse()?;
       let expr = XtextDisplay::X(Xtext);

       let _: Token![<] = input.parse()?;
       let _: Token![/] = input.parse()?;
       let _: Token![?] = input.parse()?;
       let close: Token![>] = input.parse()?;

       Ok(XtextDisplayExpr {
          open: open,
          expr: expr,
          close: close,
       })
    }
}
impl ToTokens for XtextDisplayExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
       self.expr.to_tokens(tokens);
    }
}

