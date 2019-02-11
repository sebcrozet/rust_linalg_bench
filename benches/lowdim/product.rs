use std::ops::Mul;
use test::{self, Bencher};
use cgmath::InnerSpace;

#[path="../macros.rs"]
mod macros;

/*
 *
 * nalgebra
 *
 */
bench_binop!(mat4_mul_mat4_na, mul, reproductible_matrix4_na, reproductible_matrix4_na);
bench_binop!(mat3_mul_mat3_na, mul, reproductible_matrix3_na, reproductible_matrix3_na);
bench_binop!(mat2_mul_mat2_na, mul, reproductible_matrix2_na, reproductible_matrix2_na);

bench_binop!(mat4_mul_vec4_na, mul, reproductible_matrix4_na, reproductible_vector4_na);
bench_binop!(mat3_mul_vec3_na, mul, reproductible_matrix3_na, reproductible_vector3_na);
bench_binop!(mat2_mul_vec2_na, mul, reproductible_matrix2_na, reproductible_vector2_na);


bench_binop_ref!(vec4_dot_vec4_na, dot, reproductible_vector4_na, reproductible_vector4_na);
bench_binop_ref!(vec3_dot_vec3_na, dot, reproductible_vector3_na, reproductible_vector3_na);
bench_binop_ref!(vec2_dot_vec2_na, dot, reproductible_vector2_na, reproductible_vector2_na);


/*
 *
 * cgmath
 *
 */
bench_binop!(mat4_mul_mat4_cgmath, mul, reproductible_matrix4_cgmath, reproductible_matrix4_cgmath);
bench_binop!(mat3_mul_mat3_cgmath, mul, reproductible_matrix3_cgmath, reproductible_matrix3_cgmath);
bench_binop!(mat2_mul_mat2_cgmath, mul, reproductible_matrix2_cgmath, reproductible_matrix2_cgmath);

bench_binop!(mat4_mul_vec4_cgmath, mul, reproductible_matrix4_cgmath, reproductible_vector4_cgmath);
bench_binop!(mat3_mul_vec3_cgmath, mul, reproductible_matrix3_cgmath, reproductible_vector3_cgmath);
bench_binop!(mat2_mul_vec2_cgmath, mul, reproductible_matrix2_cgmath, reproductible_vector2_cgmath);

bench_binop!(vec4_dot_vec4_cgmath, dot, reproductible_vector4_cgmath, reproductible_vector4_cgmath);
bench_binop!(vec3_dot_vec3_cgmath, dot, reproductible_vector3_cgmath, reproductible_vector3_cgmath);
bench_binop!(vec2_dot_vec2_cgmath, dot, reproductible_vector2_cgmath, reproductible_vector2_cgmath);


/*
 *
 * vek
 *
 */
bench_binop!(mat4_mul_mat4_vek, mul, reproductible_matrix4_vek, reproductible_matrix4_vek);
bench_binop!(mat3_mul_mat3_vek, mul, reproductible_matrix3_vek, reproductible_matrix3_vek);
bench_binop!(mat2_mul_mat2_vek, mul, reproductible_matrix2_vek, reproductible_matrix2_vek);

bench_binop!(mat4_mul_vec4_vek, mul, reproductible_matrix4_vek, reproductible_vector4_vek);
bench_binop!(mat3_mul_vec3_vek, mul, reproductible_matrix3_vek, reproductible_vector3_vek);
bench_binop!(mat2_mul_vec2_vek, mul, reproductible_matrix2_vek, reproductible_vector2_vek);

bench_binop!(vec4_dot_vec4_vek, dot, reproductible_vector4_vek, reproductible_vector4_vek);
bench_binop!(vec3_dot_vec3_vek, dot, reproductible_vector3_vek, reproductible_vector3_vek);
bench_binop!(vec2_dot_vec2_vek, dot, reproductible_vector2_vek, reproductible_vector2_vek);
