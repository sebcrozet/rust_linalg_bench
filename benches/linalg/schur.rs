use test::{self, Bencher};
use nl;


/*
 *
 *
 * 100x100
 *
 *
 */
#[bench]
fn schur_100x100_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 100);
    bh.iter(|| test::black_box(m.clone().real_schur()))
}

#[bench]
fn schur_100x100_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 100);
    bh.iter(|| test::black_box(nl::RealSchur::new(m.clone())))
}


/*
 *
 *
 * 200x200
 *
 *
 */
#[bench]
fn schur_200x200_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(200, 200);
    bh.iter(|| test::black_box(m.clone().real_schur()))
}

#[bench]
fn schur_200x200_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(200, 200);
    bh.iter(|| test::black_box(nl::RealSchur::new(m.clone())))
}
