
// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{quote_spanned, ToTokens};
use proc_macro2::{Span, Literal};
use syn::parse::{Parse, ParseStream, Result, Error};
use syn::{Ident, Token, Expr, LitStr, braced};
use syn::token::{Brace};

use crate::xhtml::{XhtmlAttr,Xhtml};

pub enum XhtmlAttrKey {
   S(String),
   G(Expr,String)
}

pub struct XhtmlTag {
   pub tag: String,
   pub attrs: Vec<(XhtmlAttrKey,Option<XhtmlAttr>)>,
   pub inner: Xhtml,
   pub outer_span: Span,
   pub inner_span_start: Span,
   pub inner_span_end: Span,
}

impl ToTokens for XhtmlTag {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let open_tag = Literal::string(&format!("<{}", self.tag));
        (quote_spanned!{self.outer_span=>
           stream.push_str(#open_tag);
        }).to_tokens(tokens);

        for (k,v) in self.attrs.iter() {
            match (k,v) {
               (XhtmlAttrKey::S(k),None) => {
                  let l = Literal::string(&format!(" {}", k));
                  (quote_spanned!{self.outer_span=>
                     stream.push_str(#l);
                  }).to_tokens(tokens);
               }, (XhtmlAttrKey::S(k),Some(XhtmlAttr::S(s))) => {
                  let l = Literal::string(&format!(" {}={}", k, s));
                  (quote_spanned!{self.outer_span=>
                     stream.push_str(#l);
                  }).to_tokens(tokens);
               }, (XhtmlAttrKey::S(k),Some(XhtmlAttr::F(f))) => {
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
               }, (XhtmlAttrKey::S(k),Some(XhtmlAttr::E(e))) => {
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
               }, (XhtmlAttrKey::G(g,k),None) => {
                  let l = Literal::string(&format!(" {}", k));
                  (quote_spanned!{self.outer_span=>
                     if #g { stream.push_str(#l); }
                  }).to_tokens(tokens);
               }, (XhtmlAttrKey::G(g,k),Some(XhtmlAttr::S(s))) => {
                  let l = Literal::string(&format!(" {}={}", k, s));
                  (quote_spanned!{self.outer_span=>
                     if #g { stream.push_str(#l); }
                  }).to_tokens(tokens);
               }, (XhtmlAttrKey::G(g,k),Some(XhtmlAttr::F(f))) => {
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
               }, (XhtmlAttrKey::G(g,k),Some(XhtmlAttr::E(e))) => {
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

           if self.inner.crumbs.len()>0 && self.inner.span().start() > self.inner_span_start.end() {
              let l = Literal::string(" ");
              (quote_spanned!{self.outer_span=>
                 stream.push_str(#l);
              }).to_tokens(tokens);
           }

           self.inner.to_tokens(tokens);

           if self.inner.crumbs.len()>0 && self.inner.span().end() < self.inner_span_end.start() {
              let l = Literal::string(" ");
              (quote_spanned!{self.outer_span=>
                 stream.push_str(#l);
              }).to_tokens(tokens);
           }

           let l = Literal::string(&format!("</{}>", self.tag));
           (quote_spanned!{self.outer_span=>
              stream.push_str(#l);
           }).to_tokens(tokens);
        }
    }
}

impl Parse for XhtmlTag {
    fn parse(input: ParseStream) -> Result<Self> {
        let l1: Token![<] = input.parse()?;
        let t: Ident = input.parse()?;

        let mut attrs: Vec<(XhtmlAttrKey,Option<XhtmlAttr>)> = Vec::new();
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
                  let attr_expr: XhtmlAttr = XhtmlAttr::parse(&content4, key.clone())?;
                  Some(attr_expr)
               } else { None };
               attrs.push(( XhtmlAttrKey::G(expr,key), v ));
            } else {
               let key = if input.peek(Token![as]) { let _:Token![as] = input.parse()?; "as".to_string()
               } else if input.peek(Token![abstract]) { let _:Token![abstract] = input.parse()?; "abstract".to_string()
               } else if input.peek(Token![become]) { let _:Token![become] = input.parse()?; "become".to_string()
               } else if input.peek(Token![box]) { let _:Token![box] = input.parse()?; "box".to_string()
               } else if input.peek(Token![break]) { let _:Token![break] = input.parse()?; "break".to_string()
               } else if input.peek(Token![const]) { let _:Token![const] = input.parse()?; "const".to_string()
               } else if input.peek(Token![continue]) { let _:Token![continue] = input.parse()?; "continue".to_string()
               } else if input.peek(Token![crate]) { let _:Token![crate] = input.parse()?; "crate".to_string()
               } else if input.peek(Token![do]) { let _:Token![do] = input.parse()?; "do".to_string()
               } else if input.peek(Token![else]) { let _:Token![else] = input.parse()?; "else".to_string()
               } else if input.peek(Token![enum]) { let _:Token![enum] = input.parse()?; "enum".to_string()
               } else if input.peek(Token![extern]) { let _:Token![extern] = input.parse()?; "extern".to_string()
               } else if input.peek(Token![final]) { let _:Token![final] = input.parse()?; "final".to_string()
               } else if input.peek(Token![fn]) { let _:Token![fn] = input.parse()?; "fn".to_string()
               } else if input.peek(Token![for]) { let _:Token![for] = input.parse()?; "for".to_string()
               } else if input.peek(Token![if]) { let _:Token![if] = input.parse()?; "if".to_string()
               } else if input.peek(Token![impl]) { let _:Token![impl] = input.parse()?; "impl".to_string()
               } else if input.peek(Token![in]) { let _:Token![in] = input.parse()?; "in".to_string()
               } else if input.peek(Token![let]) { let _:Token![let] = input.parse()?; "let".to_string()
               } else if input.peek(Token![loop]) { let _:Token![loop] = input.parse()?; "loop".to_string()
               } else if input.peek(Token![macro]) { let _:Token![macro] = input.parse()?; "macro".to_string()
               } else if input.peek(Token![match]) { let _:Token![match] = input.parse()?; "match".to_string()
               } else if input.peek(Token![mod]) { let _:Token![mod] = input.parse()?; "mod".to_string()
               } else if input.peek(Token![move]) { let _:Token![move] = input.parse()?; "move".to_string()
               } else if input.peek(Token![mut]) { let _:Token![mut] = input.parse()?; "mut".to_string()
               } else if input.peek(Token![override]) { let _:Token![override] = input.parse()?; "override".to_string()
               } else if input.peek(Token![priv]) { let _:Token![priv] = input.parse()?; "priv".to_string()
               } else if input.peek(Token![pub]) { let _:Token![pub] = input.parse()?; "pub".to_string()
               } else if input.peek(Token![ref]) { let _:Token![ref] = input.parse()?; "ref".to_string()
               } else if input.peek(Token![return]) { let _:Token![return] = input.parse()?; "return".to_string()
               } else if input.peek(Token![self]) { let _:Token![self] = input.parse()?; "self".to_string()
               } else if input.peek(Token![Self]) { let _:Token![Self] = input.parse()?; "Self".to_string()
               } else if input.peek(Token![static]) { let _:Token![static] = input.parse()?; "static".to_string()
               } else if input.peek(Token![struct]) { let _:Token![struct] = input.parse()?; "struct".to_string()
               } else if input.peek(Token![super]) { let _:Token![super] = input.parse()?; "super".to_string()
               } else if input.peek(Token![trait]) { let _:Token![trait] = input.parse()?; "trait".to_string()
               } else if input.peek(Token![type]) { let _:Token![type] = input.parse()?; "type".to_string()
               } else if input.peek(Token![typeof]) { let _:Token![typeof] = input.parse()?; "typeof".to_string()
               } else if input.peek(Token![unsafe]) { let _:Token![unsafe] = input.parse()?; "unsafe".to_string()
               } else if input.peek(Token![unsized]) { let _:Token![unsized] = input.parse()?; "unsized".to_string()
               } else if input.peek(Token![use]) { let _:Token![use] = input.parse()?; "use".to_string()
               } else if input.peek(Token![virtual]) { let _:Token![virtual] = input.parse()?; "virtual".to_string()
               } else if input.peek(Token![where]) { let _:Token![where] = input.parse()?; "where".to_string()
               } else if input.peek(Token![while]) { let _:Token![while] = input.parse()?; "while".to_string()
               } else if input.peek(Token![yield]) { let _:Token![yield] = input.parse()?; "yield".to_string()
               } else if input.peek(LitStr) { let s:LitStr = input.parse()?; s.value()
               } else { let key: Ident = input.parse()?; key.to_string() };
               let v = if input.peek(Token![=]) {
                  let _eq: Token![=] = input.parse()?;
                  let attr_expr: XhtmlAttr = XhtmlAttr::parse(input, key.clone())?;
                  Some(attr_expr)
               } else { None };
               attrs.push(( XhtmlAttrKey::S(key), v ));
           }
        }

        if input.peek(Token![/]) {
           let r1: Token![/] = input.parse()?;
           let r2: Token![>] = input.parse()?;

           Ok(XhtmlTag {
              tag: t.to_string(),
              attrs: attrs,
              inner: Xhtml { crumbs: vec!() },
              outer_span: l1.span.join(r2.span).unwrap_or(l1.span),
              inner_span_start: r1.span.clone(),
              inner_span_end: l1.span.clone(),
           })
        } else {
           let l2: Token![>] = input.parse()?;

           let inner: Xhtml = input.parse()?;

           let r1: Token![<] = input.parse()?;
           let _r2: Token![/] = input.parse()?;
           let t2: Ident = input.parse()?;
           if t.to_string() != t2.to_string() {
              let msg = format!("Expected </{}> found </{}>", t, t2);
              let r = Error::new(t2.span(), msg);
              return Err(r)
           }
           let r3: Token![>] = input.parse()?;
        
           Ok(XhtmlTag {
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

