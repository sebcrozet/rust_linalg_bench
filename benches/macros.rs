#![macro_use]

macro_rules! bench_unop(
    ($name: ident,  $op: ident, $gen_a: ident) => {
        #[bench]
        fn $name(bench: &mut Bencher) {
            let a = ::$gen_a();
        
            bench.iter(|| {
                test::black_box(a.$op())
            })
        }
    }
);


macro_rules! bench_binop(
    ($name: ident,  $op: ident, $gen_a: ident, $gen_b: ident) => {
        #[bench]
        fn $name(bench: &mut Bencher) {
            let a = ::$gen_a();
            let b = ::$gen_b();
        
            bench.iter(|| {
                test::black_box(a.$op(b))
            })
        }
    }
);

macro_rules! bench_binop_ref(
    ($name: ident,  $op_ref: ident, $gen_a: ident, $gen_b: ident) => {
        #[bench]
        fn $name(bench: &mut Bencher) {
            let a = ::$gen_a();
            let b = ::$gen_b();
        
            bench.iter(|| {
                test::black_box(a.$op_ref(&b))
            })
        }
    }
);
