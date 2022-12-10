use libspartan::{Instance, VarsAssignment, InputsAssignment};
use curve25519_dalek::scalar::Scalar;

pub fn dwt_baseline_gadget() -> (
    // usize,
    // usize,
    // usize,
    // usize,
    // Instance,
    // VarsAssignment,
    // InputsAssignment
) {
    let num_cons = 11645;
    let num_vars = 12082;
    let num_inputs = 0;
    let num_non_zero_entries = 32768;

    //encode the constraints into three matrices
    let mut A: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut B: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut C: Vec<(usize, usize, [u8; 32])> = Vec::new();

    //define the constraints in dwt_baseline. There are no optimizations in the baseline method.
    //one
    let one = Scalar::one().to_bytes();
    //minus_one
    let minus_one = (-Scalar::one()).to_bytes();
    //zero
    let zero = Scalar::zero().to_bytes();
    //Power of two
    let mut Po2 = Vec::new();
    let mut tp = Scalar::one();
    let two = Scalar::from(2u8);
    Po2.push(tp.to_bytes());
    for i in 0..64 {
        tp = tp * two;
        Po2.push(tp.to_bytes());
    }

    //

}
