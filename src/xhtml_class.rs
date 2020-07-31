
// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{format_ident,quote_spanned, ToTokens};
use proc_macro2::{Span};
use syn::parse::{Parse, ParseStream, Result, Error};
use syn::{Ident, Token};

pub use crate::xhtml::{XhtmlClassChild,XhtmlClassAttr};

pub struct XhtmlClass {
   pub open: Token![<],
   pub name: String,
   pub attrs: Vec<(String,XhtmlClassAttr)>,
   pub children: Vec<XhtmlClassChild>,
   pub close: Token![>]
}

impl XhtmlClass {
    pub fn span(&self) -> Span {
       self.open.span.join(self.close.span).unwrap_or(self.open.span)
    }
}

impl ToTokens for XhtmlClass {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
       let mut ds = proc_macro2::TokenStream::new();
       let span = self.span();
       let name = format_ident!("{}", self.name, span=span);

       for (k,v) in self.attrs.iter() {
          let k = format_ident!("{}", k, span=span);
          (quote_spanned!{span=>
            #k: #v,
          }).to_tokens(&mut ds);
       }

       let mut cs = proc_macro2::TokenStream::new();
       for c in self.children.iter() {
          match c {
             XhtmlClassChild::C(c) => {
                let span = c.span();
                let child_enum = format_ident!("{}Children", self.name, span=span);
                let child_tag = format_ident!("{}", c.name, span=span);
                (quote_spanned!{span=>
                   #child_enum::#child_tag(#c),
                }).to_tokens(&mut cs);
             },
             XhtmlClassChild::D(d) => {
                let span = d.span();
                let child_enum = format_ident!("{}Children", self.name, span=span);
                (quote_spanned!{span=>
                   #child_enum::Display(Box::new(#d)),
                }).to_tokens(&mut cs);
             }
          }
       }

       (quote_spanned!{span=>
          children: vec![#cs],
          ..std::default::Default::default()
       }).to_tokens(&mut ds);

       (quote_spanned!{span=>
          #name {
             #ds
          }
       }).to_tokens(tokens);
    }
}

impl Parse for XhtmlClass {
    fn parse(input: ParseStream) -> Result<Self> {
       let open: Token![<] = input.parse()?;
       let _ex: Token![!] = input.parse()?;
       let name: Ident = input.parse()?;

       let mut attrs = Vec::new();
       while input.peek(Ident) {
          let attr_name: Ident = input.parse()?;
          let _eq: Token![=] = input.parse()?;
          let attr_val = XhtmlClassAttr::parse(input, attr_name.to_string())?;
          attrs.push((attr_name.to_string(), attr_val));
       }

       if input.peek(Token![/]) {
          let _slash: Token![/] = input.parse()?;
          let close: Token![>] = input.parse()?;
          Ok(XhtmlClass {
             open: open,
             name: name.to_string(),
             attrs: attrs,
             children: Vec::new(),
             close: close
          })
       } else {
          let _gt: Token![>] = input.parse()?;
          
          let mut children = Vec::new();
          while !(input.peek(Token![<]) && input.peek2(Token![/])) {
             let c: XhtmlClassChild = input.parse()?;
             children.push(c);
          }

          let _lt: Token![<] = input.parse()?;
          let _slash: Token![/] = input.parse()?;

          let close_tag: Ident = input.parse()?;
          if name.to_string() != close_tag.to_string() {
              let msg = format!("Expected </{}> found </{}>", name, close_tag);
              let r = Error::new(close_tag.span(), msg);
              return Err(r)
           }

          let close: Token![>] = input.parse()?;

          Ok(XhtmlClass {
             open: open,
             name: name.to_string(),
             attrs: attrs,
             children: children,
             close: close
          })
       }
    }
}
