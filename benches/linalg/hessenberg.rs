use test::{self, Bencher};
use na::Hessenberg;
use nl;
use rulinalg::matrix::decomposition::{Decomposition, HessenbergDecomposition};

/*
 *
 * 100x100
 *
 */
#[bench]
fn hessenberg_100x100_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 100);
    bh.iter(|| test::black_box(Hessenberg::new(m.clone())))
}


#[bench]
fn hessenberg_100x100_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(100, 100);
    bh.iter(|| test::black_box(HessenbergDecomposition::decompose(m.clone())))
}

#[bench]
fn hessenberg_100x100_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 100);
    bh.iter(|| test::black_box(nl::Hessenberg::new(m.clone())))
}


#[bench]
fn hessenberg_unpack_100x100_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 100);
    bh.iter(|| test::black_box(Hessenberg::new(m.clone()).unpack()))
}


#[bench]
fn hessenberg_unpack_100x100_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(100, 100);
    bh.iter(|| test::black_box(HessenbergDecomposition::decompose(m.clone()).unpack()))
}


#[bench]
fn hessenberg_unpack_100x100_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 100);
    bh.iter(|| test::black_box(nl::Hessenberg::new(m.clone()).unpack()))
}

/*
 *
 * 200x200
 *
 */
#[bench]
fn hessenberg_200x200_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(200, 200);
    bh.iter(|| test::black_box(Hessenberg::new(m.clone())))
}


#[bench]
fn hessenberg_200x200_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(200, 200);
    bh.iter(|| test::black_box(HessenbergDecomposition::decompose(m.clone())))
}

#[bench]
fn hessenberg_200x200_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(200, 200);
    bh.iter(|| test::black_box(nl::Hessenberg::new(m.clone())))
}


#[bench]
fn hessenberg_unpack_200x200_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(200, 200);
    bh.iter(|| test::black_box(Hessenberg::new(m.clone()).unpack()))
}


#[bench]
fn hessenberg_unpack_200x200_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(200, 200);
    bh.iter(|| test::black_box(HessenbergDecomposition::decompose(m.clone()).unpack()))
}

#[bench]
fn hessenberg_unpack_200x200_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(200, 200);
    bh.iter(|| test::black_box(nl::Hessenberg::new(m.clone()).unpack()))
}

/*
 *
 * 500x500
 *
 */
#[bench]
fn hessenberg_500x500_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(500, 500);
    bh.iter(|| test::black_box(Hessenberg::new(m.clone())))
}


#[bench]
fn hessenberg_500x500_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(500, 500);
    bh.iter(|| test::black_box(HessenbergDecomposition::decompose(m.clone())))
}

#[bench]
fn hessenberg_500x500_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(500, 500);
    bh.iter(|| test::black_box(nl::Hessenberg::new(m.clone())))
}


#[bench]
fn hessenberg_unpack_500x500_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(500, 500);
    bh.iter(|| test::black_box(Hessenberg::new(m.clone()).unpack()))
}


#[bench]
fn hessenberg_unpack_500x500_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(500, 500);
    bh.iter(|| test::black_box(HessenbergDecomposition::decompose(m.clone()).unpack()))
}


#[bench]
fn hessenberg_unpack_500x500_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(500, 500);
    bh.iter(|| test::black_box(nl::Hessenberg::new(m.clone()).unpack()))
}

