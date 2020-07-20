
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
use crate::xhtml::{XhtmlTag,XhtmlExpr,BracketedExpr,XhtmlClass};

pub enum XhtmlCrumb {
   S(String, Span),
   T(XhtmlTag),
   E(XhtmlExpr),
   F(BracketedExpr),
   C(XhtmlClass)
}

impl XhtmlCrumb {
    pub fn does_emit(&self) -> bool {
       match self {
          XhtmlCrumb::S(_,_) => { true },
          XhtmlCrumb::T(_) => { true },
          XhtmlCrumb::E(e) => { e.does_emit() },
          XhtmlCrumb::F(_) => { true },
          XhtmlCrumb::C(_) => { true },
       }
    }
    pub fn span(&self) -> Span {
        match self {
            XhtmlCrumb::S(_,sp) => { sp.clone() }
            XhtmlCrumb::T(t) => { t.outer_span.clone() }
            XhtmlCrumb::E(e) => { e.brace_token1.span.clone() }
            XhtmlCrumb::F(f) => { f.span() }
            XhtmlCrumb::C(c) => { c.open.span.join(c.close.span).unwrap_or(c.open.span) }
        }
    }
    pub fn parse_outer(input: ParseStream) -> Result<Vec<Self>> {
        let mut cs = vec!();
        while !input.is_empty() &&
              !(input.peek(Token![<]) && input.peek2(Token![/])) {
           let c: XhtmlCrumb = input.parse()?;
           cs.push(c);
        }
        Ok(cs)
    }
}

impl Parse for XhtmlCrumb {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![<]) && input.peek2(Token![!]) {
           let c: XhtmlClass = input.parse()?;
           Ok(XhtmlCrumb::C(c))
        } else if input.peek(Token![<]) {
           let t: XhtmlTag = input.parse()?;
           Ok(XhtmlCrumb::T(t))
        } else if input.peek(Bracket) {
           let f: BracketedExpr = BracketedExpr::parse("markup".to_string(),input)?;
           Ok(XhtmlCrumb::F(f))
        } else if input.peek(Brace) {
           let e: XhtmlExpr = input.parse()?;
           Ok(XhtmlCrumb::E(e))
        } else {
           let t: TokenAsLiteral = input.parse()?;
           Ok(XhtmlCrumb::S(t.token_literal, t.span))
        }
    }
}

impl ToTokens for XhtmlCrumb {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
           XhtmlCrumb::S(s,span) => {
              let l = Literal::string(&s);
              (quote_spanned!{span.clone()=>
                 stream.push_str(#l);
              }).to_tokens(tokens);
           },
           XhtmlCrumb::T(t) => {
              t.to_tokens(tokens);
           }
           XhtmlCrumb::E(e) => {
              e.to_tokens(tokens);
           }
           XhtmlCrumb::F(e) => {
              e.to_tokens(tokens);
           }
           XhtmlCrumb::C(c) => {
              let span = c.span();
              (quote_spanned!{span=>
                 stream.push_str(&#c.to_string());
              }).to_tokens(tokens);
           }
        }
    }
}
