//! # AutoSized Int Macros
//!
//! This crate provides a set of procedural macros that automatically select
//! the smallest integer type capable of representing a given literal.
//!
//! - `auto_sized_unsigned!` / `auto_sized_unsigned_val!`  
//!   → Choose among unsigned integers (`u8`, `u16`, `u32`, `u64`, `u128`).
//!
//! - `auto_sized_signed!` / `auto_sized_signed_val!`  
//!   → Choose among signed integers (`i8`, `i16`, `i32`, `i64`, `i128`).
//!
//! - `auto_sized_int!` / `auto_sized_int_val!`  
//!   → If the literal is negative, a signed type is chosen.  
//!   If the literal is non‑negative, an unsigned type is chosen.  
//!   **The accepted input range is the full `i128` domain (not `u128`).**
//!
//! ## Type vs. Value Macros
//! - `*_unsigned!`, `*_signed!`, `*_int!` → expand to a **type**.
//! - `*_val` variants → expand to a **value** (with an explicit `as` cast).
//!
//! ## Examples
//! ```rust
//! use autosized_num::*;
//!
//! // Type macros
//! type T1 = auto_sized_unsigned!(300); // expands to u16
//! type T2 = auto_sized_signed!(-200);  // expands to i16
//! type T3 = auto_sized_int!(10);       // expands to u8
//! type T4 = auto_sized_int!(-10);      // expands to i8
//!
//! // Value macros
//! let a = auto_sized_unsigned_val!(300); // 300u16
//! let b = auto_sized_signed_val!(-200);  // -200i16
//! let c = auto_sized_int_val!(10);       // 10u8
//! let d = auto_sized_int_val!(-10);      // -10i8
//! ```
//!
//! ## Intended Use Cases
//! - Binary parsing or serialization where minimal integer widths matter.
//! - Defining constants or generic parameters with the smallest fitting type.
//! - Compile‑time tests ensuring literals map to the expected integer type.
//!
//! ## Notes
//! - `auto_sized_int!` and `auto_sized_int_val!` accept the full `i128` range.
//! - Non‑integer inputs will trigger a `compile_error!`.

use proc_macro::TokenStream;
use quote::quote;
use syn::{LitInt, parse_macro_input};

/// Returns the smallest unsigned integer type (`u8`, `u16`, `u32`, `u64`, or `u128`)
/// that can represent the given literal.
///
/// # Examples
/// ```
/// use autosized_num::auto_sized_unsigned;
///
/// type T = auto_sized_unsigned!(300);
/// // expands to: type T = u16;
/// ```
#[proc_macro]
pub fn auto_sized_unsigned(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitInt);
    let value = match lit.base10_parse::<u128>() {
        Ok(v) => v,
        Err(_) => {
            return quote! {
                compile_error!("auto_sized_unsign! only accepts integer literals");
            }
            .into();
        }
    };

    pick_unsigned_type(value).into()
}

/// Returns the given literal as a value, cast to the smallest unsigned integer type
/// that can represent it.
///
/// # Examples
/// ```
/// use autosized_num::auto_sized_unsigned_val;
///
/// let x = auto_sized_unsigned_val!(300);
/// // expands to: 300 as u16
/// ```
#[proc_macro]
pub fn auto_sized_unsigned_val(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitInt);
    let value = match lit.base10_parse::<u128>() {
        Ok(v) => v,
        Err(_) => {
            return quote! {
                compile_error!("auto_sized_unsign_val! only accepts integer literals");
            }
            .into();
        }
    };

    let ty = pick_unsigned_type(value);

    quote! { #value as #ty }.into()
}

/// Returns the smallest signed integer type (`i8`, `i16`, `i32`, `i64`, or `i128`)
/// that can represent the given literal.
///
/// # Examples
/// ```
/// use autosized_num::auto_sized_signed;
///
/// type T = auto_sized_signed!(-200);
/// // expands to: type T = i16;
/// ```
#[proc_macro]
pub fn auto_sized_signed(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitInt);
    let value = match lit.base10_parse::<i128>() {
        Ok(v) => v,
        Err(_) => {
            return quote! {
                compile_error!("auto_sized_sign! only accepts integer literals");
            }
            .into();
        }
    };

    pick_signed_type(value).into()
}

/// Returns the given literal as a value, cast to the smallest signed integer type
/// that can represent it.
///
/// # Examples
/// ```
/// use autosized_num::auto_sized_signed_val;
///
/// let y = auto_sized_signed_val!(-200);
/// // expands to: -200 as i16
/// ```
#[proc_macro]
pub fn auto_sized_signed_val(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitInt);
    let value = match lit.base10_parse::<i128>() {
        Ok(v) => v,
        Err(_) => {
            return quote! {
                compile_error!("auto_sized_sign_val! only accepts integer literals");
            }
            .into();
        }
    };

    let ty = pick_signed_type(value);

    quote! { #value as #ty }.into()
}

/// Returns the smallest integer type (signed or unsigned) that can represent the given literal.
/// - If the literal is negative, a signed type is chosen.
/// - If the literal is non-negative, an unsigned type is chosen.
/// - The accepted range of input is the full `i128` range (not `u128`).
///
/// # Examples
/// ```
/// use autosized_num::auto_sized_int;
///
/// type T1 = auto_sized_int!(10);   // expands to u8
/// type T2 = auto_sized_int!(-10);  // expands to i8
/// type T3 = auto_sized_int!(12345678901234567890); // expands to u64/u128 depending on value
/// ```
#[proc_macro]
pub fn auto_sized_int(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitInt);
    let value = match lit.base10_parse::<i128>() {
        Ok(v) => v,
        Err(_) => {
            return quote! {
                compile_error!("auto_sized_int! only accepts integer literals");
            }
            .into();
        }
    };

    if value < 0 {
        pick_signed_type(value)
    } else {
        pick_unsigned_type(value as u128)
    }
    .into()
}

/// Returns the given literal as a value, cast to the smallest integer type
/// (signed or unsigned) that can represent it.
/// - If the literal is negative, a signed type is chosen.
/// - If the literal is non-negative, an unsigned type is chosen.
/// - The accepted range of input is the full `i128` range (not `u128`).
///
/// # Examples
/// ```
/// use autosized_num::auto_sized_int_val;
///
/// let a = auto_sized_int_val!(10);   // expands to 10 as u8
/// let b = auto_sized_int_val!(-10);  // expands to -10 as i8
/// let c = auto_sized_int_val!(12345678901234567890); // expands to value as u64/u128
/// ```
#[proc_macro]
pub fn auto_sized_int_val(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitInt);
    let value = match lit.base10_parse::<i128>() {
        Ok(v) => v,
        Err(_) => {
            return quote! {
                compile_error!("auto_sized_int_val! only accepts integer literals");
            }
            .into();
        }
    };

    let ty = if value < 0 {
        pick_signed_type(value)
    } else {
        pick_unsigned_type(value as u128)
    };

    quote! { #value as #ty }.into()
}

fn pick_unsigned_type(value: u128) -> proc_macro2::TokenStream {
    if value <= u8::MAX as u128 {
        quote! { u8 }
    } else if value <= u16::MAX as u128 {
        quote! { u16 }
    } else if value <= u32::MAX as u128 {
        quote! { u32 }
    } else if value <= u64::MAX as u128 {
        quote! { u64 }
    } else {
        quote! { u128 }
    }
}

fn pick_signed_type(value: i128) -> proc_macro2::TokenStream {
    if value >= i8::MIN as i128 && value <= i8::MAX as i128 {
        quote! { i8 }
    } else if value >= i16::MIN as i128 && value <= i16::MAX as i128 {
        quote! { i16 }
    } else if value >= i32::MIN as i128 && value <= i32::MAX as i128 {
        quote! { i32 }
    } else if value >= i64::MIN as i128 && value <= i64::MAX as i128 {
        quote! { i64 }
    } else {
        quote! { i128 }
    }
}
