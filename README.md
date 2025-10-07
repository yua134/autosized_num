[![CI](https://github.com/yua134/autosized_num/actions/workflows/ci.yml/badge.svg)](https://github.com/yua134/autosized_num/actions/workflows/ci.yml)

# autosized-num

Auto-sized integer macros: choose the smallest signed/unsigned type for a literal at compile time.

## features

## Features

- **Auto-sized integer macros**  
  Automatically choose the smallest fitting integer type for a literal.

- **Unsigned support**  
  `auto_sized_unsigned!` / `auto_sized_unsigned_val!` → expands to `u8`, `u16`, `u32`, `u64`, or `u128`.

- **Signed support**  
  `auto_sized_signed!` / `auto_sized_signed_val!` → expands to `i8`, `i16`, `i32`, `i64`, or `i128`.

- **Unified macros**  
  `auto_sized_int!` / `auto_sized_int_val!` → automatically pick signed or unsigned depending on the literal.  
  Accepts the full `i128` range.

- **Type and value variants**  
  - `*_unsigned!`, `*_signed!`, `*_int!` → return a type.  
  - `*_val` variants → return a value with an explicit cast.

- **no_std friendly**  
  Expanded code uses only primitive integer types, so it works in `no_std` environments.

## Usage

Add the crate with Cargo:

```bash
cargo add autosized-num
```

Then import the macros and use them in your code:

```rust
use autosized::*;

fn main() {
    // Type macros
    type T1 = auto_sized_unsigned!(300); // expands to u16
    type T2 = auto_sized_signed!(-200);  // expands to i16
    type T3 = auto_sized_int!(10);       // expands to u8
    type T4 = auto_sized_int!(-10);      // expands to i8

    // Value macros
    let a = auto_sized_unsigned_val!(300); // 300u16
    let b = auto_sized_signed_val!(-200);  // -200i16
    let c = auto_sized_int_val!(10);       // 10u8
    let d = auto_sized_int_val!(-10);      // -10i8

    println!("{a}, {b}, {c}, {d}");
}
```

### Notes

- `auto_sized_int!` / `auto_sized_int_val!` accept the full `i128` range.
- The macros expand to primitive integer types only, so they are fully usable in `no_std` environments.

## Crate Info

- License: MIT OR Apache-2.0
- Repository: [GitHub](https://github.com/yua134/autosized-num)