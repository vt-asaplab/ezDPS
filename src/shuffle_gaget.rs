use libspartan::{Instance, VarsAssignment, InputsAssignment};
use curve25519_dalek::scalar::Scalar;
use std::cmp::min;
use std::env::var;
use std::collections::hash_map::Entry::Vacant;

/// The shuffle gadget is one of the building blocks of our machine_learning pipeline.
/// This function can produce the R1CS instance of shuffle function, which is used in the phase of SVC
/// The constraints can be described as:
/// (x_1 - r) * (x_2 - r) = z_1
/// (x_3 - r) * z_1 = z_2
/// ...
/// (x_s - r) * z_(s-2) = z_(s-1)
/// (y_1 - r) * (y_2 - r) = t_1
/// (y_3 - r) * t_1 = t_2
/// ...
/// (y_s - r) * t_(s-2) = t_(s-1)
/// (z_(s-1) - t_(s-1)) * 1 = 0

pub fn shuffle_gadget() -> (
    usize,
    usize,
    usize,
    usize,
    Instance,
    VarsAssignment,
    InputsAssignment
) {
    let s = 5;
    let num_cons = 2 * s - 1;
    let num_vars = 4 * s - 2;
    let num_inputs = 1;
    let num_non_zero_entries = 4 * s - 2;

    //encode the above constraints into three matrices A, B, C
    let mut A: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut B: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut C: Vec<(usize, usize, [u8; 32])> = Vec::new();

    //The variable that holds a byte representation of 1
    let one = Scalar::one().to_bytes();
    let minus_one = (Scalar::zero() - Scalar::one()).to_bytes();

    //Constraint 0 entries in (A, B, C)
    //We set the order of the variables as: [(vector x), (middle_z), (vector y), (middle t))
    A.push((0, 0, one));
    A.push((0, num_vars+1, minus_one));
    B.push((0, 1, one));
    B.push((0, num_vars+1, minus_one));
    C.push((0, s, one));
    for i in 1..(s-1) {
        A.push((i, i+1, one));
        A.push((i, num_vars+1, minus_one));
        B.push((i, s+i-1, one));
        C.push((i, s+i, one));
    }
    A.push((s-1, 2*s-1, one));
    A.push((s-1, num_vars+1, minus_one));
    B.push((s-1, 2*s, one));
    B.push((s-1, num_vars+1, minus_one));
    C.push((s-1, 3*s-1, one));
    for i in s..2*(s-1) {
        A.push((i, i+s+1, one));
        A.push((i, num_vars+1, minus_one));
        B.push((i, i+2*s-1, one));
        C.push((i, i+2*s, one));
    }
    A.push((2*s-2, 2*s-2, one));
    A.push((2*s-2, 4*s-3, minus_one));
    B.push((2*s-2, num_vars, one));

    let inst = Instance::new(num_cons, num_vars, num_inputs, &A, &B, &C).unwrap();

    //compute a satisfying assignment
    let x = [Scalar::one(), Scalar::from(2u8), Scalar::from(3u8), Scalar::from(4u8), Scalar::from(5u8)];
    let y = [Scalar::from(5u8), Scalar::from(4u8), Scalar::from(3u8), Scalar::from(2u8), Scalar::one()];
    let r = Scalar::one();
    let mut z = Vec::new();
    let mut t = Vec::new();
    z.push((x[0]-r) * (x[1]-r));
    t.push((y[0]-r) * (y[1]-r));
    for i in 0..3 {
        z.push(z[i] * (x[i+2] - r));
        t.push(t[i] * (y[i+2] - r));
    }
    //Create a VarAssignment
    let mut vars = vec![Scalar::zero().to_bytes(); num_vars];
    for i in 0..5 {
        vars[i] = x[i].to_bytes();
        vars[i+9] = y[i].to_bytes();
    }
    for i in 0..4 {
        vars[i+5] = z[i].to_bytes();
        vars[i+14] = t[i].to_bytes();
    }
    let assignment_vars = VarsAssignment::new(&vars).unwrap();

    //Create an InputsAssignment
    let mut inputs = vec![Scalar::zero().to_bytes(); num_inputs];
    inputs[0] = r.to_bytes();
    let assignment_inputs = InputsAssignment::new(&inputs).unwrap();

    //check if the instance we created is satisfiable
    let res = inst.is_sat(&assignment_vars, &assignment_inputs);
    assert_eq!(res.unwrap(), true);

    (
        num_cons,
        num_vars,
        num_inputs,
        num_non_zero_entries,
        inst,
        assignment_vars,
        assignment_inputs
        )
}