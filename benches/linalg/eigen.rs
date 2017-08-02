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
    let m = ::reproductible_sdp_na(100);
    bh.iter(|| test::black_box(m.eigenvalues()))
}


#[bench]
fn eigenvalues_100x100_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_sdp_rulinalg(100);
    bh.iter(|| test::black_box(m.clone().eigenvalues()))
}

#[bench]
fn eigenvalues_100x100_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(100);
    bh.iter(|| test::black_box(nl::RealEigensystem::new(m.clone(), false, false).unwrap().eigenvalues))
}

#[bench]
fn eigenvalues_100x100_linxal(bh: &mut Bencher) {
    let m = ::reproductible_sdp_ndarray(100);
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
    let m = ::reproductible_sdp_na(200);
    bh.iter(|| test::black_box(m.eigenvalues()))
}


#[bench]
fn eigenvalues_200x200_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_sdp_rulinalg(200);
    bh.iter(|| test::black_box(m.clone().eigenvalues()))
}

#[bench]
fn eigenvalues_200x200_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(200);
    bh.iter(|| test::black_box(nl::RealEigensystem::new(m.clone(), false, false).unwrap().eigenvalues))
}

#[bench]
fn eigenvalues_200x200_linxal(bh: &mut Bencher) {
    let m = ::reproductible_sdp_ndarray(200);
    bh.iter(|| test::black_box(m.eigenvalues()))
}

/*
 *
 * rulinalg takes too long to complete the 500x500 benchmarks.
 *
 */
