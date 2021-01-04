
// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{quote_spanned, ToTokens};
use proc_macro2::{Span};
use syn::parse::{Parse, ParseStream, Result};

pub use crate::bracketed_expr::BracketedExpr;
pub use crate::xtext_display_expr::XtextDisplayExpr;
pub use crate::xtext_expr::XtextExpr;
pub use crate::xtext_class_attr::XtextClassAttr;
pub use crate::xtext_attr::XtextAttr;
pub use crate::xtext_class_child::XtextClassChild;
pub use crate::xtext_class::XtextClass;
pub use crate::xtext_tag::XtextTag;
pub use crate::xtext_crumb::XtextCrumb;

pub struct Xtext {
    pub crumbs: Vec<XtextCrumb>
}

impl Xtext {
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

impl ToTokens for Xtext {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut prev: Option<Span> = None;
        for c in self.crumbs.iter() {
            c.to_tokens(tokens);
        }
    }
}

impl Parse for Xtext {
    fn parse(input: ParseStream) -> Result<Self> {
        let crumbs: Vec<XtextCrumb> = input.call(XtextCrumb::parse_outer)?;

        Ok(Xtext {
            crumbs: crumbs
        })
    }
}
