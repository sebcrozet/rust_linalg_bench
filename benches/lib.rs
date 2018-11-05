#![feature(test)]

extern crate test;
extern crate rand;

extern crate nalgebra as na;
#[cfg(feature = "lapack")]
extern crate nalgebra_lapack as nl;
extern crate cgmath;
extern crate rulinalg;
//extern crate linxal;
extern crate ndarray;
#[cfg(feature = "lapack")]
extern crate ndarray_linalg;


mod lowdim;
mod linalg;

use rand::{Rng, IsaacRng};
use na::dimension::Dynamic;
use na::debug::RandomSDP;
use ndarray::{ShapeBuilder, Array1, Array2};



/*
 *
 * For nalgebra.
 *
 */
fn reproductible_dmatrix_na(nrows: usize, ncols: usize) -> na::DMatrix<f64> {
    let mut rng = IsaacRng::new_unseeded();
    na::DMatrix::<f64>::from_fn(nrows, ncols, |_, _| rng.gen())
}

fn reproductible_dvector_na(dim: usize) -> na::DVector<f64> {
    let mut rng = IsaacRng::new_unseeded();
    na::DVector::<f64>::from_fn(dim, |_, _| rng.gen())
}

fn reproductible_matrix4_na() -> na::Matrix4<f64> {
    let mut rng = IsaacRng::new_unseeded();
    na::Matrix4::<f64>::from_fn(|_, _| rng.gen())
}

fn reproductible_matrix3_na() -> na::Matrix3<f64> {
    let mut rng = IsaacRng::new_unseeded();
    na::Matrix3::<f64>::from_fn(|_, _| rng.gen())
}

fn reproductible_matrix2_na() -> na::Matrix2<f64> {
    let mut rng = IsaacRng::new_unseeded();
    na::Matrix2::<f64>::from_fn(|_, _| rng.gen())
}


fn reproductible_vector4_na() -> na::Vector4<f64> {
    let mut rng = IsaacRng::new_unseeded();
    na::Vector4::<f64>::from_fn(|_, _| rng.gen())
}

fn reproductible_vector3_na() -> na::Vector3<f64> {
    let mut rng = IsaacRng::new_unseeded();
    na::Vector3::<f64>::from_fn(|_, _| rng.gen())
}

fn reproductible_vector2_na() -> na::Vector2<f64> {
    let mut rng = IsaacRng::new_unseeded();
    na::Vector2::<f64>::from_fn(|_, _| rng.gen())
}

fn reproductible_sdp_na(dim: usize) -> na::DMatrix<f64> {
    let mut rng = IsaacRng::new_unseeded();
    RandomSDP::new(Dynamic::new(dim), || rng.gen()).unwrap()
}





/*
 *
 *
 * For cgmath.
 *
 *
 */
fn reproductible_matrix2_cgmath() -> cgmath::Matrix2<f64> {
    let mut rng = IsaacRng::new_unseeded();
    cgmath::Matrix2::new(
        rng.gen(), rng.gen(),
        rng.gen(), rng.gen())
}

fn reproductible_matrix3_cgmath() -> cgmath::Matrix3<f64> {
    let mut rng = IsaacRng::new_unseeded();
    cgmath::Matrix3::new(
        rng.gen(), rng.gen(), rng.gen(),
        rng.gen(), rng.gen(), rng.gen(),
        rng.gen(), rng.gen(), rng.gen())
}

fn reproductible_matrix4_cgmath() -> cgmath::Matrix4<f64> {
    let mut rng = IsaacRng::new_unseeded();
    cgmath::Matrix4::new(
        rng.gen(), rng.gen(), rng.gen(), rng.gen(),
        rng.gen(), rng.gen(), rng.gen(), rng.gen(),
        rng.gen(), rng.gen(), rng.gen(), rng.gen(),
        rng.gen(), rng.gen(), rng.gen(), rng.gen())
}

fn reproductible_vector2_cgmath() -> cgmath::Vector2<f64> {
    let mut rng = IsaacRng::new_unseeded();
    cgmath::Vector2::new(rng.gen(), rng.gen())
}

fn reproductible_vector3_cgmath() -> cgmath::Vector3<f64> {
    let mut rng = IsaacRng::new_unseeded();
    cgmath::Vector3::new(rng.gen(), rng.gen(), rng.gen())
}

fn reproductible_vector4_cgmath() -> cgmath::Vector4<f64> {
    let mut rng = IsaacRng::new_unseeded();
    cgmath::Vector4::new(rng.gen(), rng.gen(), rng.gen(), rng.gen())
}

/*
 *
 *
 * For rulinalg
 *
 *
 */
fn reproductible_dvector_rulinalg(dim: usize) -> rulinalg::vector::Vector<f64> {
    let m = reproductible_dvector_na(dim);
    rulinalg::vector::Vector::from_fn(dim, |i| m[i])
}

fn reproductible_dmatrix_rulinalg(nrows: usize, ncols: usize) -> rulinalg::matrix::Matrix<f64> {
    let m = reproductible_dmatrix_na(nrows, ncols);
    rulinalg::matrix::Matrix::from_fn(nrows, ncols, |j, i| m[(i, j)])
}


fn reproductible_sdp_rulinalg(dim: usize) -> rulinalg::matrix::Matrix<f64> {
    let m = reproductible_sdp_na(dim);
    rulinalg::matrix::Matrix::from_fn(dim, dim, |j, i| m[(i, j)])
}

/*
 *
 *
 * For ndarray
 *
 *
 */
fn reproductible_dvector_ndarray(dim: usize) -> Array1<f64> {
    let m = reproductible_dvector_na(dim);
    Array1::from_shape_fn(dim, |i| m[i])
}

fn reproductible_dmatrix_ndarray(nrows: usize, ncols: usize) -> Array2<f64> {
    let m = reproductible_dmatrix_na(nrows, ncols);
    Array2::from_shape_fn((nrows, ncols), |(i, j)| m[(i, j)])
}


fn reproductible_sdp_ndarray(dim: usize) -> Array2<f64> {
    let m = reproductible_sdp_na(dim);
    Array2::from_shape_fn((dim, dim), |(i, j)| m[(i, j)])
}

fn reproductible_column_major_dmatrix_ndarray(nrows: usize, ncols: usize) -> Array2<f64> {
    let m = reproductible_dmatrix_na(nrows, ncols);
    Array2::from_shape_fn((nrows, ncols).f(), |(i, j)| m[(i, j)])
}


fn reproductible_column_major_sdp_ndarray(dim: usize) -> Array2<f64> {
    let m = reproductible_sdp_na(dim);
    Array2::from_shape_fn((dim, dim).f(), |(i, j)| m[(i, j)])
}
