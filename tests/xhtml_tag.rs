use rdxl_internals::xhtml::XhtmlTag;
use syn::parse_quote;

#[test]
fn tag1() {
   let t: XhtmlTag = parse_quote! { <div></div> };
   assert_eq!(
      t.inner_span_start.end(),
      t.inner_span_end.start()
   )
}

/*
#[test]
fn tag2() {
   let t: XhtmlTag = parse_quote! { <div> </div> };
   println!(".end(): {:?}", t.inner_span_start.end());
   println!(".start(): {:?}", t.inner_span_end.start());
   assert!(
      t.inner_span_start.end() <
      t.inner_span_end.start()
   )
}
*/
