
// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{format_ident, quote_spanned, ToTokens};
use proc_macro2::{Span};
use syn::parse::{ParseStream, Result};
use syn::{Expr, bracketed};
use syn::token::{Bracket};

pub struct BracketedExpr {
   pub bracket: Bracket,
   pub context: String,
   pub expr: Expr
}

impl ToTokens for BracketedExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
       let ref expr = self.expr;
       let coerce = format_ident!("to_{}", self.context, span=self.bracket.span);

       (quote_spanned! {self.bracket.span=>
          stream.push_str(&#expr.#coerce());
       }).to_tokens(tokens);
    }
}

impl BracketedExpr {
    pub fn span(&self) -> Span {
       self.bracket.span
    }
    pub fn parse(context: String, input: ParseStream) -> Result<Self> {
       let content;
       let content2;
       let bracket1 = bracketed!(content in input);
       let _bracket2 = bracketed!(content2 in content);
       let expr: Expr = content2.parse()?;
       Ok(BracketedExpr{ bracket:bracket1, context:context, expr:expr })
    }
}
