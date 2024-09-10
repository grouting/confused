use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse::Parse, parse_macro_input, Expr, LitInt, Token, Type};

struct Confused {
	number: u8,
	ok: Type,
	err: Option<Type>,
}

impl Parse for Confused {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let number_literal: LitInt = input.parse()?;
		let number = number_literal.base10_parse::<u8>()?;

		input.parse::<Token![,]>()?;

		let ok: Type = input.parse()?;

		let err = if input.parse::<Token![,]>().is_ok() {
			Some(input.parse::<Type>()?)
		} else {
			None
		};

		Ok(Self { number, ok, err })
	}
}

/// # Usage
/// ```
/// fn what() -> confused!(3, bool) { } // fn what() -> Result<Result<Result<bool, ()>, ()>, ()> { }
/// fn what() -> confused!(3, bool, bool) { } // fn what() -> Result<Result<Result<bool, bool>, bool>, bool> { }
/// ```
#[proc_macro]
pub fn confused(input: TokenStream) -> TokenStream {
	let config = parse_macro_input!(input as Confused);

	let ok_type_name = config.ok.to_token_stream().to_string();
	let err_type_name = config.err.map(|err| err.to_token_stream().to_string());

	let mut out = String::new();

	for _ in 0..config.number {
		out.push_str("::core::result::Result<");
	}

	out.push_str(&ok_type_name);

	for _ in 0..config.number {
		if let Some(ref err) = err_type_name {
			out.push_str(&format!(", {err}>"));
		} else {
			out.push_str(", ()>");
		}
	}

	out.parse().unwrap()
}

struct NestedOk(u8, Expr);

impl Parse for NestedOk {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let number_literal: LitInt = input.parse()?;
		let number = number_literal.base10_parse::<u8>()?;

		input.parse::<Token![,]>()?;

		let expression: Expr = input.parse()?;

		Ok(Self(number, expression))
	}
}

/// # Usage
/// ```
/// let out = confusion!(3, true); // Ok(Ok(Ok(true)))
/// ```
#[proc_macro]
pub fn confusion(input: TokenStream) -> TokenStream {
	let number = parse_macro_input!(input as NestedOk);

	let mut out = String::new();

	for _ in 0..number.0 {
		out.push_str("::core::result::Result::Ok(");
	}

	out.push_str(&number.1.into_token_stream().to_string());

	for _ in 0..number.0 {
		out.push_str(")");
	}

	out.parse().unwrap()
}
