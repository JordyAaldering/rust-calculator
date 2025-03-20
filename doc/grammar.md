
# Language Definition

```bnf
<expr> := '(' <expr> ')'
        | <binary>
        | <unary>
        | <int>
```

```bnf
<binary> := <expr> <bop> <expr>

<bop> := '+' | '-' | '*' | '/' | '^'
       | '==' | '!='
```

```bnf
<unary> := <uop> <expr>

<uop> := '-'
       | '!'
```
