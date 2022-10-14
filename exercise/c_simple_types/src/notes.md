# Section 2 Fundamentals

### [Scalar Types](https://doc.rust-lang.org/book/ch03-02-data-types.html#scalar-types)

- 4 scalar types in Rust:
- Integer, Floats, Booleans, and Characters

#### Integers

- Int types starting with _u_ are _unsigned_ integers, followed by the number of bits they'll take. e.g. u32 - unsigned 32 bit integer.
- [usize](https://doc.rust-lang.org/std/primitive.usize.html) is a unique type that is the size of the platform's pointer type. (idk what that means right now), however I do know this is likely the type of int that will be used when accessing array and vector indexes.
- Int types starting with _i_ are signed _signed_ integers, also followed by their bitsize.
- [isize](https://doc.rust-lang.org/std/primitive.isize.html) follows the same as for unsigned.. the size determined by the platforms pointers. this is likely the type of int that will be used when accessing array and vector indexes.
- If you don't type your integer, default is _i32_
- Integer literals can be either _decimal_, _binary_, _hexadecimal_, or _utf8_ which is often called just _'binary'_
- Floating Point values can be either _f32_ or _f64_ and default is 64 bit

#### Booleans

- Either **true** or **false** (lower case)
- These do not correspond to numeric values 1 or 0 like they do in JS for instance, so you can't do math on them. _unless you cast them to an i8 or integer_

#### Characters

- An important distinction and critical to remember is that in Rust, single quotes denote a _'character literal'_
- While double quotes represent a _"string literal"_
- The _char_ type is 4 bytes in size which means it can represent 4 times as many characters/lang

### [Compound Types](https://doc.rust-lang.org/book/ch03-02-data-types.html#compound-types)