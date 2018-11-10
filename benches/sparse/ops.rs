use test::{self, Bencher};
use na;
use sprs;

/*
 * Sparse matrix multiplication.
 */
#[bench]
fn cs_sparse_mul_100x100_10percent_na(bh: &mut Bencher) {
    let m1 = ::reproductible_csmatrix_na(100, 100, 9000);
    let m2 = ::reproductible_csmatrix_na(100, 100, 9000);
    bh.iter(|| test::black_box(&m1 * &m2))
}

#[bench]
fn cs_sparse_mul_100x100_50percent_na(bh: &mut Bencher) {
    let m1 = ::reproductible_csmatrix_na(100, 100, 5000);
    let m2 = ::reproductible_csmatrix_na(100, 100, 5000);
    bh.iter(|| test::black_box(&m1 * &m2))
}

#[bench]
fn cs_sparse_mul_100x100_100percent_na(bh: &mut Bencher) {
    let m1 = ::reproductible_csmatrix_na(100, 100, 0);
    let m2 = ::reproductible_csmatrix_na(100, 100, 0);
    bh.iter(|| test::black_box(&m1 * &m2))
}

#[bench]
fn cs_sparse_mul_100x100_10percent_sprs(bh: &mut Bencher) {
    let m1 = ::reproductible_csmatrix_sprs(100, 100, 9000);
    let m2 = ::reproductible_csmatrix_sprs(100, 100, 9000);
    bh.iter(|| test::black_box(&m1 * &m2))
}

#[bench]
fn cs_sparse_mul_100x100_50percent_sprs(bh: &mut Bencher) {
    let m1 = ::reproductible_csmatrix_sprs(100, 100, 5000);
    let m2 = ::reproductible_csmatrix_sprs(100, 100, 5000);
    bh.iter(|| test::black_box(&m1 * &m2))
}

#[bench]
fn cs_sparse_mul_100x100_100percent_sprs(bh: &mut Bencher) {
    let m1 = ::reproductible_csmatrix_sprs(100, 100, 0);
    let m2 = ::reproductible_csmatrix_sprs(100, 100, 0);
    bh.iter(|| test::black_box(&m1 * &m2))
}

#[bench]
fn cs_sparse_mul_add20mtx_na(bh: &mut Bencher) {
    let m: na::CsMatrix<f64> = na::io::cs_matrix_from_matrix_market("benches/assets/add20.mtx").unwrap();
    bh.iter(|| test::black_box(&m * &m))
}

#[bench]
fn cs_sparse_mul_add20mtx_sprs(bh: &mut Bencher) {
    let m: sprs::CsMat<f64> = sprs::io::read_matrix_market("benches/assets/add20.mtx").unwrap().to_csc();
    bh.iter(|| test::black_box(&m * &m))
}

/*
 * Sparse matrix addition.
 */
#[bench]
fn cs_sparse_add_100x100_10percent_na(bh: &mut Bencher) {
    let m1 = ::reproductible_csmatrix_na(100, 100, 9000);
    let m2 = ::reproductible_csmatrix_na(100, 100, 9000);
    bh.iter(|| test::black_box(&m1 + &m2))
}

#[bench]
fn cs_sparse_add_100x100_50percent_na(bh: &mut Bencher) {
    let m1 = ::reproductible_csmatrix_na(100, 100, 5000);
    let m2 = ::reproductible_csmatrix_na(100, 100, 5000);
    bh.iter(|| test::black_box(&m1 + &m2))
}

#[bench]
fn cs_sparse_add_100x100_100percent_na(bh: &mut Bencher) {
    let m1 = ::reproductible_csmatrix_na(100, 100, 0);
    let m2 = ::reproductible_csmatrix_na(100, 100, 0);
    bh.iter(|| test::black_box(&m1 + &m2))
}

#[bench]
fn cs_sparse_add_100x100_10percent_sprs(bh: &mut Bencher) {
    let m1 = ::reproductible_csmatrix_sprs(100, 100, 9000);
    let m2 = ::reproductible_csmatrix_sprs(100, 100, 9000);
    bh.iter(|| test::black_box(&m1 + &m2))
}

#[bench]
fn cs_sparse_add_100x100_50percent_sprs(bh: &mut Bencher) {
    let m1 = ::reproductible_csmatrix_sprs(100, 100, 5000);
    let m2 = ::reproductible_csmatrix_sprs(100, 100, 5000);
    bh.iter(|| test::black_box(&m1 + &m2))
}

#[bench]
fn cs_sparse_add_100x100_100percent_sprs(bh: &mut Bencher) {
    let m1 = ::reproductible_csmatrix_sprs(100, 100, 0);
    let m2 = ::reproductible_csmatrix_sprs(100, 100, 0);
    bh.iter(|| test::black_box(&m1 + &m2))
}



#[bench]
fn cs_sparse_add_add20mtx_na(bh: &mut Bencher) {
    let m: na::CsMatrix<f64> = na::io::cs_matrix_from_matrix_market("benches/assets/add20.mtx").unwrap();
    bh.iter(|| test::black_box(&m + &m))
}

#[bench]
fn cs_sparse_add_add20mtx_sprs(bh: &mut Bencher) {
    let m: sprs::CsMat<f64> = sprs::io::read_matrix_market("benches/assets/add20.mtx").unwrap().to_csc();
    bh.iter(|| test::black_box(&m + &m))
}