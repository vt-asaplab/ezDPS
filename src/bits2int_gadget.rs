use libspartan::{Instance, VarsAssignment, InputsAssignment};
use curve25519_dalek::scalar::Scalar;
use std::env::var;


/// This function is one of the building blocks of our machine-learning pipeline.
/// The goal of this function is to produce the R1CS instance for the conversion between bits vector
/// and integers.
/// The constraints can be described as:
/// b0 * 2^0 + b1 * 2^1 + b2 * 2^2 + ... + b255 * 2^255 = x
/// bi * bi = bi (i in 0..255)

pub fn bis2int_gadget() -> (
    usize,
    usize,
    usize,
    usize,
    Instance,
    VarsAssignment,
    InputsAssignment,
){
    //set the length of the bit vector
    let n = 4;
    //parameters of the R1CS instance
    let num_cons = n + 1;
    let num_vars = n + 1;
    let num_inputs = 0;
    let num_non_zero_entries = n + 1;

    //encode the above constraints into three matrices A, B, C
    let mut A: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut B: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut C: Vec<(usize, usize, [u8; 32])> = Vec::new();

    // a variable that holds a byte representation of 1
    let mut two_base = Scalar::one();
    let one = Scalar::one().to_bytes();

    //set the matrices. Suppose the order of the variables is (b0, b1, ..., b255, x)
    for i in 0..n as i32 {
        A.push((0, i as usize, two_base.to_bytes()));
        two_base = two_base * Scalar::from(2u32);
    }
    B.push((0, num_vars, one));
    C.push((0, n, one));

    for i in 1..(n+1) {
        A.push((i, i-1, one));
        B.push((i, i-1, one));
        C.push((i, i-1, one));
    }

    let inst = Instance::new(num_cons, num_vars, num_inputs, &A, &B, &C).unwrap();

    //set a satisfying assignment
    let mut bits = [Scalar::zero(), Scalar::zero(), Scalar::one(), Scalar::one()];

    let x = Scalar::from(12u32);

    //create a VarsAssignment
    let mut vars = vec![Scalar::zero().to_bytes(); num_vars];
    for i in 0..4 {
        vars[i] = bits[i].to_bytes();
    }
    vars[n] = x.to_bytes();
    let assignment_vars = VarsAssignment::new(&vars).unwrap();

    // create an InputAssignment
    let mut inputs = vec![Scalar::zero().to_bytes(); num_inputs];
    let assignment_inputs = InputsAssignment::new(&inputs).unwrap();

    // check if the instance we created is satisfiable
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