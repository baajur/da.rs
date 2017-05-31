
extern crate ndarray;
extern crate data_assimilation as da;

use ndarray::*;

#[test]
fn size() {
    let n = 2; // dimension of each state
    let m = 10; // ensemble size
    let x0 = arr1(&[1.0, 2.0]);
    let xs = da::ensemble::Ensemble::isotropic_gaussian(&x0, m, 1.0);
    assert_eq!(xs.dim(), n);
    assert_eq!(xs.size(), m);
    assert_eq!(xs.strides(), [n as isize, 1]);
}

#[test]
fn ensemble_iter() {
    let n = 2; // dimension of each state
    let m = 10; // ensemble size
    let x0 = arr1(&[1.0, 2.0]);
    let mut xs = da::ensemble::Ensemble::isotropic_gaussian(&x0, m, 1.0);
    for v in xs.eiter() {
        assert_eq!(v.len(), n);
    }
    for v in xs.eiter_mut() {
        assert_eq!(v.len(), n);
    }
}
