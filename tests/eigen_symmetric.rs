#[macro_use]
extern crate linxal;
extern crate ndarray;

use ndarray::{arr1, arr2};
use linxal::types::{Symmetric, Magnitude};
use linxal::eigenvalues::{SymEigen};

#[test]
fn try_eig() {
    let m = arr2(&[[1.0 as f32, 2.0], [2.0, 1.0]]);

    let r = SymEigen::compute_into(m, Symmetric::Upper);
    assert!(r.is_ok());

    assert_eq_within_tol!(r.unwrap(), arr1(&[-1.0, 3.0]), 0.01);
}
