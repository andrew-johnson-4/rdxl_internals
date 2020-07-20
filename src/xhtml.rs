
// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{quote_spanned, ToTokens};
use proc_macro2::{Span};
use syn::parse::{Parse, ParseStream, Result};

pub use crate::bracketed_expr::BracketedExpr;
pub use crate::xhtml_display_expr::XhtmlDisplayExpr;
pub use crate::xhtml_expr::XhtmlExpr;
pub use crate::xhtml_class_attr::XhtmlClassAttr;
pub use crate::xhtml_attr::XhtmlAttr;
pub use crate::xhtml_class_child::XhtmlClassChild;
pub use crate::xhtml_class::XhtmlClass;
pub use crate::xhtml_tag::XhtmlTag;
pub use crate::xhtml_crumb::XhtmlCrumb;

pub struct Xhtml {
    pub crumbs: Vec<XhtmlCrumb>
}

impl Xhtml {
    pub fn span(&self) -> Span {
       if self.crumbs.len() > 0 {
          let mut span = self.crumbs[0].span();
          for c in self.crumbs[1..].iter() {
             span = span.join(c.span()).unwrap_or(c.span());
          }
          span
       } else {
          Span::call_site()
       }
    }
}

impl ToTokens for Xhtml {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut prev: Option<Span> = None;
        for c in self.crumbs.iter() {
            let span = c.span();
            if let Some(sp) = prev {
            if c.does_emit() && span.start() > sp.end() {
               (quote_spanned!{span.clone()=>
                  stream.push_str(" ");
               }).to_tokens(tokens);
            }}

            prev = Some(span.clone());
            c.to_tokens(tokens);
        }
    }
}

impl Parse for Xhtml {
    fn parse(input: ParseStream) -> Result<Self> {
        let crumbs: Vec<XhtmlCrumb> = input.call(XhtmlCrumb::parse_outer)?;

        Ok(Xhtml {
            crumbs: crumbs
        })
    }
}
