use test::{self, Bencher};
use na::LU;
use nl;
use ndarray_linalg::solve::FactorizeInto;
use linxal::factorization;
use rulinalg::matrix::decomposition::{Decomposition, PartialPivLu};

/*
 *
 * 100x100
 *
 */
#[bench]
fn lu_100x100_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 100);
    bh.iter(|| test::black_box(LU::new(m.clone())))
}


#[bench]
fn lu_100x100_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(100, 100);
    bh.iter(|| test::black_box(PartialPivLu::decompose(m.clone())))
}

#[bench]
fn lu_100x100_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 100);
    bh.iter(|| test::black_box(nl::LU::new(m.clone())))
}

#[bench]
fn lu_100x100_linxal(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_ndarray(100, 100);
    bh.iter(|| test::black_box(factorization::LU::compute_into(m.clone())))
}

#[bench]
fn lu_100x100_ndarray_linalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_ndarray(100, 100);
    bh.iter(|| test::black_box(m.clone().factorize_into()))
}

#[bench]
fn lu_unpack_100x100_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 100);
    bh.iter(|| test::black_box(LU::new(m.clone()).unpack()))
}

#[bench]
fn lu_unpack_100x100_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(100, 100);
    bh.iter(|| test::black_box(PartialPivLu::decompose(m.clone()).unwrap().unpack()))
}


/*
 *
 * 200x200
 *
 */
#[bench]
fn lu_200x200_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(200, 200);
    bh.iter(|| test::black_box(LU::new(m.clone())))
}


#[bench]
fn lu_200x200_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(200, 200);
    bh.iter(|| test::black_box(PartialPivLu::decompose(m.clone())))
}

#[bench]
fn lu_200x200_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(200, 200);
    bh.iter(|| test::black_box(nl::LU::new(m.clone())))
}

#[bench]
fn lu_200x200_linxal(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_ndarray(200, 200);
    bh.iter(|| test::black_box(factorization::LU::compute_into(m.clone())))
}

#[bench]
fn lu_200x200_ndarray_linalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_ndarray(200, 200);
    bh.iter(|| test::black_box(m.clone().factorize_into()))
}

#[bench]
fn lu_unpack_200x200_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(200, 200);
    bh.iter(|| test::black_box(LU::new(m.clone()).unpack()))
}

#[bench]
fn lu_unpack_200x200_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(200, 200);
    bh.iter(|| test::black_box(PartialPivLu::decompose(m.clone()).unwrap().unpack()))
}

/*
 *
 * 500x500
 *
 */
#[bench]
fn lu_500x500_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(500, 500);
    bh.iter(|| test::black_box(LU::new(m.clone())))
}

#[bench]
fn lu_500x500_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(500, 500);
    bh.iter(|| test::black_box(PartialPivLu::decompose(m.clone())))
}

#[bench]
fn lu_500x500_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(500, 500);
    bh.iter(|| test::black_box(nl::LU::new(m.clone())))
}

#[bench]
fn lu_500x500_linxal(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_ndarray(500, 500);
    bh.iter(|| test::black_box(factorization::LU::compute_into(m.clone())))
}

#[bench]
fn lu_500x500_ndarray_linalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_ndarray(500, 500);
    bh.iter(|| test::black_box(m.clone().factorize_into()))
}


#[bench]
fn lu_unpack_500x500_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(500, 500);
    bh.iter(|| test::black_box(LU::new(m.clone()).unpack()))
}


#[bench]
fn lu_unpack_500x500_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(500, 500);
    bh.iter(|| test::black_box(PartialPivLu::decompose(m.clone()).unwrap().unpack()))
}
