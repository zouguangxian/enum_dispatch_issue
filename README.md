with rust stable version, `cargo expand --lib foo` will output:
```
error[E0425]: cannot find value `inner` in this scope
  --> src/foo.rs:27:9
   |
27 |         #[enum_dispatch(Hello)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
...
34 | hello_set!(Foo, A: HelloA, B: HelloB);
   | ------------------------------------- in this macro invocation
   |
   = note: this error originates in the attribute macro `enum_dispatch` which comes from the expansion of the macro `hello_set` (in Nightly builds, run with -Z macro-backtrace for more info)
For more information about this error, try `rustc --explain E0425`.
```
