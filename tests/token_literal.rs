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

#[test]
fn token_literal26() {
   let t1: TokenAsLiteral = parse_quote! { as };
   assert_eq!( t1.token_literal, "as" );
}

#[test]
fn token_literal27() {
   let t1: TokenAsLiteral = parse_quote! { become };
   assert_eq!( t1.token_literal, "become" );
}

#[test]
fn token_literal28() {
   let t1: TokenAsLiteral = parse_quote! { box };
   assert_eq!( t1.token_literal, "box" );
}

#[test]
fn token_literal29() {
   let t1: TokenAsLiteral = parse_quote! { break };
   assert_eq!( t1.token_literal, "break" );
}

#[test]
fn token_literal30() {
   let t1: TokenAsLiteral = parse_quote! { const };
   assert_eq!( t1.token_literal, "const" );
}

#[test]
fn token_literal31() {
   let t1: TokenAsLiteral = parse_quote! { continue };
   assert_eq!( t1.token_literal, "continue" );
}

#[test]
fn token_literal32() {
   let t1: TokenAsLiteral = parse_quote! { crate };
   assert_eq!( t1.token_literal, "crate" );
}

#[test]
fn token_literal33() {
   let t1: TokenAsLiteral = parse_quote! { do };
   assert_eq!( t1.token_literal, "do" );
}

#[test]
fn token_literal34() {
   let t1: TokenAsLiteral = parse_quote! { else };
   assert_eq!( t1.token_literal, "else" );
}

#[test]
fn token_literal35() {
   let t1: TokenAsLiteral = parse_quote! { enum };
   assert_eq!( t1.token_literal, "enum" );
}

#[test]
fn token_literal36() {
   let t1: TokenAsLiteral = parse_quote! { extern };
   assert_eq!( t1.token_literal, "extern" );
}

#[test]
fn token_literal37() {
   let t1: TokenAsLiteral = parse_quote! { final };
   assert_eq!( t1.token_literal, "final" );
}

#[test]
fn token_literal38() {
   let t1: TokenAsLiteral = parse_quote! { fn };
   assert_eq!( t1.token_literal, "fn" );
}

#[test]
fn token_literal39() {
   let t1: TokenAsLiteral = parse_quote! { for };
   assert_eq!( t1.token_literal, "for" );
}

#[test]
fn token_literal40() {
   let t1: TokenAsLiteral = parse_quote! { if };
   assert_eq!( t1.token_literal, "if" );
}

#[test]
fn token_literal41() {
   let t1: TokenAsLiteral = parse_quote! { impl };
   assert_eq!( t1.token_literal, "impl" );
}

#[test]
fn token_literal42() {
   let t1: TokenAsLiteral = parse_quote! { in };
   assert_eq!( t1.token_literal, "in" );
}

#[test]
fn token_literal43() {
   let t1: TokenAsLiteral = parse_quote! { let };
   assert_eq!( t1.token_literal, "let" );
}

#[test]
fn token_literal44() {
   let t1: TokenAsLiteral = parse_quote! { loop };
   assert_eq!( t1.token_literal, "loop" );
}

#[test]
fn token_literal45() {
   let t1: TokenAsLiteral = parse_quote! { macro };
   assert_eq!( t1.token_literal, "macro" );
}

#[test]
fn token_literal46() {
   let t1: TokenAsLiteral = parse_quote! { match };
   assert_eq!( t1.token_literal, "match" );
}

#[test]
fn token_literal47() {
   let t1: TokenAsLiteral = parse_quote! { mod };
   assert_eq!( t1.token_literal, "mod" );
}

#[test]
fn token_literal48() {
   let t1: TokenAsLiteral = parse_quote! { move };
   assert_eq!( t1.token_literal, "move" );
}

#[test]
fn token_literal49() {
   let t1: TokenAsLiteral = parse_quote! { mut };
   assert_eq!( t1.token_literal, "mut" );
}

#[test]
fn token_literal50() {
   let t1: TokenAsLiteral = parse_quote! { override };
   assert_eq!( t1.token_literal, "override" );
}

#[test]
fn token_literal51() {
   let t1: TokenAsLiteral = parse_quote! { priv };
   assert_eq!( t1.token_literal, "priv" );
}

#[test]
fn token_literal52() {
   let t1: TokenAsLiteral = parse_quote! { pub };
   assert_eq!( t1.token_literal, "pub" );
}

#[test]
fn token_literal53() {
   let t1: TokenAsLiteral = parse_quote! { ref };
   assert_eq!( t1.token_literal, "ref" );
}

#[test]
fn token_literal54() {
   let t1: TokenAsLiteral = parse_quote! { return };
   assert_eq!( t1.token_literal, "return" );
}

#[test]
fn token_literal55() {
   let t1: TokenAsLiteral = parse_quote! { self };
   assert_eq!( t1.token_literal, "self" );
}

#[test]
fn token_literal56() {
   let t1: TokenAsLiteral = parse_quote! { Self };
   assert_eq!( t1.token_literal, "Self" );
}

#[test]
fn token_literal57() {
   let t1: TokenAsLiteral = parse_quote! { static };
   assert_eq!( t1.token_literal, "static" );
}

#[test]
fn token_literal58() {
   let t1: TokenAsLiteral = parse_quote! { struct };
   assert_eq!( t1.token_literal, "struct" );
}

#[test]
fn token_literal59() {
   let t1: TokenAsLiteral = parse_quote! { super };
   assert_eq!( t1.token_literal, "super" );
}

#[test]
fn token_literal60() {
   let t1: TokenAsLiteral = parse_quote! { trait };
   assert_eq!( t1.token_literal, "trait" );
}

#[test]
fn token_literal61() {
   let t1: TokenAsLiteral = parse_quote! { type };
   assert_eq!( t1.token_literal, "type" );
}

#[test]
fn token_literal62() {
   let t1: TokenAsLiteral = parse_quote! { typeof };
   assert_eq!( t1.token_literal, "typeof" );
}

#[test]
fn token_literal63() {
   let t1: TokenAsLiteral = parse_quote! { unsafe };
   assert_eq!( t1.token_literal, "unsafe" );
}

#[test]
fn token_literal64() {
   let t1: TokenAsLiteral = parse_quote! { unsized };
   assert_eq!( t1.token_literal, "unsized" );
}

#[test]
fn token_literal65() {
   let t1: TokenAsLiteral = parse_quote! { use };
   assert_eq!( t1.token_literal, "use" );
}

#[test]
fn token_literal66() {
   let t1: TokenAsLiteral = parse_quote! { virtual };
   assert_eq!( t1.token_literal, "virtual" );
}

#[test]
fn token_literal67() {
   let t1: TokenAsLiteral = parse_quote! { where };
   assert_eq!( t1.token_literal, "where" );
}

#[test]
fn token_literal68() {
   let t1: TokenAsLiteral = parse_quote! { while };
   assert_eq!( t1.token_literal, "while" );
}

#[test]
fn token_literal69() {
   let t1: TokenAsLiteral = parse_quote! { yield };
   assert_eq!( t1.token_literal, "yield" );
}

#[test]
fn token_literal70() {
   let t1: TokenAsLiteral = parse_quote! { snake_case };
   assert_eq!( t1.token_literal, "snake_case" );
}

#[test]
fn token_literal71() {
   let t1: TokenAsLiteral = parse_quote! { camel_case };
   assert_eq!( t1.token_literal, "camel_case" );
}

#[test]
fn token_literal72() {
   let t1: TokenAsLiteral = parse_quote! { SCREAMING_SNAKE_CASE };
   assert_eq!( t1.token_literal, "SCREAMING_SNAKE_CASE" );
}

#[test]
#[should_panic]
fn token_literal73() {
   let t1: TokenAsLiteral = parse_quote! { SCREAMING_SNAKE_CASE };
   assert_eq!( t1.token_literal, "SCREAMING_SNAKE_CASe" );
}

