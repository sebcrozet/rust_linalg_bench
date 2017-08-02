use test::{self, Bencher};
use na;
use nl;
use ndarray_linalg::svd::SVD;
use linxal::prelude::*;

/*
 *
 *
 * 100x100
 *
 *
 */
#[bench]
fn svd_100x100_na(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(100);
    bh.iter(|| test::black_box(na::SVD::new(m.clone(), true, true)))
}

#[bench]
fn svd_100x100_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_sdp_rulinalg(100);
    bh.iter(|| test::black_box(m.clone().svd()))
}

#[bench]
fn svd_100x100_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(100);
    bh.iter(|| test::black_box(nl::SVD::new(m.clone())))
}

#[bench]
fn svd_100x100_linxal(bh: &mut Bencher) {
    let m = ::reproductible_sdp_ndarray(100);
    bh.iter(|| test::black_box(m.svd_full()))
}

#[bench]
fn svd_100x100_ndarray_linalg(bh: &mut Bencher) {
    let m = ::reproductible_sdp_ndarray(100);
    bh.iter(|| test::black_box(m.svd(true, true)))
}

/*
 *
 *
 * 200x200
 *
 *
 */
#[bench]
fn svd_200x200_na(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(200);
    bh.iter(|| test::black_box(na::SVD::new(m.clone(), true, true)))
}

#[bench]
fn svd_200x200_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_sdp_rulinalg(200);
    bh.iter(|| test::black_box(m.clone().svd()))
}

#[bench]
fn svd_200x200_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_sdp_na(200);
    bh.iter(|| test::black_box(nl::SVD::new(m.clone())))
}

#[bench]
fn svd_200x200_linxal(bh: &mut Bencher) {
    let m = ::reproductible_sdp_ndarray(200);
    bh.iter(|| test::black_box(m.svd_full()))
}

#[bench]
fn svd_200x200_ndarray_linalg(bh: &mut Bencher) {
    let m = ::reproductible_sdp_ndarray(200);
    bh.iter(|| test::black_box(m.svd(true, true)))
}
