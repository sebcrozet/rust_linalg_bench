use test::{self, Bencher};
use rulinalg::matrix::decomposition::{Decomposition, FullPivLu as RuFullPivLu};

/*
 *
 * 100x100
 *
 */
#[bench]
fn full_piv_lu_100x100_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 100);
    bh.iter(|| test::black_box(m.clone().full_piv_lu()))
}


#[bench]
fn full_piv_lu_100x100_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(100, 100);
    bh.iter(|| test::black_box(RuFullPivLu::decompose(m.clone())))
}


#[bench]
fn full_piv_lu_unpack_100x100_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 100);
    bh.iter(|| test::black_box(m.clone().full_piv_lu().unpack()))
}


#[bench]
fn full_piv_lu_unpack_100x100_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(100, 100);
    bh.iter(|| test::black_box(RuFullPivLu::decompose(m.clone()).unwrap().unpack()))
}


/*
 *
 * 200x200
 *
 */
#[bench]
fn full_piv_lu_200x200_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(200, 200);
    bh.iter(|| test::black_box(m.clone().full_piv_lu()))
}


#[bench]
fn full_piv_lu_200x200_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(200, 200);
    bh.iter(|| test::black_box(RuFullPivLu::decompose(m.clone())))
}


#[bench]
fn full_piv_lu_unpack_200x200_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(200, 200);
    bh.iter(|| test::black_box(m.clone().full_piv_lu().unpack()))
}


#[bench]
fn full_piv_lu_unpack_200x200_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(200, 200);
    bh.iter(|| test::black_box(RuFullPivLu::decompose(m.clone()).unwrap().unpack()))
}

/*
 *
 * 500x500
 *
 */
#[bench]
fn full_piv_lu_500x500_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(500, 500);
    bh.iter(|| test::black_box(m.clone().full_piv_lu()))
}


#[bench]
fn full_piv_lu_500x500_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(500, 500);
    bh.iter(|| test::black_box(RuFullPivLu::decompose(m.clone())))
}


#[bench]
fn full_piv_lu_unpack_500x500_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(500, 500);
    bh.iter(|| test::black_box(m.clone().full_piv_lu().unpack()))
}


#[bench]
fn full_piv_lu_unpack_500x500_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(500, 500);
    bh.iter(|| test::black_box(RuFullPivLu::decompose(m.clone()).unwrap().unpack()))
}
