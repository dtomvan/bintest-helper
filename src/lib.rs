//! # Bintest Helper
//! Use this in combination with [assert_cmd](https://github.com/assert-rs/assert_cmd).
//! Example:
//! ```ignore
//! fn main() {
//!     println!("Hello, World!");
//! }
//!
//! #[cfg(test)]
//! mod tests {
//!     #[bintest_helper::bintest_helper]
//!     #[test]
//!     fn main_test() {
//!         use assert_cmd::Command;
//! 
//!         let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
//!         cmd.assert().stdout("Hello, World!");
//!     }
//! }
//! ```
//! See also: the `examples` directory.
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::parse_quote;
use syn::Item;

#[proc_macro_attribute]
/// Add this to your unit-tests to (re)build your binary, if performing bintests.
pub fn bintest_helper(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ty = parse_macro_input!(input as Item);
    assert!(args.is_empty());
    if let Item::Fn(fun) = &ty {
        let mut fun = fun.clone();
        let is_test = fun
            .attrs
            .iter()
            .find(|x| {
                x.path
                    .segments
                    .first()
                    .expect("Invalid test function: attributes are empty.")
                    .ident
                    .to_string()
                    == String::from("test")
            })
            .is_some();
        assert!(is_test, "You can only use this macro on a unit test. Did you put this macro above `#[test]`?");
        let fn_path = fun.sig.ident.to_string();

        fun.block.stmts.insert(0, parse_quote! {
            use std::io::Write;
        });
        fun.block.stmts.insert(1, parse_quote! {
            ::std::io::stderr()
                .lock()
                .write_all(format!("[{}::{}] BUILDING MAIN EXECUTABLE\n", module_path!(), #fn_path).as_bytes())
                .unwrap();
        });
        fun.block.stmts.insert(
            2,
            parse_quote! {
                ::std::process::Command::new("cargo")
                    .current_dir(env!("CARGO_MANIFEST_DIR"))
                    .arg("build")
                    .spawn()
                    .expect("Couldn't start cargo. Did you install a rust toolchain?")
                    .wait()
                    .expect("The compiler unexpectedly stopped working");
            },
        );
        ty = Item::Fn(fun);
    } else {
        panic!(
            "You can only use this macro on an fn.\nExpected: Fn\nFound: {:?}",
            ty
        );
    }

    proc_macro::TokenStream::from(quote! { #ty })
}
