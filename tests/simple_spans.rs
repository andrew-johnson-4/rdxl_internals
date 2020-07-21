
/*
use syn::{parse_quote,Token};
use quote::{quote,TokenStreamExt};

//assertion fails because LineColumn are default and equal
#[test]
fn span1() {
   let t: Token![!] = parse_quote! { ! };
   println!("{:?}:{:?}-{:?}", t.span, t.span.start(), t.span.end());
   assert_ne!(
      t.span.start(),
      t.span.end()
   )
}


//panics with: procedural macro API is used outside of a procedural macro
#[test]
fn span2() {
   let q = quote! { ! }.into();
   let t: Token![!] = syn::parse(q).unwrap();
   println!("{:?}:{:?}-{:?}", t.span, t.span.start(), t.span.end());
   assert_ne!(
      t.span.start(),
      t.span.end()
   )
}

#[test]
fn span3() {
   let q = quote! { ! };
   let t: Token![!] = syn::parse2(q).unwrap();
   println!("{:?}:{:?}-{:?}", t.span, t.span.start(), t.span.end());
   assert_ne!(
      t.span.start(),
      t.span.end()
   )
}

#[test]
fn span4() {
   let q = quote_precise::quote_precise! { ! };
   let t: Token![!] = syn::parse2(q).unwrap();
   println!("{:?}:{:?}-{:?}", t.span, t.span.start(), t.span.end());
   assert_ne!(
      t.span.start(),
      t.span.end()
   )
}
*/


