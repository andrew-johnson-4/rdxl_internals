
// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{quote_spanned, ToTokens};
use proc_macro2::{Span, Literal};
use syn::parse::{Parse, ParseStream, Result, Error};
use syn::{Ident, Token, Expr, LitStr, braced};
use syn::token::{Brace};

use crate::xtext::{XtextAttr,Xtext};
use crate::core::TokenAsLiteral;

pub enum XtextAttrKey {
   S(String),
   G(Expr,String)
}

pub struct XtextTag {
   pub tag: String,
   pub attrs: Vec<(XtextAttrKey,Option<XtextAttr>)>,
   pub inner: Xtext,
   pub outer_span: Span,
   pub inner_span_start: Span,
   pub inner_span_end: Span,
}

impl ToTokens for XtextTag {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let open_tag = Literal::string(&format!("<{}", self.tag));
        (quote_spanned!{self.outer_span=>
           stream.push_str(#open_tag);
        }).to_tokens(tokens);

        for (k,v) in self.attrs.iter() {
            match (k,v) {
               (XtextAttrKey::S(k),None) => {
                  let l = Literal::string(&format!(" {}", k));
                  (quote_spanned!{self.outer_span=>
                     stream.push_str(#l);
                  }).to_tokens(tokens);
               }, (XtextAttrKey::S(k),Some(XtextAttr::S(s))) => {
                  let l = Literal::string(&format!(" {}={}", k, s));
                  (quote_spanned!{self.outer_span=>
                     stream.push_str(#l);
                  }).to_tokens(tokens);
               }, (XtextAttrKey::S(k),Some(XtextAttr::F(f))) => {
                  let l = Literal::string(&format!(" {}=", k));
                  (quote_spanned!{self.outer_span=>
                     stream.push_str(#l);
                     stream.push_str("\""); 
                     stream.push_str(&{
                       let mut stream = String::new();
                       #f
                       stream.replace("\"", "\\\"")
                     });
                     stream.push_str("\""); 
                  }).to_tokens(tokens);
               }, (XtextAttrKey::S(k),Some(XtextAttr::E(e))) => {
                  let l = Literal::string(&format!(" {}=", k));
                  (quote_spanned!{self.outer_span=>
                     stream.push_str(#l);
                     stream.push_str("\""); 
                     stream.push_str(&{
                       let mut stream = String::new();
                       #e
                       stream.replace("\"", "\\\"")
                     });
                     stream.push_str("\""); 
                  }).to_tokens(tokens);
               }, (XtextAttrKey::G(g,k),None) => {
                  let l = Literal::string(&format!(" {}", k));
                  (quote_spanned!{self.outer_span=>
                     if #g { stream.push_str(#l); }
                  }).to_tokens(tokens);
               }, (XtextAttrKey::G(g,k),Some(XtextAttr::S(s))) => {
                  let l = Literal::string(&format!(" {}={}", k, s));
                  (quote_spanned!{self.outer_span=>
                     if #g { stream.push_str(#l); }
                  }).to_tokens(tokens);
               }, (XtextAttrKey::G(g,k),Some(XtextAttr::F(f))) => {
                  let l = Literal::string(&format!(" {}=", k));
                  (quote_spanned!{self.outer_span=>
                     if #g {
                        stream.push_str(#l);
                        stream.push_str("\""); 
                        stream.push_str(&{
                          let mut stream = String::new();
                          #f
                          stream.replace("\"", "\\\"")
                        });
                        stream.push_str("\""); 
                     }
                  }).to_tokens(tokens);
               }, (XtextAttrKey::G(g,k),Some(XtextAttr::E(e))) => {
                  let l = Literal::string(&format!(" {}=", k));
                  (quote_spanned!{self.outer_span=>
                     if #g {
                        stream.push_str(#l);
                        stream.push_str("\""); 
                        stream.push_str(&{
                          let mut stream = String::new();
                          #e
                          stream.replace("\"", "\\\"")
                        });
                        stream.push_str("\""); 
                     }
                  }).to_tokens(tokens);
               }
            }
        }

        let self_closing = vec!["area","base","br","embed","hr","iframe","img",
           "input","link","meta","param","source","track"];
        if self.inner.crumbs.len()==0 && self_closing.iter().any(|s| (&self.tag)==s) {
           let l = Literal::string("/>");
           (quote_spanned!{self.outer_span=>
              stream.push_str(#l);
           }).to_tokens(tokens);
        } else {
           let l = Literal::string(">");
           (quote_spanned!{self.outer_span=>
              stream.push_str(#l);
           }).to_tokens(tokens);

           self.inner.to_tokens(tokens);

           let l = Literal::string(&format!("</{}>", self.tag));
           (quote_spanned!{self.outer_span=>
              stream.push_str(#l);
           }).to_tokens(tokens);
        }
    }
}

impl Parse for XtextTag {
    fn parse(input: ParseStream) -> Result<Self> {
        let l1: Token![<] = input.parse()?;
        let t: Ident = input.parse()?;

        let mut attrs: Vec<(XtextAttrKey,Option<XtextAttr>)> = Vec::new();
        while input.peek(Ident) ||
              input.peek(LitStr) ||
              input.peek(Token![as]) ||
              input.peek(Token![break]) ||
              input.peek(Token![const]) ||
              input.peek(Token![continue]) ||
              input.peek(Token![crate]) ||
              input.peek(Token![else]) ||
              input.peek(Token![enum]) ||
              input.peek(Token![extern]) ||
              input.peek(Token![fn]) ||
              input.peek(Token![for]) ||
              input.peek(Token![if]) ||
              input.peek(Token![impl]) ||
              input.peek(Token![in]) ||
              input.peek(Token![let]) ||
              input.peek(Token![loop]) ||
              input.peek(Token![match]) ||
              input.peek(Token![mod]) ||
              input.peek(Token![move]) ||
              input.peek(Token![mut]) ||
              input.peek(Token![pub]) ||
              input.peek(Token![ref]) ||
              input.peek(Token![return]) ||
              input.peek(Token![self]) ||
              input.peek(Token![Self]) ||
              input.peek(Token![static]) ||
              input.peek(Token![struct]) ||
              input.peek(Token![super]) ||
              input.peek(Token![trait]) ||
              input.peek(Token![type]) ||
              input.peek(Token![unsafe]) ||
              input.peek(Token![use]) ||
              input.peek(Token![where]) ||
              input.peek(Token![while]) ||
              input.peek(Brace) {
            if input.peek(Brace) {
               let content1;
               let content2;
               let _brace1: Brace = braced!(content1 in input);
               let _brace2: Brace = braced!(content2 in content1);
               let _if: Token![if] = content2.parse()?;
               let expr: Expr = content2.parse()?;

               let content3;
               let content4;
               let _brace3: Brace = braced!(content3 in content2);
               let _brace4: Brace = braced!(content4 in content3);
               let key = if content4.peek(LitStr) { let s:LitStr = content4.parse()?; s.value()
                         } else { let key: Ident = content4.parse()?; key.to_string() };
               let v = if content4.peek(Token![=]) {
                  let _eq: Token![=] = content4.parse()?;
                  let attr_expr: XtextAttr = XtextAttr::parse(&content4, key.clone())?;
                  Some(attr_expr)
               } else { None };
               attrs.push(( XtextAttrKey::G(expr,key), v ));
            } else {
               let t: TokenAsLiteral = input.parse()?;
               let key = t.token_literal.clone();
               let v = if input.peek(Token![=]) {
                  let _eq: Token![=] = input.parse()?;
                  let attr_expr: XtextAttr = XtextAttr::parse(input, key.clone())?;
                  Some(attr_expr)
               } else { None };
               attrs.push(( XtextAttrKey::S(key), v ));
           }
        }

        if input.peek(Token![/]) {
           let r1: Token![/] = input.parse()?;
           let r2: Token![>] = input.parse()?;

           Ok(XtextTag {
              tag: t.to_string(),
              attrs: attrs,
              inner: Xtext { crumbs: vec!() },
              outer_span: l1.span.join(r2.span).unwrap_or(l1.span),
              inner_span_start: r1.span.clone(),
              inner_span_end: r2.span.clone(),
           })
        } else {
           let l2: Token![>] = input.parse()?;

           let inner: Xtext = input.parse()?;

           let r1: Token![<] = input.parse()?;
           let _r2: Token![/] = input.parse()?;
           let t2: Ident = input.parse()?;
           if t.to_string() != t2.to_string() {
              let msg = format!("Expected </{}> found </{}>", t, t2);
              let r = Error::new(t2.span(), msg);
              return Err(r)
           }
           let r3: Token![>] = input.parse()?;
        
           Ok(XtextTag {
              tag: t.to_string(),
              attrs: attrs,
              inner: inner,
              outer_span: l1.span.join(r3.span).unwrap_or(l1.span),
              inner_span_start: l2.span.clone(),
              inner_span_end: r1.span.clone(),
           })
       }
    }
}

