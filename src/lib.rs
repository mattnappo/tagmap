//! Format:
//! Each enum has this structure:
//!
//! ```c
//! struct Enum {
//!     enum EnumTag variant;
//!     union { ... };
//! };
//! ```
//!
//! # Mapping
//! ## Unit variant
//! Units will have type `typedef empty uint8_t` in the union. Naming scheme
//! is below.
//!
//! ## Struct variant
//! Create an anon struct in the union, where the fields in
//! the struct are named in accordance to the struct variant names.
//!
//! ## Tuple variant
//! Create anon struct in the union where the fields in the
//! struct are labeled t_0, t_1, ...
//!
//! # Naming
//! A variant named `EnumVariant` will have the name `enum_variant` in the union

/// A simple, c-style enum where each variant <-> an int.
/// So really this is the same thing as
/// ```
/// enum Simple {
///     A(i32),
///     B(i32),
///     C(i32)
/// }
/// ```
///
/// ```c
/// typedef int empty;
/// enum SimpleTag {A, B, C};
/// struct Simple {
///     enum SimpleTag variant;
///     union {
///         empty a;
///         empty b;
///         empty c;
///     };
/// };
///
/// ```
pub enum Simple {
    A,
    B,
    C,
}

/// A simple sum type.
///
/// ```c
/// enum SumTag {
///     SUM_A,
///     SUM_B,
/// };
/// struct Sum {
///     enum SumTag variant;
///     union {
///         char *a;
///         int b;
///     };
/// };
/// ```
pub enum Sum {
    A(String),
    B(i32),
}

/// Unnamed use temps (they're still named).
///
/// ```c
/// struct Unnamed {
///     enum UnnamedTag variant;
///     union {
///         struct {
///             char *t_0;
///             int   t_1;
///         } a;
///         long long b;
///     };
/// }
/// ```
pub enum Unnamed {
    A(String, i32),
    B(u128),
}

/// A slightly more complicated sum type. The only difference between named
/// and unnamed is that unnamed uses temp values (t_0, t_1, ...)
///
/// ```c
/// struct Named {
///     enum NamedTag variant;
///     union {
///         struct {
///             char *x;
///             int y;
///         } a;
///         empty b;
///     };
/// };
/// ```
pub enum Named {
    A { x: String, y: i32 },
    B,
}
