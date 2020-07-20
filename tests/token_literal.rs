use rdxl_internals::core::TokenAsLiteral;
use syn::parse_quote;

#[test]
fn token_literal1() {
   let t1: TokenAsLiteral = parse_quote! {
      "string"
   };
   assert_eq!(
      t1.token_literal,
      "string"
   );
}

#[test]
fn token_literal2() {
   let t1: TokenAsLiteral = parse_quote! {
      123
   };
   assert_eq!(
      t1.token_literal,
      "123"
   );
}
