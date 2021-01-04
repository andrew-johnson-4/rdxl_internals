
// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{quote_spanned, ToTokens};
use proc_macro2::{Span, Literal};
use syn::parse::{Parse, ParseStream, Result};
use syn::{Token};
use syn::token::{Bracket,Brace};

use crate::core::{TokenAsLiteral};
use crate::xtext::{XtextTag,XtextExpr,BracketedExpr,XtextClass};

pub enum XtextCrumb {
   S(String, Span),
   T(XtextTag),
   E(XtextExpr),
   F(BracketedExpr),
   C(XtextClass)
}

impl XtextCrumb {
    pub fn span(&self) -> Span {
        match self {
            XtextCrumb::S(_,sp) => { sp.clone() }
            XtextCrumb::T(t) => { t.outer_span.clone() }
            XtextCrumb::E(e) => { e.brace_token1.span.clone() }
            XtextCrumb::F(f) => { f.span() }
            XtextCrumb::C(c) => { c.open.span.join(c.close.span).unwrap_or(c.open.span) }
        }
    }
    pub fn parse_outer(input: ParseStream) -> Result<Vec<Self>> {
        let mut cs = vec!();
        while !input.is_empty() &&
              !(input.peek(Token![<]) && input.peek2(Token![/])) {
           let c: XtextCrumb = input.parse()?;
           cs.push(c);
        }
        Ok(cs)
    }
}

impl Parse for XtextCrumb {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![<]) && input.peek2(Token![!]) {
           let c: XtextClass = input.parse()?;
           Ok(XtextCrumb::C(c))
        } else if input.peek(Token![<]) {
           let t: XtextTag = input.parse()?;
           Ok(XtextCrumb::T(t))
        } else if input.peek(Bracket) {
           let f: BracketedExpr = BracketedExpr::parse("markup".to_string(),input)?;
           Ok(XtextCrumb::F(f))
        } else if input.peek(Brace) {
           let e: XtextExpr = input.parse()?;
           Ok(XtextCrumb::E(e))
        } else {
           let t: TokenAsLiteral = input.parse()?;
           Ok(XtextCrumb::S(t.token_literal, t.span))
        }
    }
}

impl ToTokens for XtextCrumb {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
           XtextCrumb::S(s,span) => {
              let l = Literal::string(&s);
              (quote_spanned!{span.clone()=>
                 stream.push_str(#l);
              }).to_tokens(tokens);
           },
           XtextCrumb::T(t) => {
              t.to_tokens(tokens);
           }
           XtextCrumb::E(e) => {
              e.to_tokens(tokens);
           }
           XtextCrumb::F(e) => {
              e.to_tokens(tokens);
           }
           XtextCrumb::C(c) => {
              let span = c.span();
              (quote_spanned!{span=>
                 stream.push_str(&#c.to_string());
              }).to_tokens(tokens);
           }
        }
    }
}
