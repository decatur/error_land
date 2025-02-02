Error handling in Rust is deep.
The shallow part [enum Result<T, E>](https://doc.rust-lang.org/std/result/enum.Result.html) is part of the Rust language.
It is easy to understand and well designed.

Choosing an error handling concept for your project is the deep part.

# Proposal

This repo proposes a dead simple solution such that

* The question mark operator [?](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator) just works.
* Errors are part of the tracing and are never swallowed.
* For each method/function, declare up front which Errors are allowed to be propagated.
* No need for `.context("")` clutter.
* No need to collect a backtrace and fumble with `RUST(_LIB)_BACKTRACE` or `debug = true` in release mode
* No need to mess with `panic::set_hook` to [convert error post mortem output to structured log](https://stackoverflow.com/questions/78708247)

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
Because `From<T> for U` implies `Into<U> for T`, we simply have to generate a `From` trait implemenation for all allowed error propagations.
In the generated `From` impl, we use `#[track_caller]` to output [file, line and column](https://doc.rust-lang.org/std/panic/struct.Location.html).

# Kitchen Sink Resources

## Error Tree

* https://github.com/klebs6/klebs-general/tree/master/error-tree
* https://github.com/bobozaur/transitive
* https://github.com/steffahn/transitive_from
