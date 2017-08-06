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
fn symmetric_eigenvalues_100x100_na(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(100);
    bh.iter(|| test::black_box(m.symmetric_eigenvalues()))
}


#[bench]
fn symmetric_eigenvalues_100x100_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_sdp_rulinalg(100);
    bh.iter(|| test::black_box(m.clone().eigenvalues()))
}

#[bench]
fn symmetric_eigenvalues_100x100_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(100);
    bh.iter(|| test::black_box(nl::SymmetricEigen::eigenvalues(m.clone())))
}

#[bench]
fn symmetric_eigenvalues_100x100_linxal(bh: &mut Bencher) {
    let m = ::reproductible_sdp_ndarray(100);
    bh.iter(|| test::black_box(m.symmetric_eigenvalues(Symmetric::Lower)))
}


/*
 *
 *
 * 200x200
 *
 *
 */
#[bench]
fn symmetric_eigenvalues_200x200_na(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(200);
    bh.iter(|| test::black_box(m.symmetric_eigenvalues()))
}


#[bench]
fn symmetric_eigenvalues_200x200_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_sdp_rulinalg(200);
    bh.iter(|| test::black_box(m.clone().eigenvalues()))
}

#[bench]
fn symmetric_eigenvalues_200x200_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(200);
    bh.iter(|| test::black_box(nl::SymmetricEigen::eigenvalues(m.clone())))
}

#[bench]
fn symmetric_eigenvalues_200x200_linxal(bh: &mut Bencher) {
    let m = ::reproductible_sdp_ndarray(200);
    bh.iter(|| test::black_box(m.symmetric_eigenvalues(Symmetric::Lower)))
}

/*
 *
 *
 * 500x500
 *
 *
 */
#[bench]
fn symmetric_eigenvalues_500x500_na(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(500);
    bh.iter(|| test::black_box(m.symmetric_eigenvalues()))
}


#[bench]
fn symmetric_eigenvalues_500x500_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(500);
    bh.iter(|| test::black_box(nl::SymmetricEigen::eigenvalues(m.clone())))
}

#[bench]
fn symmetric_eigenvalues_500x500_linxal(bh: &mut Bencher) {
    let m = ::reproductible_sdp_ndarray(500);
    bh.iter(|| test::black_box(m.symmetric_eigenvalues(Symmetric::Lower)))
}

// NOTE: rulinalg is not included because it takes too long to complete the 500x500 benchmarks.
