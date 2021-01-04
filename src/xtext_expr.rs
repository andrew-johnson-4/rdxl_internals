// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{Token, Expr, Pat, braced};
use syn::token::{Brace};
use crate::xtext::XtextCrumb;

pub enum XtextExprInner {
   S(Expr),
   E(Expr),
   F(Token![for],Pat,Expr,Vec<XtextCrumb>),
   W(Token![while],Expr,Vec<XtextCrumb>),
   L(Token![let],Pat,Expr),
   I(Token![if],Expr,Vec<XtextCrumb>,Vec<(Expr,Vec<XtextCrumb>)>,Vec<XtextCrumb>),
   P(Token![loop],Vec<XtextCrumb>),
}
impl ToTokens for XtextExprInner {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
           XtextExprInner::E(e) => {
              (quote_spanned!{ syn::spanned::Spanned::span(e)=>
                 stream.push_str(&#e.to_string());
              }).to_tokens(tokens);
           }, XtextExprInner::S(e) => {
              (quote_spanned!{ syn::spanned::Spanned::span(e)=>
                 #e;
              }).to_tokens(tokens);
           }, XtextExprInner::F(f,p,i,cs) => {
              (quote_spanned!{f.span=>
                 for #p in #i { #(#cs)* stream.push_str(" "); }
              }).to_tokens(tokens);
           }, XtextExprInner::P(l,cs) => {
              (quote_spanned!{l.span=>
                 loop { #(#cs)* stream.push_str(" "); }
              }).to_tokens(tokens);
           }, XtextExprInner::I(i,c,bs,es,e) => {
              (quote_spanned!{i.span=>
                if #c { #(#bs)* stream.push_str(" "); }
              }).to_tokens(tokens);

              for (c,e) in es.iter() {
                 (quote_spanned!{i.span=>
                    else if #c { #(#e)* stream.push_str(" "); }
                 }).to_tokens(tokens);
              }

              if e.len() > 0 {
                 (quote_spanned!{i.span=>
                    else { #(#e)* stream.push_str(" "); }
                 }).to_tokens(tokens);
              }
           }, XtextExprInner::W(w,i,cs) => {
              (quote_spanned!{w.span=>
                 while #i { #(#cs)* stream.push_str(" "); }
              }).to_tokens(tokens);
           }, XtextExprInner::L(t,l,e) => {
              (quote_spanned!{t.span=>
                 let #l = #e;
              }).to_tokens(tokens);
           }
        }
    }
}
impl Parse for XtextExprInner {
    fn parse(input: ParseStream) -> Result<Self> {
       if input.peek(Token![for]) {
          let _for: Token![for] = input.parse()?;
          let pat: Pat = input.parse()?;
          let _in: Token![in] = input.parse()?;
          let iter: Expr = input.parse()?;
          let content;
          let content2;
          let _brace1 = braced!(content in input);
          let _brace2 = braced!(content2 in content);
          let body: Vec<XtextCrumb> = content2.call(XtextCrumb::parse_outer)?;
          Ok(XtextExprInner::F(_for,pat,iter,body))
       } else if input.peek(Token![loop]) {
          let _loop: Token![loop] = input.parse()?;
          let content;
          let content2;
          let _brace1 = braced!(content in input);
          let _brace2 = braced!(content2 in content);
          let body: Vec<XtextCrumb> = content2.call(XtextCrumb::parse_outer)?;
          Ok(XtextExprInner::P(_loop,body))
       } else if input.peek(Token![while]) {
          let _while: Token![while] = input.parse()?;
          let iter: Expr = input.parse()?;
          let content;
          let content2;
          let _brace1 = braced!(content in input);
          let _brace2 = braced!(content2 in content);
          let body: Vec<XtextCrumb> = content2.call(XtextCrumb::parse_outer)?;
          Ok(XtextExprInner::W(_while,iter,body))
       } else if input.peek(Token![if]) {
          let _if: Token![if] = input.parse()?;
          let b: Expr = input.parse()?;
          let mut es = Vec::new();
          let mut e = Vec::new();
          let content;
          let content2;
          let _brace1 = braced!(content in input);
          let _brace2 = braced!(content2 in content);
          let body: Vec<XtextCrumb> = content2.call(XtextCrumb::parse_outer)?;

          while input.peek(Token![else]) && input.peek2(Token![if]) {
             let _else: Token![else] = input.parse()?;
             let _if: Token![if] = input.parse()?;
             let b: Expr = input.parse()?;
             let content;
             let content2;
             let _brace1 = braced!(content in input);
             let _brace2 = braced!(content2 in content);
             let e = content2.call(XtextCrumb::parse_outer)?;
             es.push((b,e));
          }

          if input.peek(Token![else]) {
             let _else: Token![else] = input.parse()?;
             let content;
             let content2;
             let _brace1 = braced!(content in input);
             let _brace2 = braced!(content2 in content);
             e = content2.call(XtextCrumb::parse_outer)?;
          }

          Ok(XtextExprInner::I(_if,b,body,es,e))
       } else if input.peek(Token![let]) {
          let _let: Token![let] = input.parse()?;
          let pat: Pat = input.parse()?;
          let _eq: Token![=] = input.parse()?;
          let expr: Expr = input.parse()?;
          if input.peek(Token![;]) {
             let _: Token![;] = input.parse()?;
          }
          Ok(XtextExprInner::L(_let,pat,expr))
       } else {
          let e: Expr = input.parse()?;
          if input.peek(Token![;]) {
             let _semi: Token![;] = input.parse()?;
             Ok(XtextExprInner::S(e))
          } else {
             Ok(XtextExprInner::E(e))
          }
       }
    }
}

pub struct XtextExpr {
   pub brace_token1: Brace,
   pub brace_token2: Brace,
   pub expr: XtextExprInner
}
impl Parse for XtextExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let _content;
        let content2;
        Ok(XtextExpr {
           brace_token1: braced!(_content in input),
           brace_token2: braced!(content2 in _content),
           expr: content2.call(XtextExprInner::parse)?,
        })
    }
}
impl ToTokens for XtextExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.expr.to_tokens(tokens)
    }
}

