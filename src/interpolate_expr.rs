// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{Token, Expr, Pat, braced};
use syn::token::{Brace};
use crate::xhtml::XhtmlCrumb;

pub enum XhtmlExprInner {
   S(Expr),
   E(Expr),
   F(Token![for],Pat,Expr,Vec<XhtmlCrumb>),
   W(Token![while],Expr,Vec<XhtmlCrumb>),
   L(Token![let],Pat,Expr),
   I(Token![if],Expr,Vec<XhtmlCrumb>,Vec<(Expr,Vec<XhtmlCrumb>)>,Vec<XhtmlCrumb>),
   P(Token![loop],Vec<XhtmlCrumb>),
}
impl XhtmlExprInner {
    pub fn does_emit(&self) -> bool {
       match self {
          XhtmlExprInner::S(_) => { false },
          XhtmlExprInner::E(_) => { true },
          XhtmlExprInner::F(_,_,_,_) => { true },
          XhtmlExprInner::P(_,_) => { true },
          XhtmlExprInner::W(_,_,_) => { true },
          XhtmlExprInner::L(_,_,_) => { false },
          XhtmlExprInner::I(_,_,_,_,_) => { true },
       }
    }
}
impl ToTokens for XhtmlExprInner {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
           XhtmlExprInner::E(e) => {
              (quote_spanned!{ syn::spanned::Spanned::span(e)=>
                 stream.push_str(&#e.to_string());
              }).to_tokens(tokens);
           }, XhtmlExprInner::S(e) => {
              (quote_spanned!{ syn::spanned::Spanned::span(e)=>
                 #e;
              }).to_tokens(tokens);
           }, XhtmlExprInner::F(f,p,i,cs) => {
              (quote_spanned!{f.span=>
                 for #p in #i { #(#cs)* stream.push_str(" "); }
              }).to_tokens(tokens);
           }, XhtmlExprInner::P(l,cs) => {
              (quote_spanned!{l.span=>
                 loop { #(#cs)* stream.push_str(" "); }
              }).to_tokens(tokens);
           }, XhtmlExprInner::I(i,c,bs,es,e) => {
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
           }, XhtmlExprInner::W(w,i,cs) => {
              (quote_spanned!{w.span=>
                 while #i { #(#cs)* stream.push_str(" "); }
              }).to_tokens(tokens);
           }, XhtmlExprInner::L(t,l,e) => {
              (quote_spanned!{t.span=>
                 let #l = #e;
              }).to_tokens(tokens);
           }
        }
    }
}
impl Parse for XhtmlExprInner {
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
          let body: Vec<XhtmlCrumb> = content2.call(XhtmlCrumb::parse_outer)?;
          Ok(XhtmlExprInner::F(_for,pat,iter,body))
       } else if input.peek(Token![loop]) {
          let _loop: Token![loop] = input.parse()?;
          let content;
          let content2;
          let _brace1 = braced!(content in input);
          let _brace2 = braced!(content2 in content);
          let body: Vec<XhtmlCrumb> = content2.call(XhtmlCrumb::parse_outer)?;
          Ok(XhtmlExprInner::P(_loop,body))
       } else if input.peek(Token![while]) {
          let _while: Token![while] = input.parse()?;
          let iter: Expr = input.parse()?;
          let content;
          let content2;
          let _brace1 = braced!(content in input);
          let _brace2 = braced!(content2 in content);
          let body: Vec<XhtmlCrumb> = content2.call(XhtmlCrumb::parse_outer)?;
          Ok(XhtmlExprInner::W(_while,iter,body))
       } else if input.peek(Token![if]) {
          let _if: Token![if] = input.parse()?;
          let b: Expr = input.parse()?;
          let mut es = Vec::new();
          let mut e = Vec::new();
          let content;
          let content2;
          let _brace1 = braced!(content in input);
          let _brace2 = braced!(content2 in content);
          let body: Vec<XhtmlCrumb> = content2.call(XhtmlCrumb::parse_outer)?;

          while input.peek(Token![else]) && input.peek2(Token![if]) {
             let _else: Token![else] = input.parse()?;
             let _if: Token![if] = input.parse()?;
             let b: Expr = input.parse()?;
             let content;
             let content2;
             let _brace1 = braced!(content in input);
             let _brace2 = braced!(content2 in content);
             let e = content2.call(XhtmlCrumb::parse_outer)?;
             es.push((b,e));
          }

          if input.peek(Token![else]) {
             let _else: Token![else] = input.parse()?;
             let content;
             let content2;
             let _brace1 = braced!(content in input);
             let _brace2 = braced!(content2 in content);
             e = content2.call(XhtmlCrumb::parse_outer)?;
          }

          Ok(XhtmlExprInner::I(_if,b,body,es,e))
       } else if input.peek(Token![let]) {
          let _let: Token![let] = input.parse()?;
          let pat: Pat = input.parse()?;
          let _eq: Token![=] = input.parse()?;
          let expr: Expr = input.parse()?;
          if input.peek(Token![;]) {
             let _: Token![;] = input.parse()?;
          }
          Ok(XhtmlExprInner::L(_let,pat,expr))
       } else {
          let e: Expr = input.parse()?;
          if input.peek(Token![;]) {
             let _semi: Token![;] = input.parse()?;
             Ok(XhtmlExprInner::S(e))
          } else {
             Ok(XhtmlExprInner::E(e))
          }
       }
    }
}

pub struct XhtmlExpr {
   pub brace_token1: Brace,
   pub brace_token2: Brace,
   pub expr: XhtmlExprInner
}
impl XhtmlExpr {
    pub fn does_emit(&self) -> bool {
       self.expr.does_emit()
    }
}
impl Parse for XhtmlExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let _content;
        let content2;
        Ok(XhtmlExpr {
           brace_token1: braced!(_content in input),
           brace_token2: braced!(content2 in _content),
           expr: content2.call(XhtmlExprInner::parse)?,
        })
    }
}
impl ToTokens for XhtmlExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.expr.to_tokens(tokens)
    }
}

