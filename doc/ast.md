
# Abstract Syntax Tree

At the core of our compiler is the Abstract Syntax Tree (AST). The AST is an
internal tree representation of our program that our computer can understand.
The parser constructs this AST, with all following phases inspecting or
modifying this structure.

Representing our program in a tree encodes a hierarchy.
At the top is the expression node, which can be a binary expression, unary
expression, or a number.
This hierarchy also encodes the precedence of the operations.
Consequently, we do not need a distinct node for parentheses, as these are
encoded into the tree structure itself.

For example, `(1 + 2) - 3` is represented as:

```
    -
   / \
  +   3
 / \
1   2
```

Whereas `1 + (2 - 3)` is represented as:

```
  +
 / \
1   -
   / \
  2   3
```

In practise, in our AST this would really be defined as:

```
     Expr
      |
    Binary
  /   |   \
Num  Add   Expr
 |          |
 1        Binary
        /   |   \
      Num  Min  Num
       |         |
       2         3
```

Where `Binary` and `Unary` are defined as recursive structs, and `Num` is a
simple wrapper around `u32`.

```rust
pub struct Binary {
    pub l: Box<Expr>,
    pub r: Box<Expr>,
    pub op: Bop,
}

pub struct Unary {
    pub r: Box<Expr>,
    pub op: Uop,
}

pub struct Num {
    pub num: u32,
}
```

Although we could define expressions as a trait, we often need to be able to
pattern match on the concrete expression type when traversing the AST.
For example, if we want to count the amount of numbers in the program, we need
to be able to distinguish the number case from the binary and unary cases.
Therefore, we define it as an enum type instead.

```rust
pub enum Expr {
    Binary(Binary),
    Unary(Unary),
    Num(Num),
}
```
