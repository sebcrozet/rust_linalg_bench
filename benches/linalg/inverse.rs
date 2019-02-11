use test::{self, Bencher};
#[cfg(feature = "lapack")]
use nl;
//use linxal::prelude::*;
#[cfg(feature = "lapack")]
use ndarray_linalg::solve::InverseInto;


/*
 *
 * 100x100
 *
 */
#[bench]
fn inverse_100x100_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 100);
    bh.iter(|| test::black_box(m.clone().try_inverse()))
}

#[bench]
fn inverse_100x100_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(100, 100);
    bh.iter(|| test::black_box(m.clone().inverse()))
}

#[bench]
#[cfg(feature = "lapack")]
fn inverse_100x100_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 100);
    bh.iter(|| test::black_box(nl::LU::new(m.clone()).inverse()))
}

//#[bench]
//fn inverse_100x100_linxal(bh: &mut Bencher) {
//    let m = ::reproductible_dmatrix_ndarray(100, 100);
//    bh.iter(|| test::black_box(m.inverse()))
//}
//
//#[bench]
//fn inverse_100x100_linxal_column_major(bh: &mut Bencher) {
//    let m = ::reproductible_column_major_dmatrix_ndarray(100, 100);
//    bh.iter(|| test::black_box(m.inverse()))
//}

#[bench]
#[cfg(feature = "lapack")]
fn inverse_100x100_ndarray_linalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_ndarray(100, 100);
    bh.iter(|| test::black_box(m.clone().inv_into()))
}


/*
 *
 * 200x200
 *
 */
#[bench]
fn inverse_200x200_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(200, 200);
    bh.iter(|| test::black_box(m.clone().try_inverse()))
}


#[bench]
fn inverse_200x200_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(200, 200);
    bh.iter(|| test::black_box(m.clone().inverse()))
}

#[bench]
#[cfg(feature = "lapack")]
fn inverse_200x200_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(200, 200);
    bh.iter(|| test::black_box(nl::LU::new(m.clone()).inverse()))
}

//#[bench]
//fn inverse_200x200_linxal(bh: &mut Bencher) {
//    let m = ::reproductible_dmatrix_ndarray(200, 200);
//    bh.iter(|| test::black_box(m.inverse()))
//}
//
//#[bench]
//fn inverse_200x200_linxal_column_major(bh: &mut Bencher) {
//    let m = ::reproductible_column_major_dmatrix_ndarray(200, 200);
//    bh.iter(|| test::black_box(m.inverse()))
//}

#[bench]
#[cfg(feature = "lapack")]
fn inverse_200x200_ndarray_linalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_ndarray(200, 200);
    bh.iter(|| test::black_box(m.clone().inv_into()))
}


/*
 *
 * 500x500
 *
 */
#[bench]
fn inverse_500x500_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(500, 500);
    bh.iter(|| test::black_box(m.clone().try_inverse()))
}

#[bench]
fn inverse_500x500_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(500, 500);
    bh.iter(|| test::black_box(m.clone().inverse()))
}

#[bench]
#[cfg(feature = "lapack")]
fn inverse_500x500_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(500, 500);
    bh.iter(|| test::black_box(nl::LU::new(m.clone()).inverse()))
}

//#[bench]
//fn inverse_500x500_linxal(bh: &mut Bencher) {
//    let m = ::reproductible_dmatrix_ndarray(500, 500);
//    bh.iter(|| test::black_box(m.inverse()))
//}
//
//#[bench]
//fn inverse_500x500_linxal_column_major(bh: &mut Bencher) {
//    let m = ::reproductible_column_major_dmatrix_ndarray(500, 500);
//    bh.iter(|| test::black_box(m.inverse()))
//}

#[bench]
#[cfg(feature = "lapack")]
fn inverse_500x500_ndarray_linalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_ndarray(500, 500);
    bh.iter(|| test::black_box(m.clone().inv_into()))
}
