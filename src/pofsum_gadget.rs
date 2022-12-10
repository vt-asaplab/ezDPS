use libspartan::{Instance, VarsAssignment, InputsAssignment};
use curve25519_dalek::scalar::Scalar;

pub fn pofsum_gadget() -> (
    // usize,
    // usize,
    // usize,
    // usize,
    // Instance,
    // VarsAssignment,
    // InputsAssignment,
) {
    //set the parameters, k is the output dimension, and m is the input dimension
    let k = 9;
    let m = 170;
    let num_cons = (k + 1) * m + 1;
    let num_vars = (2 * k + 2) * m + k;
    let num_inputs = k;
    let num_non_zero_entries = 0;

    //encode the above constraints into three matrices
    let mut A: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut B: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut C: Vec<(usize, usize, [u8; 32])> = Vec::new();

    // one
    let one = Scalar::one().to_bytes();

    // construct the constraints. We set the order of the variables is ((u), (x), (z), (y))
    // where u is the parameters, x is the input vector, y is the output and z represents the middle vector

    for j in 0..k {
        for i in 0..m {
            let cons = m * j + i; //represents the number of constraints
            let u_number = m * j + i; //represents the number of u parameter
            let alpha = num_vars + j + 1; //represents the number of alpha
            let outz = (k + j + 1) * m + i; //represents the number of out z
            A.push((cons, u_number, one));
            B.push((cons, alpha, one));
            C.push((cons, outz, one));
        }
    }

    for i in 0..m {
        let cons = k * m + i;
        for j in 0..k {
            A.push((cons, (k + i + 1) * m + j, one));
        }
        B.push((cons, k * m + i, one));
        C.push((cons, (2 * k + 1) * m + i, one));
    }

    for i in 0..m {
        A.push(((k+1) * m, (2 * k + 1) * m + i, one));
        C.push(((k+1) * m, (2 * k + 2) * m + i, one));
    }
    B.push(((k+1) * m, num_vars, one));

    let inst = Instance::new(num_cons, num_vars, num_inputs, &A, &B, &C).unwrap();

    //give an satisfying assignment





}