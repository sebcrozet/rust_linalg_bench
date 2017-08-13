use test::{self, Bencher};
use nl;
use linxal::prelude::*;


/*
 *
 *
 * 100x100
 *
 *
 */
#[bench]
fn eigenvalues_100x100_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 100);
    bh.iter(|| test::black_box(m.complex_eigenvalues()))
}


#[bench]
fn eigenvalues_100x100_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 100);
    bh.iter(|| test::black_box(nl::Eigen::complex_eigenvalues(m.clone())))
}

#[bench]
fn eigenvalues_100x100_linxal(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_ndarray(100, 100);
    bh.iter(|| test::black_box(m.eigenvalues()))
}

#[bench]
fn eigenvalues_100x100_linxal_column_major(bh: &mut Bencher) {
    let m = ::reproductible_column_major_dmatrix_ndarray(100, 100);
    bh.iter(|| test::black_box(m.eigenvalues()))
}


/*
 *
 *
 * 200x200
 *
 *
 */
#[bench]
fn eigenvalues_200x200_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(200, 200);
    bh.iter(|| test::black_box(m.complex_eigenvalues()))
}

#[bench]
fn eigenvalues_200x200_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(200, 200);
    bh.iter(|| test::black_box(nl::Eigen::complex_eigenvalues(m.clone())))
}

#[bench]
fn eigenvalues_200x200_linxal(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_ndarray(200, 200);
    bh.iter(|| test::black_box(m.eigenvalues()))
}

#[bench]
fn eigenvalues_200x200_linxal_column_major(bh: &mut Bencher) {
    let m = ::reproductible_column_major_dmatrix_ndarray(200, 200);
    bh.iter(|| test::black_box(m.eigenvalues()))
}


// 500x500 benchmarks take too long.
