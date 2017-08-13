# Linear algebra benchmarks for the Rust programming language.

Benchmarks the following linear Rust algebra crates for matrix factorizations:

* nalgebra (pure-rust implementations)
* rulinalg (pure-rust implementations)
* nalgebra-lapack (based on Lapack bindings)
* linxal using row-major matrices (based on Lapack bindings)
* linxal using column-major matrices (based on Lapack bindings)
* ndarray-linalg (based on Lapack bindings)

Note that the comparison of crates based on Lapack bindings makes sense only the
same Lapack backend is selected for all of them. By default, the `openblas`
implementation is chosen.

This also includes a few benchmarks confronting **nalgebra** with **cgmath**
(with explicit simd enabled) for a few low-dimensional operations. While both
show similar performance levels, note that performance differences may be
different for non-optimized code. In particular generally **nalgebra** is
slower than **cgmath** when optimization are off. Indeed, **cgmath** uses
explicit SIMD intrinsics while **nalgebra** relies on the optimizer's ability to
vectorize code automatically.
