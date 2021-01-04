
// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{format_ident, quote_spanned, TokenStreamExt, ToTokens};
use proc_macro2::{Span, Literal};
use syn::parse::{ParseStream, Result};
use syn::{Token, Expr, LitChar, LitBool, LitStr, LitInt, bracketed, braced};
use syn::token::{Bracket,Brace};
use crate::xtext::XtextClass;

pub enum XtextClassAttr {
   Cl(XtextClass),
   F(Bracket,String,Expr),
   E(Brace,Expr),
   B(LitBool,bool),
   C(LitChar,char),
   U(LitInt,u64),
   S(LitStr,String),
}
impl XtextClassAttr {
   pub fn span(&self) -> Span {
      match self {
         XtextClassAttr::Cl(cl) => { cl.span() },
         XtextClassAttr::F(b,_,_) => { b.span },
         XtextClassAttr::E(b,_) => { b.span },
         XtextClassAttr::B(v,_) => { v.span },
         XtextClassAttr::C(v,_) => { v.span() },
         XtextClassAttr::U(v,_) => { v.span() },
         XtextClassAttr::S(v,_) => { v.span() },
      }
   }
   pub fn parse(input: ParseStream, key: String) -> Result<Self> {
      if input.peek(Bracket) {
         let _content;
         let content2;
         let bracket_token1:Bracket = bracketed!(_content in input);
         let _bracket_token2:Bracket = bracketed!(content2 in _content);
         let e: Expr = content2.parse()?;
         Ok(XtextClassAttr::F(bracket_token1,key,e))
      } else if input.peek(Brace) {
         let _content;
         let content2;
         let brace_token1:Brace = braced!(_content in input);
         let _brace_token2:Brace = braced!(content2 in _content);
         let e: Expr = content2.parse()?;
         Ok(XtextClassAttr::E(brace_token1,e))
      } else if input.peek(LitBool) {
         let b: LitBool = input.parse()?;
         Ok(XtextClassAttr::B(b.clone(),b.value))
      } else if input.peek(LitInt) {
         let b: LitInt = input.parse()?;
         let u: u64 = b.base10_parse()?;
         Ok(XtextClassAttr::U(b.clone(),u))
      } else if input.peek(LitChar) {
         let b: LitChar = input.parse()?;
         Ok(XtextClassAttr::C(b.clone(),b.value()))
      } else if input.peek(Token![<]) && input.peek2(Token![!]) {
         let cl: XtextClass = input.parse()?;
         Ok(XtextClassAttr::Cl(cl))
      } else {
         let val: LitStr = input.parse()?;
         Ok(XtextClassAttr::S(val.clone(),val.value()))
      }
   }
}
impl ToTokens for XtextClassAttr {
   fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
      let span = self.span();
      match self {
         XtextClassAttr::S(_,s) => {
            let l: Literal = Literal::string(&s);
            (quote_spanned!{span=>
               #l.to_string()
            }).to_tokens(tokens);
         }, XtextClassAttr::B(_,e) => {
            let e = format_ident!("{}", e, span=span);
            e.to_tokens(tokens);
         }, XtextClassAttr::Cl(cl) => {
            cl.to_tokens(tokens);
         }, XtextClassAttr::C(_,e) => {
            let l: Literal = Literal::character(*e);
            tokens.append(l);
         }, XtextClassAttr::U(_,e) => {
            let l: Literal = Literal::u64_unsuffixed(*e);
            tokens.append(l);
         }, XtextClassAttr::F(_,f,e) => {
            let coerce = format_ident!("to_{}", f, span=span);
            (quote_spanned!{span=>
               #e.#coerce()
            }).to_tokens(tokens);
         }, XtextClassAttr::E(_,e) => {
            e.to_tokens(tokens);
         }
      }
   }
}


