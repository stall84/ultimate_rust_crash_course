# Section 2 Fundamentals

### Scalar Types

- 4 scalar types in Rust:
- Integer, Floats, Booleans, and Characters

#### Integers

- Int types starting with _u_ are _unsigned_ integers, followed by the number of bits they'll take. e.g. u32 - unsigned 32 bit integer.
- [usize](https://doc.rust-lang.org/std/primitive.usize.html) is a unique type that is the size of the platform's pointer type. (idk what that means right now)
- Int types starting with _i_ are signed _signed_ integers, also followed by their bitsize.
- [isize](https://doc.rust-lang.org/std/primitive.isize.html) follows the same as for unsigned.. the size determined by the platforms pointers.
- If you don't type your integer, default is _i32_
- Integer literals can be either _decimal_, _binary_, _hexadecimal_, or _utf8_ which is often called just _'binary'_
- Floating Point values can be either _f32_ or _f64_ and default is 64 bit
