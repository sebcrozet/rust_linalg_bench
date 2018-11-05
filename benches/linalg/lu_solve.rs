use test::{self, Bencher};
#[cfg(feature = "lapack")]
use nl;
#[cfg(feature = "lapack")]
use ndarray_linalg::{Solve, solve::FactorizeInto};
#[cfg(feature = "lapack")]
use ndarray_linalg::lapack_traits::Transpose;
use rulinalg::matrix::decomposition::PartialPivLu;


/*
 *
 * 100x100
 *
 */
#[bench]
fn lu_solve_100x100_na(bh: &mut Bencher) {
    let m  = ::reproductible_dmatrix_na(100, 100);
    let b  = ::reproductible_dvector_na(100);
    let lu = m.lu();
    bh.iter(|| test::black_box(lu.solve(&b)))
}


#[bench]
fn lu_solve_100x100_rulinalg(bh: &mut Bencher) {
    let m  = ::reproductible_dmatrix_rulinalg(100, 100);
    let b  = ::reproductible_dvector_rulinalg(100);
    let lu = PartialPivLu::decompose(m).unwrap();
    // NOTE: we do `b.clone()` here because rulinalg takes its argument by-move.
    // There is no extra overhead compared to the nalgebra version (that only pass a reference)
    // because a clone is done internally by the nalgebra LU-solve function.
    bh.iter(|| test::black_box(lu.solve(b.clone())))
}

#[bench]
#[cfg(feature = "lapack")]
fn lu_solve_100x100_na_lapack(bh: &mut Bencher) {
    let m  = ::reproductible_dmatrix_na(100, 100);
    let b  = ::reproductible_dvector_na(100);
    let lu = nl::LU::new(m);
    bh.iter(|| test::black_box(lu.solve(&b)))
}

#[bench]
#[cfg(feature = "lapack")]
fn lu_solve_100x100_ndarray_linalg(bh: &mut Bencher) {
    let m  = ::reproductible_dmatrix_ndarray(100, 100);
    let b  = ::reproductible_dvector_ndarray(100);
    let lu = m.factorize_into().unwrap();
    // See the note about the rulinalg lu_solve bench.
    bh.iter(|| test::black_box(lu.solve(&b)))
}

/*
 *
 * 200x200
 *
 */
#[bench]
fn lu_solve_200x200_na(bh: &mut Bencher) {
    let m  = ::reproductible_dmatrix_na(200, 200);
    let b  = ::reproductible_dvector_na(200);
    let lu = m.lu();
    bh.iter(|| test::black_box(lu.solve(&b)))
}


#[bench]
fn lu_solve_200x200_rulinalg(bh: &mut Bencher) {
    let m  = ::reproductible_dmatrix_rulinalg(200, 200);
    let b  = ::reproductible_dvector_rulinalg(200);
    let lu = PartialPivLu::decompose(m).unwrap();
    // NOTE: we do `b.clone()` here because rulinalg takes its argument by-move.
    // There is no extra overhead compared to the nalgebra version (that only pass a reference)
    // because a clone is done internally by the nalgebra LU-solve function.
    bh.iter(|| test::black_box(lu.solve(b.clone())))
}

#[bench]
#[cfg(feature = "lapack")]
fn lu_solve_200x200_na_lapack(bh: &mut Bencher) {
    let m  = ::reproductible_dmatrix_na(200, 200);
    let b  = ::reproductible_dvector_na(200);
    let lu = nl::LU::new(m);
    bh.iter(|| test::black_box(lu.solve(&b)))
}

#[bench]
#[cfg(feature = "lapack")]
fn lu_solve_200x200_ndarray_linalg(bh: &mut Bencher) {
    let m  = ::reproductible_dmatrix_ndarray(200, 200);
    let b  = ::reproductible_dvector_ndarray(200);
    let lu = m.factorize_into().unwrap();
    // See the note about the rulinalg lu_solve bench.
    bh.iter(|| test::black_box(lu.solve(&b)))
}


/*
 *
 * 500x500
 *
 */
#[bench]
fn lu_solve_500x500_na(bh: &mut Bencher) {
    let m  = ::reproductible_dmatrix_na(500, 500);
    let b  = ::reproductible_dvector_na(500);
    let lu = m.lu();
    bh.iter(|| test::black_box(lu.solve(&b)))
}


#[bench]
fn lu_solve_500x500_rulinalg(bh: &mut Bencher) {
    let m  = ::reproductible_dmatrix_rulinalg(500, 500);
    let b  = ::reproductible_dvector_rulinalg(500);
    let lu = PartialPivLu::decompose(m).unwrap();
    // NOTE: we do `b.clone()` here because rulinalg takes its argument by-move.
    // There is no extra overhead compared to the nalgebra version (that only pass a reference)
    // because a clone is done internally by the nalgebra LU-solve function.
    bh.iter(|| test::black_box(lu.solve(b.clone())))
}

#[bench]
#[cfg(feature = "lapack")]
fn lu_solve_500x500_na_lapack(bh: &mut Bencher) {
    let m  = ::reproductible_dmatrix_na(500, 500);
    let b  = ::reproductible_dvector_na(500);
    let lu = nl::LU::new(m);
    bh.iter(|| test::black_box(lu.solve(&b)))
}

#[bench]
#[cfg(feature = "lapack")]
fn lu_solve_500x500_ndarray_linalg(bh: &mut Bencher) {
    let m  = ::reproductible_dmatrix_ndarray(500, 500);
    let b  = ::reproductible_dvector_ndarray(500);
    let lu = m.factorize_into().unwrap();
    // See the note about the rulinalg lu_solve bench.
    bh.iter(|| test::black_box(lu.solve(&b)))
}
