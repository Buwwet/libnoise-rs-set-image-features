use crate::{constants::*, ptable::build_permutation_table};
use std::sync::Once;

struct StaticPermutationTable {
    table: Option<Vec<usize>>,
    seed: Option<u64>,
    sync: Once,
}

static mut PERMUTATION_TABLE: StaticPermutationTable = StaticPermutationTable {
    table: None,
    seed: None,
    sync: Once::new(),
};

pub fn noise1d(seed: u64, x: f64) -> f64 {
    // no transformation into lattice space required, get cube origin
    let i0 = fast_floor(x);
    // input point relative the two simplex vertices
    let x0 = x - i0;
    let x1 = x0 - 1.0;
    // hashed gradient (-1 or 1) directly, safe because this permutation table cannot index out of bounds
    let i0 = i0 as usize % PERMUTATION_TABLE_SIZE;
    let gi0 = unsafe { hash1d(seed, i0) % GRADIENT_LUT_1D_SIZE };
    let gi1 = unsafe { hash1d(seed, i0 + 1) % GRADIENT_LUT_1D_SIZE };
    // compute contributions, safe because gradient lookup table is known
    let n0 = unsafe { contribution1d(x0, gi0) };
    let n1 = unsafe { contribution1d(x1, gi1) };
    // combine contributions and scale to [-1, 1]
    (n0 + n1) * NORMALIZATION_FACTOR_1D
}

pub fn noise2d(seed: u64, x: f64, y: f64) -> f64 {
    // transform into lattice space and floor for cube origin
    let skew = (x + y) * SKEW_FACTOR_2D;
    let is = fast_floor(x + skew);
    let js = fast_floor(y + skew);
    // input point relative to unskewed cube (and simplex) origin in source space
    let unskew = (is + js) * UNSKEW_FACTOR_2D;
    let x0 = x - is + unskew;
    let y0 = y - js + unskew;
    // compute middle simplex vector(s) between 0-vector and 1-vector
    let mut i1 = 1;
    let mut j1 = 0;
    if x0 < y0 {
        i1 = 0;
        j1 = 1;
    }
    // imput point relative to other unskewed simplex vertices
    let x1 = x0 - i1 as f64 + UNSKEW_FACTOR_2D;
    let y1 = y0 - j1 as f64 + UNSKEW_FACTOR_2D;
    let x2 = x0 - 1.0 + 2.0 * UNSKEW_FACTOR_2D;
    let y2 = y0 - 1.0 + 2.0 * UNSKEW_FACTOR_2D;
    // hashed gradient indices, safe because this permutation table cannot index out of bounds
    let is = is as usize % PERMUTATION_TABLE_SIZE;
    let js = js as usize % PERMUTATION_TABLE_SIZE;
    let gi0 = unsafe { hash2d(seed, is, js) } % GRADIENT_LUT_2D_SIZE;
    let gi1 = unsafe { hash2d(seed, is + i1, js + j1) } % GRADIENT_LUT_2D_SIZE;
    let gi2 = unsafe { hash2d(seed, is + 1, js + 1) } % GRADIENT_LUT_2D_SIZE;
    // compute contributions, safe because gradient lookup table is known
    let n0 = unsafe { contribution2d(x0, y0, gi0) };
    let n1 = unsafe { contribution2d(x1, y1, gi1) };
    let n2 = unsafe { contribution2d(x2, y2, gi2) };
    // combine contributions and scale to [-1, 1]
    (n0 + n1 + n2) * NORMALIZATION_FACTOR_2D
}

pub fn noise3d(seed: u64, x: f64, y: f64, z: f64) -> f64 {
    // transform into lattice space and floor for cube origin
    let skew = (x + y + z) * SKEW_FACTOR_3D;
    let is = fast_floor(x + skew);
    let js = fast_floor(y + skew);
    let ks = fast_floor(z + skew);
    // input point relative to unskewed cube (and simplex) origin in source space
    let unskew = (is + js + ks) * UNSKEW_FACTOR_3D;
    let x0 = x - is + unskew;
    let y0 = y - js + unskew;
    let z0 = z - ks + unskew;
    // compute middle simplex vector(s) between 0-vector and 1-vector
    let idx = (x0 > y0) as usize * 4 + (y0 > z0) as usize * 2 + (x0 > z0) as usize;
    let i1 = SIMPLEX_TRAVERSAL_LUT_3D[idx][0];
    let j1 = SIMPLEX_TRAVERSAL_LUT_3D[idx][1];
    let k1 = SIMPLEX_TRAVERSAL_LUT_3D[idx][2];
    let i2 = SIMPLEX_TRAVERSAL_LUT_3D[idx][3];
    let j2 = SIMPLEX_TRAVERSAL_LUT_3D[idx][4];
    let k2 = SIMPLEX_TRAVERSAL_LUT_3D[idx][5];
    // imput point relative to other unskewed simplex vertices
    let x1 = x0 - i1 as f64 + UNSKEW_FACTOR_3D;
    let y1 = y0 - j1 as f64 + UNSKEW_FACTOR_3D;
    let z1 = z0 - k1 as f64 + UNSKEW_FACTOR_3D;
    let x2 = x0 - i2 as f64 + 2.0 * UNSKEW_FACTOR_3D;
    let y2 = y0 - j2 as f64 + 2.0 * UNSKEW_FACTOR_3D;
    let z2 = z0 - k2 as f64 + 2.0 * UNSKEW_FACTOR_3D;
    let x3 = x0 - 1.0 + 3.0 * UNSKEW_FACTOR_3D;
    let y3 = y0 - 1.0 + 3.0 * UNSKEW_FACTOR_3D;
    let z3 = z0 - 1.0 + 3.0 * UNSKEW_FACTOR_3D;
    // hashed gradient indices, safe because this permutation table cannot index out of bounds
    let is = is as usize % PERMUTATION_TABLE_SIZE;
    let js = js as usize % PERMUTATION_TABLE_SIZE;
    let ks = ks as usize % PERMUTATION_TABLE_SIZE;
    let gi0 = unsafe { hash3d(seed, is, js, ks) } % GRADIENT_LUT_3D_SIZE;
    let gi1 = unsafe { hash3d(seed, is + i1, js + j1, ks + k1) } % GRADIENT_LUT_3D_SIZE;
    let gi2 = unsafe { hash3d(seed, is + i2, js + j2, ks + k2) } % GRADIENT_LUT_3D_SIZE;
    let gi3 = unsafe { hash3d(seed, is + 1, js + 1, ks + 1) } % GRADIENT_LUT_3D_SIZE;
    // compute contributions, safe because gradient lookup table is known
    let n0 = unsafe { contribution3d(x0, y0, z0, gi0) };
    let n1 = unsafe { contribution3d(x1, y1, z1, gi1) };
    let n2 = unsafe { contribution3d(x2, y2, z2, gi2) };
    let n3 = unsafe { contribution3d(x3, y3, z3, gi3) };
    // combine contributions and scale to [-1, 1]
    (n0 + n1 + n2 + n3) * NORMALIZATION_FACTOR_3D
}

fn fast_floor(x: f64) -> f64 {
    let x_int = x as i64;
    x_int as f64 - (x < x_int as f64) as i32 as f64
}

unsafe fn hash1d(seed: u64, i: usize) -> usize {
    let perm = get_permutation_table(seed);
    *perm.get_unchecked(i)
}

unsafe fn hash2d(seed: u64, i: usize, j: usize) -> usize {
    let perm = get_permutation_table(seed);
    *perm.get_unchecked(i + perm.get_unchecked(j))
}

unsafe fn hash3d(seed: u64, i: usize, j: usize, k: usize) -> usize {
    let perm = get_permutation_table(seed);
    *perm.get_unchecked(i + perm.get_unchecked(j + perm.get_unchecked(k)))
}

unsafe fn contribution1d(x: f64, gi: usize) -> f64 {
    if x.abs() >= std::f64::consts::FRAC_1_SQRT_2 {
        0.0
    } else {
        let mut t = R_SQUARED - x * x;
        t *= t;
        t * t * GRADIENT_LUT_1D.get_unchecked(gi) * x
    }
}

unsafe fn contribution2d(x: f64, y: f64, gi: usize) -> f64 {
    let mut t = R_SQUARED - x * x - y * y;
    if t <= 0.0 {
        0.0
    } else {
        let gradient = GRADIENT_LUT_2D.get_unchecked(gi);
        t *= t;
        t * t * (gradient.get_unchecked(0) * x + gradient.get_unchecked(1) * y)
    }
}

unsafe fn contribution3d(x: f64, y: f64, z: f64, gi: usize) -> f64 {
    let mut t = R_SQUARED - x * x - y * y - z * z;
    if t <= 0.0 {
        0.0
    } else {
        let gradient = GRADIENT_LUT_3D.get_unchecked(gi);
        t *= t;
        t * t
            * (gradient.get_unchecked(0) * x
                + gradient.get_unchecked(1) * y
                + gradient.get_unchecked(2) * z)
    }
}

fn get_permutation_table(seed: u64) -> &'static Vec<usize> {
    unsafe {
        if PERMUTATION_TABLE
            .seed
            .is_some_and(|old_seed| old_seed != seed)
        {
            PERMUTATION_TABLE.sync = Once::new();
        }
        PERMUTATION_TABLE.sync.call_once(|| {
            PERMUTATION_TABLE.seed = Some(seed);
            PERMUTATION_TABLE.table =
                Some(build_permutation_table(seed, PERMUTATION_TABLE_SIZE, true));
        });
        PERMUTATION_TABLE.table.as_ref().unwrap()
    }
}
