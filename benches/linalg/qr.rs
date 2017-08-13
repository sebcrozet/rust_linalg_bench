use test::{self, Bencher};
use nl;
use linxal::prelude::*;
use ndarray_linalg::qr::QRSquareInto;
use rulinalg::matrix::decomposition::{Decomposition, HouseholderQr};

/*
 *
 *
 * 100x100
 *
 *
 */
#[bench]
fn qr_100x100_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 100);
    bh.iter(|| test::black_box(m.clone().qr()))
}


#[bench]
fn qr_100x100_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(100, 100);
    bh.iter(|| test::black_box(HouseholderQr::decompose(m.clone())))
}

#[bench]
fn qr_100x100_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 100);
    bh.iter(|| test::black_box(nl::QR::new(m.clone())))
}

#[bench]
fn qr_100x100_linxal(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_ndarray(100, 100);
    bh.iter(|| test::black_box(m.qr()))
}

#[bench]
fn qr_unpack_100x100_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 100);
    bh.iter(|| test::black_box(m.clone().qr().unpack()))
}


#[bench]
fn qr_unpack_100x100_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(100, 100);
    bh.iter(|| test::black_box(HouseholderQr::decompose(m.clone()).unpack()))
}

#[bench]
fn qr_unpack_100x100_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 100);
    bh.iter(|| test::black_box(nl::QR::new(m.clone()).unpack()))
}

#[bench]
fn qr_unpack_100x100_ndarray_linalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_ndarray(100, 100);
    bh.iter(|| test::black_box(QRSquareInto::qr_square_into(m.clone())))
}

/*
 *
 *
 * 200x200
 *
 *
 */
#[bench]
fn qr_200x200_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(200, 200);
    bh.iter(|| test::black_box(m.clone().qr()))
}


#[bench]
fn qr_200x200_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(200, 200);
    bh.iter(|| test::black_box(HouseholderQr::decompose(m.clone())))
}

#[bench]
fn qr_200x200_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(200, 200);
    bh.iter(|| test::black_box(nl::QR::new(m.clone())))
}

#[bench]
fn qr_200x200_linxal(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_ndarray(200, 200);
    bh.iter(|| test::black_box(m.qr()))
}


#[bench]
fn qr_unpack_200x200_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(200, 200);
    bh.iter(|| test::black_box(m.clone().qr().unpack()))
}


#[bench]
fn qr_unpack_200x200_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(200, 200);
    bh.iter(|| test::black_box(HouseholderQr::decompose(m.clone()).unpack()))
}


#[bench]
fn qr_unpack_200x200_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(200, 200);
    bh.iter(|| test::black_box(nl::QR::new(m.clone()).unpack()))
}


#[bench]
fn qr_unpack_200x200_ndarray_linalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_ndarray(200, 200);
    bh.iter(|| test::black_box(QRSquareInto::qr_square_into(m.clone())))
}


/*
 *
 *
 * 100x500
 *
 *
 */
#[bench]
fn qr_100x500_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 500);
    bh.iter(|| test::black_box(m.clone().qr()))
}


#[bench]
fn qr_100x500_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(100, 500);
    bh.iter(|| test::black_box(HouseholderQr::decompose(m.clone())))
}

#[bench]
fn qr_100x500_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 500);
    bh.iter(|| test::black_box(nl::QR::new(m.clone())))
}

#[bench]
fn qr_100x500_linxal(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_ndarray(100, 500);
    bh.iter(|| test::black_box(m.qr()))
}


#[bench]
fn qr_unpack_100x500_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 500);
    bh.iter(|| test::black_box(m.clone().qr().unpack()))
}


#[bench]
fn qr_unpack_100x500_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(100, 500);
    bh.iter(|| test::black_box(HouseholderQr::decompose(m.clone()).unpack()))
}

#[bench]
fn qr_unpack_100x500_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(100, 500);
    bh.iter(|| test::black_box(nl::QR::new(m.clone()).unpack()))
}


/*
 *
 *
 * 500x100
 *
 *
 */
#[bench]
fn qr_500x100_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(500, 100);
    bh.iter(|| test::black_box(m.clone().qr()))
}


#[bench]
fn qr_500x100_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(500, 100);
    bh.iter(|| test::black_box(HouseholderQr::decompose(m.clone())))
}

#[bench]
fn qr_500x100_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(500, 100);
    bh.iter(|| test::black_box(nl::QR::new(m.clone())))
}


#[bench]
fn qr_500x100_linxal(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_ndarray(500, 100);
    bh.iter(|| test::black_box(m.qr()))
}

#[bench]
fn qr_unpack_500x100_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(500, 100);
    bh.iter(|| test::black_box(m.clone().qr().unpack()))
}


#[bench]
fn qr_unpack_500x100_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(500, 100);
    bh.iter(|| test::black_box(HouseholderQr::decompose(m.clone()).unpack()))
}


#[bench]
fn qr_unpack_500x100_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(500, 100);
    bh.iter(|| test::black_box(nl::QR::new(m.clone()).unpack()))
}



/*
 *
 *
 * 500x500
 *
 *
 */
#[bench]
fn qr_500x500_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(500, 500);
    bh.iter(|| test::black_box(m.clone().qr()))
}


#[bench]
fn qr_500x500_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(500, 500);
    bh.iter(|| test::black_box(HouseholderQr::decompose(m.clone())))
}


#[bench]
fn qr_500x500_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(500, 500);
    bh.iter(|| test::black_box(nl::QR::new(m.clone())))
}

#[bench]
fn qr_500x500_linxal(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_ndarray(500, 500);
    bh.iter(|| test::black_box(m.qr()))
}


#[bench]
fn qr_unpack_500x500_na(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(500, 500);
    bh.iter(|| test::black_box(m.clone().qr().unpack()))
}


#[bench]
fn qr_unpack_500x500_rulinalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_rulinalg(500, 500);
    bh.iter(|| test::black_box(HouseholderQr::decompose(m.clone()).unpack()))
}

#[bench]
fn qr_unpack_500x500_na_lapack(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_na(500, 500);
    bh.iter(|| test::black_box(nl::QR::new(m.clone()).unpack()))
}

#[bench]
fn qr_unpack_500x500_ndarray_linalg(bh: &mut Bencher) {
    let m = ::reproductible_dmatrix_ndarray(500, 500);
    bh.iter(|| test::black_box(QRSquareInto::qr_square_into(m.clone())))
}
