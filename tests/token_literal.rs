use rdxl_internals::core::TokenAsLiteral;
use syn::parse_quote;

#[test]
fn token_literal1() {
   let t1: TokenAsLiteral = parse_quote! { "string" };
   assert_eq!( t1.token_literal, "string" );
}

#[test]
fn token_literal2() {
   let t1: TokenAsLiteral = parse_quote! { true };
   assert_eq!( t1.token_literal, "true" );
}

#[test]
fn token_literal3() {
   let t1: TokenAsLiteral = parse_quote! { 123 };
   assert_eq!( t1.token_literal, "123" );
}

#[test]
fn token_literal4() {
   let t1: TokenAsLiteral = parse_quote! { 'd' };
   assert_eq!( t1.token_literal, "d" );
}

#[test]
fn token_literal5() {
   let t1: TokenAsLiteral = parse_quote! { < };
   assert_eq!( t1.token_literal, "<" );
}

#[test]
fn token_literal6() {
   let t1: TokenAsLiteral = parse_quote! { ! };
   assert_eq!( t1.token_literal, "!" );
}

#[test]
fn token_literal7() {
   let t1: TokenAsLiteral = parse_quote! { # };
   assert_eq!( t1.token_literal, "#" );
}

#[test]
fn token_literal8() {
   let t1: TokenAsLiteral = parse_quote! { @ };
   assert_eq!( t1.token_literal, "@" );
}

#[test]
fn token_literal9() {
   let t1: TokenAsLiteral = parse_quote! { $ };
   assert_eq!( t1.token_literal, "$" );
}

#[test]
fn token_literal10() {
   let t1: TokenAsLiteral = parse_quote! { % };
   assert_eq!( t1.token_literal, "%" );
}

#[test]
fn token_literal11() {
   let t1: TokenAsLiteral = parse_quote! { ^ };
   assert_eq!( t1.token_literal, "^" );
}

#[test]
fn token_literal12() {
   let t1: TokenAsLiteral = parse_quote! { * };
   assert_eq!( t1.token_literal, "*" );
}

#[test]
fn token_literal13() {
   let t1: TokenAsLiteral = parse_quote! { - };
   assert_eq!( t1.token_literal, "-" );
}

#[test]
fn token_literal14() {
   let t1: TokenAsLiteral = parse_quote! { + };
   assert_eq!( t1.token_literal, "+" );
}

#[test]
fn token_literal15() {
   let t1: TokenAsLiteral = parse_quote! { = };
   assert_eq!( t1.token_literal, "=" );
}

#[test]
fn token_literal16() {
   let t1: TokenAsLiteral = parse_quote! { | };
   assert_eq!( t1.token_literal, "|" );
}

#[test]
fn token_literal17() {
   let t1: TokenAsLiteral = parse_quote! { : };
   assert_eq!( t1.token_literal, ":" );
}

#[test]
fn token_literal18() {
   let t1: TokenAsLiteral = parse_quote! { ; };
   assert_eq!( t1.token_literal, ";" );
}

#[test]
fn token_literal19() {
   let t1: TokenAsLiteral = parse_quote! { , };
   assert_eq!( t1.token_literal, "," );
}

#[test]
fn token_literal20() {
   let t1: TokenAsLiteral = parse_quote! { . };
   assert_eq!( t1.token_literal, "." );
}

#[test]
fn token_literal21() {
   let t1: TokenAsLiteral = parse_quote! { ? };
   assert_eq!( t1.token_literal, "?" );
}

#[test]
fn token_literal22() {
   let t1: TokenAsLiteral = parse_quote! { & };
   assert_eq!( t1.token_literal, "&" );
}

#[test]
fn token_literal23() {
   let t1: TokenAsLiteral = parse_quote! { / };
   assert_eq!( t1.token_literal, "/" );
}

#[test]
fn token_literal24() {
   let t1: TokenAsLiteral = parse_quote! { ~ };
   assert_eq!( t1.token_literal, "~" );
}

#[test]
fn token_literal25() {
   let t1: TokenAsLiteral = parse_quote! { abstract };
   assert_eq!( t1.token_literal, "abstract" );
}

/*
        } else if input.peek(Token![as]) {
           let id: Token![as] = input.parse()?;
           Ok(TokenAsLiteral::new("as".to_string(), id.span.clone()))
        } else if input.peek(Token![become]) {
           let id: Token![become] = input.parse()?;
           Ok(TokenAsLiteral::new("become".to_string(), id.span.clone()))
        } else if input.peek(Token![box]) {
           let id: Token![box] = input.parse()?;
           Ok(TokenAsLiteral::new("box".to_string(), id.span.clone()))
        } else if input.peek(Token![break]) {
           let id: Token![break] = input.parse()?;
           Ok(TokenAsLiteral::new("break".to_string(), id.span.clone()))
        } else if input.peek(Token![const]) {
           let id: Token![const] = input.parse()?;
           Ok(TokenAsLiteral::new("const".to_string(), id.span.clone()))
        } else if input.peek(Token![continue]) {
           let id: Token![continue] = input.parse()?;
           Ok(TokenAsLiteral::new("continue".to_string(), id.span.clone()))
        } else if input.peek(Token![crate]) {
           let id: Token![crate] = input.parse()?;
           Ok(TokenAsLiteral::new("crate".to_string(), id.span.clone()))
        } else if input.peek(Token![do]) {
           let id: Token![do] = input.parse()?;
           Ok(TokenAsLiteral::new("do".to_string(), id.span.clone()))
        } else if input.peek(Token![else]) {
           let id: Token![else] = input.parse()?;
           Ok(TokenAsLiteral::new("else".to_string(), id.span.clone()))
        } else if input.peek(Token![enum]) {
           let id: Token![enum] = input.parse()?;
           Ok(TokenAsLiteral::new("enum".to_string(), id.span.clone()))
        } else if input.peek(Token![extern]) {
           let id: Token![extern] = input.parse()?;
           Ok(TokenAsLiteral::new("extern".to_string(), id.span.clone()))
        } else if input.peek(Token![final]) {
           let id: Token![final] = input.parse()?;
           Ok(TokenAsLiteral::new("final".to_string(), id.span.clone()))
        } else if input.peek(Token![fn]) {
           let id: Token![fn] = input.parse()?;
           Ok(TokenAsLiteral::new("fn".to_string(), id.span.clone()))
        } else if input.peek(Token![for]) {
           let id: Token![for] = input.parse()?;
           Ok(TokenAsLiteral::new("for".to_string(), id.span.clone()))
        } else if input.peek(Token![if]) {
           let id: Token![if] = input.parse()?;
           Ok(TokenAsLiteral::new("if".to_string(), id.span.clone()))
        } else if input.peek(Token![impl]) {
           let id: Token![impl] = input.parse()?;
           Ok(TokenAsLiteral::new("impl".to_string(), id.span.clone()))
        } else if input.peek(Token![in]) {
           let id: Token![in] = input.parse()?;
           Ok(TokenAsLiteral::new("in".to_string(), id.span.clone()))
        } else if input.peek(Token![let]) {
           let id: Token![let] = input.parse()?;
           Ok(TokenAsLiteral::new("let".to_string(), id.span.clone()))
        } else if input.peek(Token![loop]) {
           let id: Token![loop] = input.parse()?;
           Ok(TokenAsLiteral::new("loop".to_string(), id.span.clone()))
        } else if input.peek(Token![macro]) {
           let id: Token![macro] = input.parse()?;
           Ok(TokenAsLiteral::new("macro".to_string(), id.span.clone()))
        } else if input.peek(Token![match]) {
           let id: Token![match] = input.parse()?;
           Ok(TokenAsLiteral::new("match".to_string(), id.span.clone()))
        } else if input.peek(Token![mod]) {
           let id: Token![mod] = input.parse()?;
           Ok(TokenAsLiteral::new("mod".to_string(), id.span.clone()))
        } else if input.peek(Token![move]) {
           let id: Token![move] = input.parse()?;
           Ok(TokenAsLiteral::new("move".to_string(), id.span.clone()))
        } else if input.peek(Token![mut]) {
           let id: Token![mut] = input.parse()?;
           Ok(TokenAsLiteral::new("mut".to_string(), id.span.clone()))
        } else if input.peek(Token![override]) {
           let id: Token![override] = input.parse()?;
           Ok(TokenAsLiteral::new("override".to_string(), id.span.clone()))
        } else if input.peek(Token![priv]) {
           let id: Token![priv] = input.parse()?;
           Ok(TokenAsLiteral::new("priv".to_string(), id.span.clone()))
        } else if input.peek(Token![pub]) {
           let id: Token![pub] = input.parse()?;
           Ok(TokenAsLiteral::new("pub".to_string(), id.span.clone()))
        } else if input.peek(Token![ref]) {
           let id: Token![ref] = input.parse()?;
           Ok(TokenAsLiteral::new("ref".to_string(), id.span.clone()))
        } else if input.peek(Token![return]) {
           let id: Token![return] = input.parse()?;
           Ok(TokenAsLiteral::new("return".to_string(), id.span.clone()))
        } else if input.peek(Token![self]) {
           let id: Token![self] = input.parse()?;
           Ok(TokenAsLiteral::new("self".to_string(), id.span.clone()))
        } else if input.peek(Token![Self]) {
           let id: Token![Self] = input.parse()?;
           Ok(TokenAsLiteral::new("Self".to_string(), id.span.clone()))
        } else if input.peek(Token![static]) {
           let id: Token![static] = input.parse()?;
           Ok(TokenAsLiteral::new("static".to_string(), id.span.clone()))
        } else if input.peek(Token![struct]) {
           let id: Token![struct] = input.parse()?;
           Ok(TokenAsLiteral::new("struct".to_string(), id.span.clone()))
        } else if input.peek(Token![super]) {
           let id: Token![super] = input.parse()?;
           Ok(TokenAsLiteral::new("super".to_string(), id.span.clone()))
        } else if input.peek(Token![trait]) {
           let id: Token![trait] = input.parse()?;
           Ok(TokenAsLiteral::new("trait".to_string(), id.span.clone()))
        } else if input.peek(Token![type]) {
           let id: Token![type] = input.parse()?;
           Ok(TokenAsLiteral::new("type".to_string(), id.span.clone()))
        } else if input.peek(Token![typeof]) {
           let id: Token![typeof] = input.parse()?;
           Ok(TokenAsLiteral::new("typeof".to_string(), id.span.clone()))
        } else if input.peek(Token![unsafe]) {
           let id: Token![unsafe] = input.parse()?;
           Ok(TokenAsLiteral::new("unsafe".to_string(), id.span.clone()))
        } else if input.peek(Token![unsized]) {
           let id: Token![unsized] = input.parse()?;
           Ok(TokenAsLiteral::new("unsized".to_string(), id.span.clone()))
        } else if input.peek(Token![use]) {
           let id: Token![use] = input.parse()?;
           Ok(TokenAsLiteral::new("use".to_string(), id.span.clone()))
        } else if input.peek(Token![virtual]) {
           let id: Token![virtual] = input.parse()?;
           Ok(TokenAsLiteral::new("virtual".to_string(), id.span.clone()))
        } else if input.peek(Token![where]) {
           let id: Token![where] = input.parse()?;
           Ok(TokenAsLiteral::new("where".to_string(), id.span.clone()))
        } else if input.peek(Token![while]) {
           let id: Token![while] = input.parse()?;
           Ok(TokenAsLiteral::new("while".to_string(), id.span.clone()))
        } else if input.peek(Token![yield]) {
           let id: Token![yield] = input.parse()?;
           Ok(TokenAsLiteral::new("yield".to_string(), id.span.clone()))
        } else {
           let id: Ident = input.parse()?;
           Ok(TokenAsLiteral::new(id.to_string(), id.span().clone()))
        }
*/
