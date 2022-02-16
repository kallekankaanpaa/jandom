# jandom

A port of Java `java.util.Random` to Rust.

The implementation follows the public API of Java 17 Random. The method signatures have been changed to be more rusty, 
for example, `nextInt` -> `next_i32`, `nextDouble` -> `next_f64`, `nextGaussian` -> `next_gaussian`, etc.
Jandom differs from other java Random implementations by having true feature parity, where even the `next_gaussian` method returns
same values as the Java equivalent. 

Jandom makes use of atomics and Mutex to be thread safe by default.
The library uses nightly toolchain to take advantage of [LazySync](https://doc.rust-lang.org/std/lazy/struct.SyncLazy.html).

Example usages can be found in the `examples` directory.

## Contributing

If you find any discrepencies between this and the Java implementation, [please file a Issue](https://github.com/kallekankaanpaa/jandom/issues/new).

## Licensing

The code has been licensed under both MIT and Apache 2.0 to follow the [Rust API guidelines](https://rust-lang.github.io/api-guidelines/necessities.html#crate-and-its-dependencies-have-a-permissive-license-c-permissive).
