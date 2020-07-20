
// Copyright 2020, The rdxl Project Developers.
// Dual Licensed under the MIT license and the Apache 2.0 license,
// see the LICENSE file or <http://opensource.org/licenses/MIT>
// also see LICENSE2 file or <https://www.apache.org/licenses/LICENSE-2.0>

use quote::{quote_spanned, ToTokens};
use proc_macro2::{Span, Literal};
use syn::parse::{Parse, ParseStream, Result};
use syn::{Ident, Token, LitChar, LitBool, LitStr, LitInt};
use syn::token::{Bracket,Brace};

use crate::xhtml::{XhtmlTag,XhtmlExpr,BracketedExpr,XhtmlClass};

pub enum XhtmlCrumb {
   L(LitStr),
   S(String, Span),
   T(XhtmlTag),
   E(XhtmlExpr),
   F(BracketedExpr),
   C(XhtmlClass)
}

impl XhtmlCrumb {
    pub fn does_emit(&self) -> bool {
       match self {
          XhtmlCrumb::L(_) => { true },
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
            XhtmlCrumb::L(l) => { l.span() }
            XhtmlCrumb::C(c) => { c.open.span.join(c.close.span).unwrap_or(c.open.span) }
        }
    }
    pub fn parse_outer(input: ParseStream) -> Result<Vec<Self>> {
        let mut cs = vec!();
        while input.peek(Ident) ||
              input.peek(LitBool) ||
              input.peek(LitChar) ||
              input.peek(LitInt) ||
              input.peek(LitStr) ||
              input.peek(Brace) ||
              input.peek(Bracket) ||
              input.peek(Token![abstract]) ||
              input.peek(Token![as]) ||
              input.peek(Token![become]) ||
              input.peek(Token![box]) ||
              input.peek(Token![break]) ||
              input.peek(Token![const]) ||
              input.peek(Token![continue]) ||
              input.peek(Token![crate]) ||
              input.peek(Token![do]) ||
              input.peek(Token![else]) ||
              input.peek(Token![enum]) ||
              input.peek(Token![extern]) ||
              input.peek(Token![final]) ||
              input.peek(Token![fn]) ||
              input.peek(Token![for]) ||
              input.peek(Token![if]) ||
              input.peek(Token![impl]) ||
              input.peek(Token![in]) ||
              input.peek(Token![let]) ||
              input.peek(Token![loop]) ||
              input.peek(Token![macro]) ||
              input.peek(Token![match]) ||
              input.peek(Token![mod]) ||
              input.peek(Token![move]) ||
              input.peek(Token![mut]) ||
              input.peek(Token![override]) ||
              input.peek(Token![priv]) ||
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
              input.peek(Token![typeof]) ||
              input.peek(Token![unsafe]) ||
              input.peek(Token![unsized]) ||
              input.peek(Token![use]) ||
              input.peek(Token![virtual]) ||
              input.peek(Token![where]) ||
              input.peek(Token![while]) ||
              input.peek(Token![yield]) ||
              input.peek(Token![~]) ||
              input.peek(Token![!]) ||
              input.peek(Token![@]) ||
              input.peek(Token![#]) ||
              input.peek(Token![$]) ||
              input.peek(Token![%]) ||
              input.peek(Token![^]) ||
              input.peek(Token![&]) ||
              input.peek(Token![/]) ||
              input.peek(Token![*]) ||
              input.peek(Token![-]) ||
              input.peek(Token![+]) ||
              input.peek(Token![=]) ||
              input.peek(Token![|]) ||
              input.peek(Token![:]) ||
              input.peek(Token![;]) ||
              input.peek(Token![,]) ||
              input.peek(Token![.]) ||
              input.peek(Token![?]) ||
              (input.peek(Token![<]) && !input.peek2(Token![/])) {


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
        } else if input.peek(LitBool) {
           let b: LitBool = input.parse()?;
           Ok(XhtmlCrumb::S(format!("{}",b.value), b.span))
        } else if input.peek(LitInt) {
           let b: LitInt = input.parse()?;
           Ok(XhtmlCrumb::S(b.base10_digits().to_string(), b.span()))
        } else if input.peek(LitChar) {
           let b: LitChar = input.parse()?;
           Ok(XhtmlCrumb::S(format!("{}",b.value()), b.span()))
        } else if input.peek(LitStr) {
           let lit: LitStr = input.parse()?;
           Ok(XhtmlCrumb::L(lit))
        } else if input.peek(Token![<]) {
           let t: XhtmlTag = input.parse()?;
           Ok(XhtmlCrumb::T(t))
        } else if input.peek(Bracket) {
           let f: BracketedExpr = BracketedExpr::parse("markup".to_string(),input)?;
           Ok(XhtmlCrumb::F(f))
        } else if input.peek(Brace) {
           let e: XhtmlExpr = input.parse()?;
           Ok(XhtmlCrumb::E(e))
        } else if input.peek(Token![!]) {
           let id: Token![!] = input.parse()?;
           Ok(XhtmlCrumb::S("!".to_string(), id.span.clone()))
        } else if input.peek(Token![#]) {
           let id: Token![#] = input.parse()?;
           Ok(XhtmlCrumb::S("#".to_string(), id.span.clone()))
        } else if input.peek(Token![@]) {
           let id: Token![@] = input.parse()?;
           Ok(XhtmlCrumb::S("@".to_string(), id.span.clone()))
        } else if input.peek(Token![$]) {
           let id: Token![$] = input.parse()?;
           Ok(XhtmlCrumb::S("$".to_string(), id.span.clone()))
        } else if input.peek(Token![%]) {
           let id: Token![%] = input.parse()?;
           Ok(XhtmlCrumb::S("%".to_string(), id.span.clone()))
        } else if input.peek(Token![^]) {
           let id: Token![^] = input.parse()?;
           Ok(XhtmlCrumb::S("^".to_string(), id.span.clone()))
        } else if input.peek(Token![*]) {
           let id: Token![*] = input.parse()?;
           Ok(XhtmlCrumb::S("*".to_string(), id.span.clone()))
        } else if input.peek(Token![-]) {
           let id: Token![-] = input.parse()?;
           Ok(XhtmlCrumb::S("-".to_string(), id.span.clone()))
        } else if input.peek(Token![+]) {
           let id: Token![+] = input.parse()?;
           Ok(XhtmlCrumb::S("+".to_string(), id.span.clone()))
        } else if input.peek(Token![=]) {
           let id: Token![=] = input.parse()?;
           Ok(XhtmlCrumb::S("=".to_string(), id.span.clone()))
        } else if input.peek(Token![|]) {
           let id: Token![|] = input.parse()?;
           Ok(XhtmlCrumb::S("|".to_string(), id.span.clone()))
        } else if input.peek(Token![:]) {
           let id: Token![:] = input.parse()?;
           Ok(XhtmlCrumb::S(":".to_string(), id.span.clone()))
        } else if input.peek(Token![;]) {
           let id: Token![;] = input.parse()?;
           Ok(XhtmlCrumb::S(";".to_string(), id.span.clone()))
        } else if input.peek(Token![,]) {
           let id: Token![,] = input.parse()?;
           Ok(XhtmlCrumb::S(",".to_string(), id.span.clone()))
        } else if input.peek(Token![.]) {
           let id: Token![.] = input.parse()?;
           Ok(XhtmlCrumb::S(".".to_string(), id.span.clone()))
        } else if input.peek(Token![?]) {
           let id: Token![?] = input.parse()?;
           Ok(XhtmlCrumb::S("?".to_string(), id.span.clone()))
        } else if input.peek(Token![&]) {
           let id: Token![&] = input.parse()?;
           Ok(XhtmlCrumb::S("&".to_string(), id.span.clone()))
        } else if input.peek(Token![/]) {
           let id: Token![/] = input.parse()?;
           Ok(XhtmlCrumb::S("/".to_string(), id.span.clone()))
        } else if input.peek(Token![~]) {
           let id: Token![~] = input.parse()?;
           Ok(XhtmlCrumb::S("~".to_string(), id.span.clone()))
        } else if input.peek(Token![abstract]) {
           let id: Token![abstract] = input.parse()?;
           Ok(XhtmlCrumb::S("abstract".to_string(), id.span.clone()))
        } else if input.peek(Token![as]) {
           let id: Token![as] = input.parse()?;
           Ok(XhtmlCrumb::S("as".to_string(), id.span.clone()))
        } else if input.peek(Token![become]) {
           let id: Token![become] = input.parse()?;
           Ok(XhtmlCrumb::S("become".to_string(), id.span.clone()))
        } else if input.peek(Token![box]) {
           let id: Token![box] = input.parse()?;
           Ok(XhtmlCrumb::S("box".to_string(), id.span.clone()))
        } else if input.peek(Token![break]) {
           let id: Token![break] = input.parse()?;
           Ok(XhtmlCrumb::S("break".to_string(), id.span.clone()))
        } else if input.peek(Token![const]) {
           let id: Token![const] = input.parse()?;
           Ok(XhtmlCrumb::S("const".to_string(), id.span.clone()))
        } else if input.peek(Token![continue]) {
           let id: Token![continue] = input.parse()?;
           Ok(XhtmlCrumb::S("continue".to_string(), id.span.clone()))
        } else if input.peek(Token![crate]) {
           let id: Token![crate] = input.parse()?;
           Ok(XhtmlCrumb::S("crate".to_string(), id.span.clone()))
        } else if input.peek(Token![do]) {
           let id: Token![do] = input.parse()?;
           Ok(XhtmlCrumb::S("do".to_string(), id.span.clone()))
        } else if input.peek(Token![else]) {
           let id: Token![else] = input.parse()?;
           Ok(XhtmlCrumb::S("else".to_string(), id.span.clone()))
        } else if input.peek(Token![enum]) {
           let id: Token![enum] = input.parse()?;
           Ok(XhtmlCrumb::S("enum".to_string(), id.span.clone()))
        } else if input.peek(Token![extern]) {
           let id: Token![extern] = input.parse()?;
           Ok(XhtmlCrumb::S("extern".to_string(), id.span.clone()))
        } else if input.peek(Token![final]) {
           let id: Token![final] = input.parse()?;
           Ok(XhtmlCrumb::S("final".to_string(), id.span.clone()))
        } else if input.peek(Token![fn]) {
           let id: Token![fn] = input.parse()?;
           Ok(XhtmlCrumb::S("fn".to_string(), id.span.clone()))
        } else if input.peek(Token![for]) {
           let id: Token![for] = input.parse()?;
           Ok(XhtmlCrumb::S("for".to_string(), id.span.clone()))
        } else if input.peek(Token![if]) {
           let id: Token![if] = input.parse()?;
           Ok(XhtmlCrumb::S("if".to_string(), id.span.clone()))
        } else if input.peek(Token![impl]) {
           let id: Token![impl] = input.parse()?;
           Ok(XhtmlCrumb::S("impl".to_string(), id.span.clone()))
        } else if input.peek(Token![in]) {
           let id: Token![in] = input.parse()?;
           Ok(XhtmlCrumb::S("in".to_string(), id.span.clone()))
        } else if input.peek(Token![let]) {
           let id: Token![let] = input.parse()?;
           Ok(XhtmlCrumb::S("let".to_string(), id.span.clone()))
        } else if input.peek(Token![loop]) {
           let id: Token![loop] = input.parse()?;
           Ok(XhtmlCrumb::S("loop".to_string(), id.span.clone()))
        } else if input.peek(Token![macro]) {
           let id: Token![macro] = input.parse()?;
           Ok(XhtmlCrumb::S("macro".to_string(), id.span.clone()))
        } else if input.peek(Token![match]) {
           let id: Token![match] = input.parse()?;
           Ok(XhtmlCrumb::S("match".to_string(), id.span.clone()))
        } else if input.peek(Token![mod]) {
           let id: Token![mod] = input.parse()?;
           Ok(XhtmlCrumb::S("mod".to_string(), id.span.clone()))
        } else if input.peek(Token![move]) {
           let id: Token![move] = input.parse()?;
           Ok(XhtmlCrumb::S("move".to_string(), id.span.clone()))
        } else if input.peek(Token![mut]) {
           let id: Token![mut] = input.parse()?;
           Ok(XhtmlCrumb::S("mut".to_string(), id.span.clone()))
        } else if input.peek(Token![override]) {
           let id: Token![override] = input.parse()?;
           Ok(XhtmlCrumb::S("override".to_string(), id.span.clone()))
        } else if input.peek(Token![priv]) {
           let id: Token![priv] = input.parse()?;
           Ok(XhtmlCrumb::S("priv".to_string(), id.span.clone()))
        } else if input.peek(Token![pub]) {
           let id: Token![pub] = input.parse()?;
           Ok(XhtmlCrumb::S("pub".to_string(), id.span.clone()))
        } else if input.peek(Token![ref]) {
           let id: Token![ref] = input.parse()?;
           Ok(XhtmlCrumb::S("ref".to_string(), id.span.clone()))
        } else if input.peek(Token![return]) {
           let id: Token![return] = input.parse()?;
           Ok(XhtmlCrumb::S("return".to_string(), id.span.clone()))
        } else if input.peek(Token![self]) {
           let id: Token![self] = input.parse()?;
           Ok(XhtmlCrumb::S("self".to_string(), id.span.clone()))
        } else if input.peek(Token![Self]) {
           let id: Token![Self] = input.parse()?;
           Ok(XhtmlCrumb::S("Self".to_string(), id.span.clone()))
        } else if input.peek(Token![static]) {
           let id: Token![static] = input.parse()?;
           Ok(XhtmlCrumb::S("static".to_string(), id.span.clone()))
        } else if input.peek(Token![struct]) {
           let id: Token![struct] = input.parse()?;
           Ok(XhtmlCrumb::S("struct".to_string(), id.span.clone()))
        } else if input.peek(Token![super]) {
           let id: Token![super] = input.parse()?;
           Ok(XhtmlCrumb::S("super".to_string(), id.span.clone()))
        } else if input.peek(Token![trait]) {
           let id: Token![trait] = input.parse()?;
           Ok(XhtmlCrumb::S("trait".to_string(), id.span.clone()))
        } else if input.peek(Token![type]) {
           let id: Token![type] = input.parse()?;
           Ok(XhtmlCrumb::S("type".to_string(), id.span.clone()))
        } else if input.peek(Token![typeof]) {
           let id: Token![typeof] = input.parse()?;
           Ok(XhtmlCrumb::S("typeof".to_string(), id.span.clone()))
        } else if input.peek(Token![unsafe]) {
           let id: Token![unsafe] = input.parse()?;
           Ok(XhtmlCrumb::S("unsafe".to_string(), id.span.clone()))
        } else if input.peek(Token![unsized]) {
           let id: Token![unsized] = input.parse()?;
           Ok(XhtmlCrumb::S("unsized".to_string(), id.span.clone()))
        } else if input.peek(Token![use]) {
           let id: Token![use] = input.parse()?;
           Ok(XhtmlCrumb::S("use".to_string(), id.span.clone()))
        } else if input.peek(Token![virtual]) {
           let id: Token![virtual] = input.parse()?;
           Ok(XhtmlCrumb::S("virtual".to_string(), id.span.clone()))
        } else if input.peek(Token![where]) {
           let id: Token![where] = input.parse()?;
           Ok(XhtmlCrumb::S("where".to_string(), id.span.clone()))
        } else if input.peek(Token![while]) {
           let id: Token![while] = input.parse()?;
           Ok(XhtmlCrumb::S("while".to_string(), id.span.clone()))
        } else if input.peek(Token![yield]) {
           let id: Token![yield] = input.parse()?;
           Ok(XhtmlCrumb::S("yield".to_string(), id.span.clone()))
        } else {
           let id: Ident = input.parse()?;
           Ok(XhtmlCrumb::S(id.to_string(), id.span().clone()))
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
           XhtmlCrumb::L(l) => {
              let span = l.span().clone();
              (quote_spanned!{span=>
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
