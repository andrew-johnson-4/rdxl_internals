use rdxl_internals::xhtml::Xhtml;
use syn::parse_quote;

#[test]
fn token_literal1() {
   let _: Xhtml = parse_quote! { <div>a b c do</div> };
}

#[test]
fn token_literal2() {
   let _: Xhtml = parse_quote! { <div>{{ "abc" }}</div> };
}
