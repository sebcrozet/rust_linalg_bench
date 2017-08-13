use test::{self, Bencher};
use nl;
use linxal::prelude::*;
use ndarray_linalg::cholesky::CholeskyInto;
use ndarray_linalg::lapack_traits::UPLO;
use rulinalg::matrix::decomposition::{Decomposition, Cholesky as RuCholesky};

/*
 *
 * 100x100
 *
 */
#[bench]
fn cholesky_100x100_na(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(100);
    bh.iter(|| test::black_box(m.clone().cholesky()))
}


#[bench]
fn cholesky_100x100_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_sdp_rulinalg(100);
    bh.iter(|| test::black_box(RuCholesky::decompose(m.clone())))
}

#[bench]
fn cholesky_100x100_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(100);
    bh.iter(|| test::black_box(nl::Cholesky::new(m.clone())))
}



#[bench]
fn cholesky_unpack_100x100_na(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(100);
    bh.iter(|| test::black_box(m.clone().cholesky().unwrap().unpack()))
}


#[bench]
fn cholesky_unpack_100x100_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_sdp_rulinalg(100);
    bh.iter(|| test::black_box(RuCholesky::decompose(m.clone()).unwrap().unpack()))
}


#[bench]
fn cholesky_unpack_100x100_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(100);
    bh.iter(|| test::black_box(nl::Cholesky::new(m.clone()).unwrap().unpack()))
}


#[bench]
fn cholesky_unpack_100x100_linxal(bh: &mut Bencher) {
    let m = ::reproductible_sdp_ndarray(100);
    bh.iter(|| test::black_box(m.cholesky(Symmetric::Lower)))
}

#[bench]
fn cholesky_unpack_100x100_ndarray_linalg(bh: &mut Bencher) {
    let m = ::reproductible_sdp_ndarray(100);
    bh.iter(|| test::black_box(CholeskyInto::cholesky_into(m.clone(), UPLO::Lower)))
}

/*
 *
 * 200x200
 *
 */
#[bench]
fn cholesky_200x200_na(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(200);
    bh.iter(|| test::black_box(m.clone().cholesky()))
}


#[bench]
fn cholesky_200x200_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_sdp_rulinalg(200);
    bh.iter(|| test::black_box(RuCholesky::decompose(m.clone())))
}


#[bench]
fn cholesky_200x200_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(200);
    bh.iter(|| test::black_box(nl::Cholesky::new(m.clone())))
}


#[bench]
fn cholesky_unpack_200x200_na(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(200);
    bh.iter(|| test::black_box(m.clone().cholesky().unwrap().unpack()))
}


#[bench]
fn cholesky_unpack_200x200_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_sdp_rulinalg(200);
    bh.iter(|| test::black_box(RuCholesky::decompose(m.clone()).unwrap().unpack()))
}


#[bench]
fn cholesky_unpack_200x200_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(200);
    bh.iter(|| test::black_box(nl::Cholesky::new(m.clone()).unwrap().unpack()))
}

#[bench]
fn cholesky_unpack_200x200_linxal(bh: &mut Bencher) {
    let m = ::reproductible_sdp_ndarray(200);
    bh.iter(|| test::black_box(m.cholesky(Symmetric::Lower)))
}

#[bench]
fn cholesky_unpack_200x200_ndarray_linalg(bh: &mut Bencher) {
    let m = ::reproductible_sdp_ndarray(200);
    bh.iter(|| test::black_box(CholeskyInto::cholesky_into(m.clone(), UPLO::Lower)))
}

/*
 *
 * 500x500
 *
 */
#[bench]
fn cholesky_500x500_na(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(500);
    bh.iter(|| test::black_box(m.clone().cholesky()))
}


#[bench]
fn cholesky_500x500_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_sdp_rulinalg(500);
    bh.iter(|| test::black_box(RuCholesky::decompose(m.clone())))
}


#[bench]
fn cholesky_500x500_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(500);
    bh.iter(|| test::black_box(nl::Cholesky::new(m.clone())))
}



#[bench]
fn cholesky_unpack_500x500_na(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(500);
    bh.iter(|| test::black_box(m.clone().cholesky().unwrap().unpack()))
}


#[bench]
fn cholesky_unpack_500x500_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_sdp_rulinalg(500);
    bh.iter(|| test::black_box(RuCholesky::decompose(m.clone()).unwrap().unpack()))
}

#[bench]
fn cholesky_unpack_500x500_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(500);
    bh.iter(|| test::black_box(nl::Cholesky::new(m.clone()).unwrap().unpack()))
}

#[bench]
fn cholesky_unpack_500x500_linxal(bh: &mut Bencher) {
    let m = ::reproductible_sdp_ndarray(500);
    bh.iter(|| test::black_box(m.cholesky(Symmetric::Lower)))
}

#[bench]
fn cholesky_unpack_500x500_ndarray_linalg(bh: &mut Bencher) {
    let m = ::reproductible_sdp_ndarray(500);
    bh.iter(|| test::black_box(CholeskyInto::cholesky_into(m.clone(), UPLO::Lower)))
}
