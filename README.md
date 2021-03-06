# `calc`

[![Build and Test Status](https://github.com/coriolinus/calc/workflows/Build%20and%20Test/badge.svg?branch=main)](https://github.com/coriolinus/calc/actions?query=branch%3Amain+workflow%3A%22Build+and+Test%22)

Yet another CLI calculator. Inspired by the excellent <https://github.com/alfredxing/calc>.

## Installation

With a Rust toolchain in place:

```sh
cargo install --force calc
```

Alternately, you can download a precompiled executable of the most recent [release](https://github.com/coriolinus/calc/releases).

## Usage

### Expression Mode

```sh
$ calc "1/(2+(3*(4-5)))"
-1
$ calc "round(12345 / 543)"
23
```

When non-flag arguments are present, `calc` interprets them as an expression and evaluates them immediately.

### Shell Mode

```sh
$ calc
[0]: 1 + 1
2
[1]: 3*(5/(3-4))
-15
[2]: 3*pi**2
29.608813203268074
[3]: @+1
30.608813203268074
[4]: @@@*2
-30
[5]: ln(-1)
NaN
```

In the absence of non-flag arguments, `calc` launches a simple shell which just evaluates each line of input.

## Reference

### Data Types

Every invocation of `calc` interprets all arguments as a single data type. By default, `calc` uses `f64`, but other data types
can be chosen by command-line flag:

- `f64` (default): signed 64-bit floating point operations
- `u64`: unsigned 64-bit integer operations
- `i64`: signed 64-bit integer operations

Note that the data type chosen will restrict the available operators, functions, and constants. For example, trigonometric operations
are not available on integers, and bit-shifting operations are not available on floats.

### Numeric Input Format

Numbers may contain `_` characters at any point. Those symbols are ignored; they are for user convenience and readability only.

`calc` can handle inputs in several numeric bases.

- Un-annotated numbers are assumed to be base 10. Example: `123.45`.

  Note: this is the only format which is legal for non-integral numbers.

- Numbers with a `0b` prefix are in base 2. Example: `0b0110_1010`.
- Numbers with a `0o` prefix are in base 8. Example: `0o755`.

  Note: a leading `0` is not in itself an octal prefix. Example: `0755` equals `0d755`.

- Numbers with a `0d` prefix are in base 10. Example: `1234_5678`.
- Numbers with a `0x` prefix are in base 16. Example: `0xdead_beef`.

It is legal to intermix inputs of varying bases.

### Numeric Output Format

The output format of an expression can be specified by adding a `:` symbol followed by a format
specifier to the expression.

The format specifier can be anything recognized by the [`num-runtime-fmt` crate](https://docs.rs/num-runtime-fmt/0.1.1/num_runtime_fmt/#format-string-grammar).

```sh
$ calc -u "0o644 | 0o111 :#o"
0o755
$ calc -u '0o755 & !0o111 :04o'
0644
```

```
[0]: 0xab :b 4
1010 1011
[1]: @[0] >>> 4 :x_4
b000_0000_0000_000a
[2]: @ & 0xF :4b
1010
```

```
$ calc pi / 3 :v#04.4
0d01.0471
```

### Order of Operations

The following order of operations is used to resolve expressions:

- Parentheses (`(...)`)
- Unary Prefix Operators (`-` `!`)
- Shifts and Exponentiation (`<<` `>>` `<<<` `>>>` `**`)
- Bitwise operations (`&` `|` `^`)
- Multiplication and Division (`*` `/` `//` `%`)
- Addition and Subtraction (`+` `-`)

Operations at the same level of precedence are resolved from left to right.

### Unary Prefix Operators

- `-`: Negation
- `!`: Bitwise Not

### Infix Operators

- `+`: Addition
- `-`: Subtraction
- `*`: Multiplication
- `/`: Division
- `//`: Truncating Division: divides, truncating all data after the decimal point.
- `**`: Exponentiation
- `%` : Arithmetic remainder
- `<<`: Left Shift
- `>>`: Right Shift
- `<<<`: Wrapping Left Shift (Rotate Left)
- `>>>`: Wrappping Right Shift (Rotate Right)
- `&`: Bitwise And
- `|`: Bitwise Or
- `^`: Bitwise Xor

### Functions

- `abs`: Absolute Value
- `ceil`: Smallest integer greater than or equal to the input
- `floor`: Greatest integer less than or equal to the input
- `round`: Nearest integer to the input; halfway cases away from 0.0
- `sin`: Sine
- `cos`: Cosine
- `tan`: Tangent
- `sinh`: Hyperbolic Sine
- `cosh`: Hyperbolic Cosine
- `tanh`: Hyperbolic Tangent
- `asin`: Arcine
- `acos`: Arccosine
- `atan`: Arctangent
- `asinh`: Inverse Hyperbolic Sine
- `acosh`: Inverse Hyperbolic Cosine
- `atanh`: Inverse Hyperbolic Tangent
- `rad`: Convert a number in degrees to radians
- `dec`: Convert a number in radians to degrees
- `sqrt`: Square Root
- `cbrt`: Cube Root
- `log`: Base-10 Logarithm
- `lg`: Base-2 Logarithm
- `ln`: Natural (Base-e) Logarithm
- `exp`: `e**x`

Trigonometric functions operate on radians.

### Constants

- `e`: Euler's Number
- `pi`: Archimedes' Constant
- `π`: Archimedes' Constant

### History

In shell mode, `calc` keeps the results of all expressions in memory until it is quit.

The pseudovariable `@` always refers to the result of the previous expression.
The pseudovariable `@@` always refers to the result of the expression before the previous.
Any number of `@` symbols can be chained this way.

Simply chaining `@` symbols can get cumbersome. The syntax `@{N}`, where `N` is an integer,
refers to the `N`th previous result. `@{1}` always refers to the result of the previous expression;
it is equivalent to `@`. `@{3}` refers to the result 3 expressions ago; it is equivalent to `@@@`.

The pseuaovariable `@[0]` always refers to the result of the first expression in this shell session.
Likewise, `@[1]` refers to the second, and so on. The shell interface indicates the current expression.

## Warnings

### No Implicit Multiplication

Implicit multiplication is not supported. Use a multiplication operator such as `*`.

### Floating Point Errors

Floating point operations can compound lossily, and `calc` makes no special efforts to guard against
this kind of error. For example:

```sh
$ calc 'sin(rad(45)) - (sqrt(2) / 2)'
-0.00000000000000011102230246251565
```

## Crate Structure

This crate includes both library code and CLI code. The CLI code is all gated behind feature `cli`; the
`cli` feature is in the default features. This means that the CLI is built by default. However, it is
possible to use this crate as a library without building any of the CLI code by including in your
`Cargo.toml`:

```toml
[dependencies]
calc = { version = "*", default-features = false }
```
