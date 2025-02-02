Error handling in Rust is deep.
The shallow part [enum Result<T, E>](https://doc.rust-lang.org/std/result/enum.Result.html) is part of the Rust language.
It is easy to understand and well designed.

Choosing an error handling concept for your project is the deep part.

# Proposal

This repo proposes a dead simple solution such that

* The question mark operator [?](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator) just works.
* Errors are part of the tracing and are never swallowed.
* No need for `.context("")` clutter.
* For each method/function, declare up front which Errors are allowed to be propagated.
* No need to collect a backtrace and fumble with `RUST(_LIB)_BACKTRACE` or `debug = true` in release mode

# How does it work

Rust desugars the error propagation `?` operator, not considering [try-trait-v2](https://rust-lang.github.io/rfcs/3058-try-trait-v2.html),
like so:
```
match expr {
    Ok(v) => v,
    Err(e) => return Err(e.into()),
}
```

The only way to inject custom behaviour is therefore the `e.into()` part.
Because `From<T> for U` implies `Into<U> for T`, we simply have to generate the `From` trait for all allowed error propagations.


# Kitchen Sink Resources

## Error Tree

* https://github.com/klebs6/klebs-general/tree/master/error-tree
* https://github.com/bobozaur/transitive
* https://github.com/steffahn/transitive_from
