# `calc`

Yet another CLI calculator. Inspired by the excellent <https://github.com/alfredxing/calc>.

**Note**: Currently, almost no features are yet implemented. This README can be considered a roadmap of planned features
rather than a usage document.

## Usage

### Expression Mode

```sh
$ calc "1/(2+(3*(4-5)))"
-1
$ calc 12345 / 543 --round 2
22.73
```

When non-flag arguments are present, `calc` interprets them as an expression and evaluates them immediately.

### Shell Mode

```sh
$ calc
[0]: 1+1
2
[1]: 3(5/(3-4))
-15
[2]: 3pi^2
29.608813203268074
[3]: @+1
30.608813203268074
[4]: @@@*2
-30
[5]: ln(-1)
NaN
> exit
$
```

In the absence of non-flag arguments, `calc` launches a simple shell which just evaluates each line of input.

## Reference

### Data Types

Every invocation of `calc` interprets all arguments as a single data type. By default, `calc` uses `f64`, but other data types
can be chosen by command-line flag:

- `f64` (default): floating point operations
- `u64`: unsigned integer operations
- `i64`: signed integer operations
- `decimal N`: signed fixed-position decimal operations building on the [`bigdecimal` crate](https://crates.io/crates/bigdecimal)

Note that the data type chosen will restrict the available operators, functions, and constants. For example, trigonometric operations
are not available on integers, and bit-shifting operations are not available on floats.

### Order of Operations

The following order of operations is used to resolve expressions:

- Parentheses (`(...)`)
- Unary Prefix Operators (`-` `!`)
- Bitwise operations (`&` `|` `^`)
- Shifts (`<<` `>>`)
- Exponentiation (`**`)
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
- `&`: Bitwise And
- `|`: Bitwise Or
- `^`: Bitwise xor

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

## Crate Structure

This crate includes both library code and CLI code. The CLI code is all gated behind feature `cli`; the
`cli` feature is in the default features. This means that the CLI is built by default. However, it is
possible to use this crate as a library without building any of the CLI code by including in your
`Cargo.toml`:

```toml
[dependencies]
calc = { version = "*", default-features = false }
```
