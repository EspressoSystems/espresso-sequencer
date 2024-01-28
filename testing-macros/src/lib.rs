//! Macros for testing over all network implementations and nodetype implementations

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::parse::Result;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, ExprArray, ExprPath, ExprTuple, Ident, LitBool, Token,
};

/// description of a crosstest
#[derive(derive_builder::Builder, Debug, Clone)]
struct CrossTestData {
    /// imlementations
    impls: ExprArray,
    /// types
    types: ExprArray,
    /// name of the test
    test_name: Ident,
    /// test description/spec
    metadata: Expr,
    /// whether or not to ignore
    ignore: LitBool,
}

impl CrossTestDataBuilder {
    /// if we've extracted all the metadata
    fn is_ready(&self) -> bool {
        self.impls.is_some()
            && self.types.is_some()
            && self.test_name.is_some()
            && self.metadata.is_some()
            && self.test_name.is_some()
            && self.ignore.is_some()
    }
}

/// requisite data to generate a single test
#[derive(derive_builder::Builder, Debug, Clone)]
struct TestData {
    /// type
    ty: ExprPath,
    /// impl
    imply: ExprPath,
    /// name of test
    test_name: Ident,
    /// test description
    metadata: Expr,
    /// whether or not to ignore the test
    ignore: LitBool,
}

/// trait make a string lower and snake case
trait ToLowerSnakeStr {
    /// make a lower and snake case string
    fn to_lower_snake_str(&self) -> String;
}

impl ToLowerSnakeStr for ExprPath {
    fn to_lower_snake_str(&self) -> String {
        self.path
            .segments
            .iter()
            .fold(String::new(), |mut acc, s| {
                acc.push_str(&s.ident.to_string().to_lowercase());
                acc.push('_');
                acc
            })
            .to_lowercase()
    }
}

impl ToLowerSnakeStr for ExprTuple {
    /// allow panic because this is a compiler error
    #[allow(clippy::panic)]
    fn to_lower_snake_str(&self) -> String {
        self.elems
            .iter()
            .map(|x| {
                let Expr::Path(expr_path) = x else {
                    panic!("Expected path expr, got {x:?}");
                };
                expr_path
            })
            .fold(String::new(), |mut acc, s| {
                acc.push_str(&s.to_lower_snake_str());
                acc
            })
    }
}

impl TestData {
    /// generate the code for a single test
    fn generate_test(&self) -> TokenStream2 {
        let TestData {
            ty,
            imply,
            test_name,
            metadata,
            ignore,
        } = self;

        let slow_attribute = if ignore.value() {
            // quote! { #[cfg(feature = "slow-tests")] }
            quote! { #[ignore] }
        } else {
            quote! {}
        };
        quote! {
            #[cfg(test)]
            #slow_attribute
            #[cfg_attr(
                async_executor_impl = "tokio",
                tokio::test(flavor = "multi_thread", worker_threads = 2)
            )]
            #[cfg_attr(async_executor_impl = "async-std", async_std::test)]
            #[tracing::instrument]
            async fn #test_name() {
                async_compatibility_layer::logging::setup_logging();
                async_compatibility_layer::logging::setup_backtrace();
                (#metadata).gen_launcher::<#ty, #imply>(0).launch().run_test().await;
            }
        }
    }
}

/// macro specific custom keywords
mod keywords {
    syn::custom_keyword!(Metadata);
    syn::custom_keyword!(Ignore);
    syn::custom_keyword!(TestName);
    syn::custom_keyword!(Types);
    syn::custom_keyword!(Impls);
}

impl Parse for CrossTestData {
    /// allow panic because this is a compiler error
    #[allow(clippy::panic)]
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut description = CrossTestDataBuilder::create_empty();

        while !description.is_ready() {
            if input.peek(keywords::Types) {
                let _ = input.parse::<keywords::Types>()?;
                input.parse::<Token![:]>()?;
                let types = input.parse::<ExprArray>()?;
                description.types(types);
            } else if input.peek(keywords::Impls) {
                let _ = input.parse::<keywords::Impls>()?;
                input.parse::<Token![:]>()?;
                let impls = input.parse::<ExprArray>()?;
                description.impls(impls);
            } else if input.peek(keywords::TestName) {
                let _ = input.parse::<keywords::TestName>()?;
                input.parse::<Token![:]>()?;
                let test_name = input.parse::<Ident>()?;
                description.test_name(test_name);
            } else if input.peek(keywords::Metadata) {
                let _ = input.parse::<keywords::Metadata>()?;
                input.parse::<Token![:]>()?;
                let metadata = input.parse::<Expr>()?;
                description.metadata(metadata);
            } else if input.peek(keywords::Ignore) {
                let _ = input.parse::<keywords::Ignore>()?;
                input.parse::<Token![:]>()?;
                let ignore = input.parse::<LitBool>()?;
                description.ignore(ignore);
            } else {
                panic!(
                    "Unexpected token. Expected one of: Metadata, Ignore, Impls, Types, Testname"
                );
            }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        description
            .build()
            .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), format!("{e}")))
    }
}

/// Helper function to do the actual code gen
/// allow panic because this is a compiler error
#[allow(clippy::panic)]
fn cross_tests_internal(test_spec: CrossTestData) -> TokenStream {
    let impls = test_spec.impls.elems.iter().map(|t| {
        let Expr::Path(p) = t else {
            panic!("Expected Path for Impl! Got {t:?}");
        };
        p
    });
    //
    let types = test_spec.types.elems.iter().map(|t| {
        let Expr::Path(p) = t else {
            panic!("Expected Path for Type! Got {t:?}");
        };
        p
    });

    let mut result = quote! {};
    for ty in types.clone() {
        let mut type_mod = quote! {};
        for imp in impls.clone() {
            let test_data = TestDataBuilder::create_empty()
                .test_name(test_spec.test_name.clone())
                .metadata(test_spec.metadata.clone())
                .ignore(test_spec.ignore.clone())
                .imply(imp.clone())
                .ty(ty.clone())
                .build()
                .unwrap();
            let test = test_data.generate_test();

            let impl_str = format_ident!("{}", imp.to_lower_snake_str());
            let impl_result = quote! {
                pub mod #impl_str {
                    use super::*;
                    #test
                }
            };
            type_mod.extend(impl_result);
        }
        let ty_str = format_ident!("{}", ty.to_lower_snake_str());
        let typ_result = quote! {
            pub mod #ty_str {
                use super::*;
                #type_mod
            }
        };
        result.extend(typ_result);
    }
    let name = test_spec.test_name;
    quote! {
        pub mod #name {
            use super::*;
            #result
        }
    }
    .into()
}

/// Generate a cartesian product of tests across all types
/// Arguments:
/// - `Impls: []` - a list of types that implement nodetype
/// - `Metadata`: `TestMetadata::default()` - test metadata
/// - `Types: []` - a list types that implement `NodeImplementation` over the types in `Impls`
/// - `TestName: example_test` - the name of the test
/// - `Ignore`: whether or not this set of tests are ignored
/// Example usage: see tests in this module
#[proc_macro]
pub fn cross_tests(input: TokenStream) -> TokenStream {
    let test_spec = parse_macro_input!(input as CrossTestData);
    cross_tests_internal(test_spec)
}
