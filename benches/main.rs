use criterion::{criterion_group, criterion_main, Criterion};

const BENCH_ROOT: &'static str = "sightglass/benchmarks/";

macro_rules! def_compile_bench {
    ($func:ident, $name:expr, $with_fuel:ident) => {
        fn $func(c: &mut Criterion) {
            let name = $name.replace("_", "-");
            let root = std::path::Path::new(BENCH_ROOT).join(&name);
            let input = std::fs::read(root.join("benchmark.wasm")).unwrap();
            let input_size = input.len();

            let with_fuel_id = stringify!($with_fuel);
            let with_fuel_id = with_fuel_id.replace("_", "-");
            let id = criterion::BenchmarkId::new(
                &format!("compile:{}:{}", &name, &with_fuel_id),
                format!("{} bytes", input_size),
            );
            let vm = wasm_bench::VM::$with_fuel();

            c.bench_with_input(id, &input, |b, input| {
                b.iter(|| vm.compile(input).unwrap());
            });
        }
    };
}

macro_rules! def_exec_bench {
    ($func:ident, $name:expr, $with_fuel:ident) => {
        fn $func(c: &mut Criterion) {
            let cwd = std::env::current_dir().unwrap();
            let name = $name.replace("_", "-");
            let root = std::path::Path::new(BENCH_ROOT).join(&name);
            let with_fuel_id = stringify!($with_fuel);
            let with_fuel_id = with_fuel_id.replace("_", "-");

            assert!(std::env::set_current_dir(&root).is_ok());
            let input = std::fs::read("benchmark.wasm").unwrap();
            let input_size = input.len();
            let id = criterion::BenchmarkId::new(
                &format!("exec:{}:{}", &name, &with_fuel_id),
                format!("{} bytes", input_size),
            );
            let vm = wasm_bench::VM::$with_fuel();
            let mut store = vm.make_store();
            let bytes = vm.compile(&input).unwrap();
            let module = vm.deserialize(&bytes).unwrap();

            c.bench_with_input(id, &module, |b, module| {
                b.iter(|| {
                    vm.exec(&mut store, &module).unwrap();
                });
            });
            assert!(std::env::set_current_dir(&cwd).is_ok());
        }
    };
}

/// An iterator macro that defines the benchmarking matrix.
/// Benchmark entries should be of the form:
/// @<compile | exec> <default | with_fuel> <benchmark function definition> => <benchmark_directory>
macro_rules! for_each_bench {
    ($mac:ident) => {
        $mac! {
            @compile default bls12_381_pairing => bls12_381_pairing
            @compile with_fuel bls12_381_pairing_with_fuel => bls12_381_pairing
            @compile default bls12_381_msm_g1_10 => bls12_381_msm_g1_10
            @compile with_fuel bls12_381_msm_g1_10_with_fuel => bls12_381_msm_g1_10
            @compile default bls12_381_msm_g1_1000 => bls12_381_msm_g1_1000
            @compile with_fuel bls12_381_msm_g1_1000_with_fuel => bls12_381_msm_g1_1000
            @compile default bls12_381_msm_g2_10 => bls12_381_msm_g2_10
            @compile with_fuel bls12_381_msm_g2_10_with_fuel => bls12_381_msm_g2_10
            @compile default bls12_381_msm_g2_1000 => bls12_381_msm_g2_1000
            @compile with_fuel bls12_381_msm_g2_1000_with_fuel => bls12_381_msm_g2_1000
            @compile default bls12_381_mul_affine_g1 => bls12_381_mul_affine_g1
            @compile with_fuel bls12_381_mul_affine_g1_with_fuel => bls12_381_mul_affine_g1
            @compile default bls12_381_mul_affine_g2 => bls12_381_mul_affine_g2
            @compile with_fuel bls12_381_mul_affine_g2_with_fuel => bls12_381_mul_affine_g2
            @compile default bls12_381_mul_projective_g1 => bls12_381_mul_projective_g1
            @compile with_fuel bls12_381_mul_projective_g1_with_fuel => bls12_381_mul_projective_g1
            @compile default bls12_381_mul_projective_g2 => bls12_381_mul_projective_g2
            @compile with_fuel bls12_381_mul_projective_g2_with_fuel => bls12_381_mul_projective_g2
            @compile default bls12_377_pairing => bls12_377_pairing
            @compile with_fuel bls12_377_pairing_with_fuel => bls12_377_pairing
            @compile default bls12_377_msm_g1_10 => bls12_377_msm_g1_10
            @compile with_fuel bls12_377_msm_g1_10_with_fuel => bls12_377_msm_g1_10
            @compile default bls12_377_msm_g1_1000 => bls12_377_msm_g1_1000
            @compile with_fuel bls12_377_msm_g1_1000_with_fuel => bls12_377_msm_g1_1000
            @compile default bls12_377_msm_g2_10 => bls12_377_msm_g2_10
            @compile with_fuel bls12_377_msm_g2_10_with_fuel => bls12_377_msm_g2_10
            @compile default bls12_377_msm_g2_1000 => bls12_377_msm_g2_1000
            @compile with_fuel bls12_377_msm_g2_1000_with_fuel => bls12_377_msm_g2_1000
            @compile default bls12_377_mul_affine_g1 => bls12_377_mul_affine_g1
            @compile with_fuel bls12_377_mul_affine_g1_with_fuel => bls12_377_mul_affine_g1
            @compile default bls12_377_mul_affine_g2 => bls12_377_mul_affine_g2
            @compile with_fuel bls12_377_mul_affine_g2_with_fuel => bls12_377_mul_affine_g2
            @compile default bls12_377_mul_projective_g1 => bls12_377_mul_projective_g1
            @compile with_fuel bls12_377_mul_projective_g1_with_fuel => bls12_377_mul_projective_g1
            @compile default bls12_377_mul_projective_g2 => bls12_377_mul_projective_g2
            @compile with_fuel bls12_377_mul_projective_g2_with_fuel => bls12_377_mul_projective_g2
            @compile default bw6_761_pairing => bw6_761_pairing
            @compile with_fuel bw6_761_pairing_with_fuel => bw6_761_pairing
            @compile default bw6_761_msm_g1_10 => bw6_761_msm_g1_10
            @compile with_fuel bw6_761_msm_g1_10_with_fuel => bw6_761_msm_g1_10
            @compile default bw6_761_msm_g1_1000 => bw6_761_msm_g1_1000
            @compile with_fuel bw6_761_msm_g1_1000_with_fuel => bw6_761_msm_g1_1000
            @compile default bw6_761_msm_g2_10 => bw6_761_msm_g2_10
            @compile with_fuel bw6_761_msm_g2_10_with_fuel => bw6_761_msm_g2_10
            @compile default bw6_761_msm_g2_1000 => bw6_761_msm_g2_1000
            @compile with_fuel bw6_761_msm_g2_1000_with_fuel => bw6_761_msm_g2_1000
            @compile default bw6_761_mul_affine_g1 => bw6_761_mul_affine_g1
            @compile with_fuel bw6_761_mul_affine_g1_with_fuel => bw6_761_mul_affine_g1
            @compile default bw6_761_mul_affine_g2 => bw6_761_mul_affine_g2
            @compile with_fuel bw6_761_mul_affine_g2_with_fuel => bw6_761_mul_affine_g2
            @compile default bw6_761_mul_projective_g1 => bw6_761_mul_projective_g1
            @compile with_fuel bw6_761_mul_projective_g1_with_fuel => bw6_761_mul_projective_g1
            @compile default bw6_761_mul_projective_g2 => bw6_761_mul_projective_g2
            @compile with_fuel bw6_761_mul_projective_g2_with_fuel => bw6_761_mul_projective_g2
            @compile default ed_on_bls12_381_mul_affine_sw => ed_on_bls12_381_mul_affine_sw
            @compile with_fuel ed_on_bls12_381_mul_affine_sw_with_fuel => ed_on_bls12_381_mul_affine_sw
            @compile default ed_on_bls12_377_mul_affine => ed_on_bls12_377_mul_affine
            @compile with_fuel ed_on_bls12_377_mul_affine_with_fuel => ed_on_bls12_377_mul_affine

            @exec default exec_bls12_381_pairing => bls12_381_pairing
            @exec with_fuel exec_bls12_381_pairing_with_fuel => bls12_381_pairing
            @exec default exec_bls12_381_msm_g1_10 => bls12_381_msm_g1_10
            @exec with_fuel exec_bls12_381_msm_g1_10_with_fuel => bls12_381_msm_g1_10
            @exec default exec_bls12_381_msm_g1_1000 => bls12_381_msm_g1_1000
            @exec with_fuel exec_bls12_381_msm_g1_1000_with_fuel => bls12_381_msm_g1_1000
            @exec default exec_bls12_381_msm_g2_10 => bls12_381_msm_g2_10
            @exec with_fuel exec_bls12_381_msm_g2_10_with_fuel => bls12_381_msm_g2_10
            @exec default exec_bls12_381_msm_g2_1000 => bls12_381_msm_g2_1000
            @exec with_fuel exec_bls12_381_msm_g2_1000_with_fuel => bls12_381_msm_g2_1000
            @exec default exec_bls12_381_mul_affine_g1 => bls12_381_mul_affine_g1
            @exec with_fuel exec_bls12_381_mul_affine_g1_with_fuel => bls12_381_mul_affine_g1
            @exec default exec_bls12_381_mul_affine_g2 => bls12_381_mul_affine_g2
            @exec with_fuel exec_bls12_381_mul_affine_g2_with_fuel => bls12_381_mul_affine_g2
            @exec default exec_bls12_381_mul_projective_g1 => bls12_381_mul_projective_g1
            @exec with_fuel exec_bls12_381_mul_projective_g1_with_fuel => bls12_381_mul_projective_g1
            @exec default exec_bls12_381_mul_projective_g2 => bls12_381_mul_projective_g2
            @exec with_fuel exec_bls12_381_mul_projective_g2_with_fuel => bls12_381_mul_projective_g2
            @exec default exec_bls12_377_pairing => bls12_377_pairing
            @exec with_fuel exec_bls12_377_pairing_with_fuel => bls12_377_pairing
            @exec default exec_bls12_377_msm_g1_10 => bls12_377_msm_g1_10
            @exec with_fuel exec_bls12_377_msm_g1_10_with_fuel => bls12_377_msm_g1_10
            @exec default exec_bls12_377_msm_g1_1000 => bls12_377_msm_g1_1000
            @exec with_fuel exec_bls12_377_msm_g1_1000_with_fuel => bls12_377_msm_g1_1000
            @exec default exec_bls12_377_msm_g2_10 => bls12_377_msm_g2_10
            @exec with_fuel exec_bls12_377_msm_g2_10_with_fuel => bls12_377_msm_g2_10
            @exec default exec_bls12_377_mul_affine_g1 => bls12_377_mul_affine_g1
            @exec with_fuel exec_bls12_377_mul_affine_g1_with_fuel => bls12_377_mul_affine_g1
            @exec default exec_bls12_377_mul_affine_g2 => bls12_377_mul_affine_g2
            @exec with_fuel exec_bls12_377_mul_affine_g2_with_fuel => bls12_377_mul_affine_g2
            @exec default exec_bls12_377_mul_projective_g1 => bls12_377_mul_projective_g1
            @exec with_fuel exec_bls12_377_mul_projective_g1_with_fuel => bls12_377_mul_projective_g1
            @exec default exec_bls12_377_mul_projective_g2 => bls12_377_mul_projective_g2
            @exec with_fuel exec_bls12_377_mul_projective_g2_with_fuel => bls12_377_mul_projective_g2
            @exec default exec_bw6_761_pairing => bw6_761_pairing
            @exec with_fuel exec_bw6_761_pairing_with_fuel => bw6_761_pairing
            @exec default exec_bw6_761_msm_g1_10 => bw6_761_msm_g1_10
            @exec with_fuel exec_bw6_761_msm_g1_10_with_fuel => bw6_761_msm_g1_10
            @exec default exec_bw6_761_msm_g1_1000 => bw6_761_msm_g1_1000
            @exec with_fuel exec_bw6_761_msm_g1_1000_with_fuel => bw6_761_msm_g1_1000
            @exec default exec_bw6_761_msm_g2_10 => bw6_761_msm_g2_10
            @exec with_fuel exec_bw6_761_msm_g2_10_with_fuel => bw6_761_msm_g2_10
            @exec default exec_bw6_761_msm_g2_1000 => bw6_761_msm_g2_1000
            @exec with_fuel exec_bw6_761_msm_g2_1000_with_fuel => bw6_761_msm_g2_1000
            @exec default exec_bw6_761_mul_affine_g1 => bw6_761_mul_affine_g1
            @exec with_fuel exec_bw6_761_mul_affine_g1_with_fuel => bw6_761_mul_affine_g1
            @exec default exec_bw6_761_mul_affine_g2 => bw6_761_mul_affine_g2
            @exec with_fuel exec_bw6_761_mul_affine_g2_with_fuel => bw6_761_mul_affine_g2
            @exec default exec_bw6_761_mul_projective_g1 => bw6_761_mul_projective_g1
            @exec with_fuel exec_bw6_761_mul_projective_g1_with_fuel => bw6_761_mul_projective_g1
            @exec default exec_bw6_761_mul_projective_g2 => bw6_761_mul_projective_g2
            @exec with_fuel exec_bw6_761_mul_projective_g2_with_fuel => bw6_761_mul_projective_g2
            @exec default exec_ed_on_bls12_381_mul_affine_sw => ed_on_bls12_381_mul_affine_sw
            @exec with_fuel exec_ed_on_bls12_381_mul_affine_sw_with_fuel => ed_on_bls12_381_mul_affine_sw
            @exec default exec_ed_on_bls12_377_mul_affine => ed_on_bls12_377_mul_affine
            @exec with_fuel exec_ed_on_bls12_377_mul_affine_with_fuel => ed_on_bls12_377_mul_affine
        }
    };
}

/// Macro to define either a compilation or execution benchmark.
///
/// Benchmarks defined through this macro, will follow a specific
/// naming convention;
/// * Compilation benchmarks with fuel: `cargo run -- bench --name=compile:<name>:with_fuel`
/// * Compilation benchmarks without fuel: `cargo run -- bench --name=compile:<name>:default`
/// * Execution benchmarks with fuel: `cargo run -- bench --name=exec:<name>:with_fuel`
/// * Compilation benchmarks without fuel: `cargo run -- bench --name=exec:<name>:default`
macro_rules! def_bench {
    ( @compile $ty:ident $def:ident => $name:ident $($rest:tt)* ) => {
	def_compile_bench!($def, stringify!($name), $ty);
	def_bench!($($rest)*);
    };

    ( @exec $ty:ident $def:ident => $name:ident $($rest:tt)* ) => {
	def_exec_bench!($def, stringify!($name), $ty);
	def_bench!($($rest)*);
    };

    () => {};
}

for_each_bench!(def_bench);

criterion_group!(
    benches,
    bls12_381_pairing,
    bls12_381_pairing_with_fuel,
    bls12_381_msm_g1_10,
    bls12_381_msm_g1_10_with_fuel,
    bls12_381_msm_g1_1000,
    bls12_381_msm_g1_1000_with_fuel,
    bls12_381_msm_g2_10,
    bls12_381_msm_g2_10_with_fuel,
    bls12_381_msm_g2_1000,
    bls12_381_msm_g2_1000_with_fuel,
    bls12_381_mul_affine_g1,
    bls12_381_mul_affine_g1_with_fuel,
    bls12_381_mul_affine_g2,
    bls12_381_mul_affine_g2_with_fuel,
    bls12_381_mul_projective_g1,
    bls12_381_mul_projective_g1_with_fuel,
    bls12_381_mul_projective_g2,
    bls12_381_mul_projective_g2_with_fuel,
    bls12_377_pairing,
    bls12_377_pairing_with_fuel,
    bls12_377_msm_g1_10,
    bls12_377_msm_g1_10_with_fuel,
    bls12_377_msm_g1_1000,
    bls12_377_msm_g1_1000_with_fuel,
    bls12_377_msm_g2_10,
    bls12_377_msm_g2_10_with_fuel,
    bls12_377_msm_g2_1000,
    bls12_377_msm_g2_1000_with_fuel,
    bls12_377_mul_affine_g1,
    bls12_377_mul_affine_g1_with_fuel,
    bls12_377_mul_affine_g2,
    bls12_377_mul_affine_g2_with_fuel,
    bls12_377_mul_projective_g1,
    bls12_377_mul_projective_g1_with_fuel,
    bls12_377_mul_projective_g2,
    bls12_377_mul_projective_g2_with_fuel,
    bw6_761_pairing,
    bw6_761_pairing_with_fuel,
    bw6_761_msm_g1_10,
    bw6_761_msm_g1_10_with_fuel,
    bw6_761_msm_g1_1000,
    bw6_761_msm_g1_1000_with_fuel,
    bw6_761_msm_g2_10,
    bw6_761_msm_g2_10_with_fuel,
    bw6_761_msm_g2_1000,
    bw6_761_msm_g2_1000_with_fuel,
    bw6_761_mul_affine_g1,
    bw6_761_mul_affine_g1_with_fuel,
    bw6_761_mul_affine_g2,
    bw6_761_mul_affine_g2_with_fuel,
    bw6_761_mul_projective_g1,
    bw6_761_mul_projective_g1_with_fuel,
    bw6_761_mul_projective_g2,
    bw6_761_mul_projective_g2_with_fuel,
    ed_on_bls12_381_mul_affine_sw,
    ed_on_bls12_381_mul_affine_sw_with_fuel,
    ed_on_bls12_377_mul_affine,
    ed_on_bls12_377_mul_affine_with_fuel,
    exec_bls12_381_pairing,
    exec_bls12_381_pairing_with_fuel,
    exec_bls12_377_pairing,
    exec_bls12_377_pairing_with_fuel,
    exec_bw6_761_pairing,
    exec_bw6_761_pairing_with_fuel,
    exec_ed_on_bls12_381_mul_affine_sw,
    exec_ed_on_bls12_381_mul_affine_sw_with_fuel,
    exec_ed_on_bls12_377_mul_affine,
    exec_ed_on_bls12_377_mul_affine_with_fuel,
);

criterion_main!(benches);
