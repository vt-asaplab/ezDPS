use libspartan::{Instance, VarsAssignment, InputsAssignment};
use curve25519_dalek::scalar::Scalar;
use crate::load_data::*;
use crate::compute_dwt_output;
use std::env::join_paths;
use crate::compute_nl_outputs::compute_miu_temp;


pub fn nl_gadget() -> (
    usize,
    usize,
    usize,
    usize,
    Instance,
    VarsAssignment,
    VarsAssignment,
    VarsAssignment,
    InputsAssignment,
) {
    //set the parameters
    //the dimension of the input: m+1. e.g., x\in F^170, m=169
    let m = 749;
    let n = 86;
    let N = 24;

    let num_cons = 2 * (m + n + N) + 11;
    let num_vars = 3 * m + 2 * n + 2 * N + 9;
    let num_inputs = 0;
    // let num_non_zero_entries = 168 + 70 * m + (m-1)/2; //52972
    let num_non_zero_entries = 131064;

    //encode the constraints into three matrices
    let mut A: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut B: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut C: Vec<(usize, usize, [u8; 32])> = Vec::new();

    //define the constraints in dwt.
    //one
    let one = Scalar::one().to_bytes();
    //minus_one
    let minus_one = (-Scalar::one()).to_bytes();
    //zero
    let zero = Scalar::zero().to_bytes();
    //inv_m
    let inv_m = Scalar::from(1398u32).to_bytes();
    //unit
    let unit = Scalar::from(1048576u32).to_bytes();
    //unit^2
    let unit_squ = Scalar::from(1099511627776u64).to_bytes();
    //r1
    let r1:u128 = 104* 1048576^4;
    let r1 = Scalar::from(r1).to_bytes();
    //r2
    let r2 = Scalar::from(10485760u32).to_bytes();

    let two = Scalar::from(2u8);

    let mut power_of_two = Scalar::one();
    //Po2 is the powers of two used in the binary vectors
    let mut Po2 = Vec::new();
    Po2.push(Scalar::one().to_bytes());
    //push the powers of alpha and two in the vectors respectively.
    for i in 0..86 {
        power_of_two = power_of_two * two;
        Po2.push(power_of_two.to_bytes());
    }

    //constraint 0
    for i in 0..m {
        A.push((0, i, one));
    }
    B.push((0, num_vars, inv_m));
    C.push((0, m, one));

    //constraint 1 - m
    for i in 0..m {
        A.push((i+1, i, unit));
        A.push((i+1, m, minus_one));
        B.push((i+1, i, unit));
        B.push((i+1, m, minus_one));
        C.push((i+1, m+1+i, one))
    }

    //constraint m+1
    for i in 0..m {
        A.push((m+1, m+1+i, inv_m));
    }
    B.push((m+1, 2*m+3, minus_one));
    C.push((m+1, 3*m+4, one));

    //constraint m+2
    A.push((m+2, 2*m+1, one));
    B.push((m+2, 2*m+1, one));
    C.push((m+2, 2*m+3, one));

    //constraint m+3 - 2m+2
    for i in 0..m {
        A.push((m+3+i, i, unit));
        A.push((m+3+i, m, minus_one));
        B.push((m+3+i, 2*m+2, one));
        C.push((m+3+i, 2*m+4+i, one));
    }

    //constraint 2m+3
    A.push((2*m+3, 2*m+1, one));
    B.push((2*m+3, 2*m+2, one));
    C.push((2*m+3, 3*m+2*n+2*N+8, one));

    //constraint 2m+4
    A.push((2*m+4, 3*m+2*n+2*N+8, one));
    A.push((2*m+4, 3*m+5, one));
    B.push((2*m+4, num_vars, one));
    C.push((2*m+4, num_vars, unit_squ));

    //constraint 2m+5
    for i in 0..n {
        A.push((2*m+5, 3*m+6+i, Po2[i]));
    }
    B.push((2*m+5, num_vars, one));
    C.push((2*m+5, 3*m+4, one));

    //constraint 2m+6 - 2m+5+n
    for i in 0..n {
        A.push((2*m+6+i, 3*m+6+i, one));
        B.push((2*m+6+i, 3*m+6+i, one));
        C.push((2*m+6+i, 3*m+6+i, one));
    }

    //constraint 2m+6+n
    for i in 0..n+1 {
        A.push((2*m+6+n, 3*m+n+6+i, Po2[i]));
    }
    B.push((2*m+6+n, num_vars, one));
    C.push((2*m+6+n, num_vars, r1));
    C.push((2*m+6+n, 3*m+4, minus_one));
    C.push((2*m+6+n, num_vars, Po2[n]));

    //constraint 2*m+7+n - 2*m+2n+6
    for i in 0..n {
        A.push((2*m+n+7+i, 3*m+n+6+i, one));
        B.push((2*m+n+7+i, 3*m+n+6+i, one));
        C.push((2*m+n+7+i, 3*m+n+6+i, one));
    }

    //constraint 2m+2n+7
    A.push((2*m+2*n+7, 3*m+2*n+6, one));
    B.push((2*m+2*n+7, num_vars, one));
    C.push((2*m+2*n+7, num_vars, one));

    //constraint 2m+2n+8
    for i in 0..N {
        A.push((2 * m + 2 * n + 8, 3 * m + 2 * n + 7 + i, Po2[i]));
    }
    B.push((2 * m + 2 * n + 8, num_vars, one));
    C.push((2 * m + 2 * n + 8, 3*m+5, one));

    //constraint 2*m+2*n+9 - constraint 2*m+2*n+8+N
    for i in 0..N {
        A.push((2*m+2*n+9+i, 3*m+2*n+7+i, one));
        B.push((2*m+2*n+9+i, 3*m+2*n+7+i, one));
        C.push((2*m+2*n+9+i, 3*m+2*n+7+i, one));
    }

    //constraint 2m+2n+9+N
    for i in 0..N+1 {
        A.push((3 * m + 2 * n + 9+N, 3 * m + 2 * n + N+7 + i, Po2[i]));
    }
    B.push((3 * m + 2 * n + 9+N, num_vars, one));
    C.push((3 * m + 2 * n + 9+N, num_vars, r2));
    C.push((3 * m + 2 * n + 9+N, 3*m+5, minus_one));
    C.push((3 * m + 2 * n + 9+N, num_vars, Po2[N]));

    //constraint 2m+2n+N+10 - 2m+2n+2N+9
    for i in 0..N {
        A.push((2*m+2*n+N+10+i, 3*m+2*n+N+7+i, one));
        B.push((2*m+2*n+N+10+i, 3*m+2*n+N+7+i, one));
        C.push((2*m+2*n+N+10+i, 3*m+2*n+N+7+i, one));
    }

    //constraint 2m+2n+2N+10
    A.push((2*m+2*n+2*N+10, 3*m+2*n+2*N+7, one));
    B.push((2*m+2*n+2*N+10, num_vars, one));
    C.push((2*m+2*n+2*N+10, num_vars, one));


    //provide the satisfying assignments
    let inst = Instance::new(num_cons, num_vars, num_inputs, &A, &B, &C).unwrap();

    let mut vars_para = vec![];
    let mut vars_input = vec![];
    let mut vars = vec![];

    //PARA 0- m-1, xi
    let path1 = "./data/nl_fixpoint/nl_fixpoint_x.txt";
    let x = load_data_i64(path1);
    for ele in x {
        // println!("{:?}", ele);
        if ele < 0 {
            vars_input.push((-Scalar::from(-ele as u64)).to_bytes());
            vars.push((-Scalar::from(-ele as u64)).to_bytes());
        }
        else {
            vars_input.push(Scalar::from(ele as u64).to_bytes());
            vars.push(Scalar::from(ele as u64).to_bytes());
        }
    }

    //compute the other witnesses
    let (miu, temp, y, epsilon1, epsilon2, r1_ep1_2n, r2_ep2_2N) = compute_miu_temp();

    //PARA m miu
    vars_input.push((Scalar::from(miu as u64)).to_bytes());
    vars.push((Scalar::from(miu as u64)).to_bytes());

    //PARA m+1 - 2m
    for ele in temp {
        if ele < 0 {
            vars_input.push((-Scalar::from(-ele as u128)).to_bytes());
            vars.push((-Scalar::from(-ele as u128)).to_bytes());
        }
        else {
            vars_input.push(Scalar::from(ele as u128).to_bytes());
            vars.push(Scalar::from(ele as u128).to_bytes());
        }
    }

    //PARA 2m+1 sigma
    vars_input.push((Scalar::from(940871u64)).to_bytes());
    vars.push((Scalar::from(940871u64)).to_bytes());

    //PARA 2m+2 inverse sigma
    vars_input.push((Scalar::from(1168609u64)).to_bytes());
    vars.push((Scalar::from(1168609u64)).to_bytes());

    //PARA 2m+3 sigma^2
    vars_input.push((Scalar::from(885238238641u64)).to_bytes());
    vars.push((Scalar::from(885238238641u64)).to_bytes());

    //INPUT_VAR 2m+4 - 3m+3
    for ele in y {
        if ele < 0 {
            vars_input.push((-Scalar::from(-ele as u128)).to_bytes());
            vars.push((-Scalar::from(-ele as u128)).to_bytes());
        }
        else {
            vars_input.push(Scalar::from(ele as u128).to_bytes());
            vars.push(Scalar::from(ele as u128).to_bytes());
        }
    }

    //INPUT_VAR 3m+4
    vars_input.push((Scalar::from(epsilon1 as u128)).to_bytes());
    vars.push((Scalar::from(epsilon1 as u128)).to_bytes());

    //INPUT_VAR 3m+5
    vars_input.push((Scalar::from(epsilon2 as u64)).to_bytes());
    vars.push((Scalar::from(epsilon2 as u64)).to_bytes());



    let assignment_vars_para = VarsAssignment::new(&vars_para).unwrap();
    let padded_vars_para = {
        let num_padded_vars = inst.inst.get_num_vars();
        let num_vars = assignment_vars_para.assignment.len();
        let padded_vars = if num_padded_vars > num_vars {
            assignment_vars_para.pad(num_padded_vars)
        } else {
            assignment_vars_para
        };
        padded_vars
    };

    let assignment_vars_input = VarsAssignment::new(&vars_input).unwrap();
    let padded_vars_input = {
        let num_padded_vars = inst.inst.get_num_vars();
        let num_vars = assignment_vars_input.assignment.len();
        let padded_vars = if num_padded_vars > num_vars {
            assignment_vars_input.pad(num_padded_vars)
        } else {
            assignment_vars_input
        };
        padded_vars
    };

    let assignment_vars = VarsAssignment::new(&vars).unwrap();
    let padded_vars = {
        let num_padded_vars = inst.inst.get_num_vars();
        let num_vars = assignment_vars.assignment.len();
        let padded_vars = if num_padded_vars > num_vars {
            assignment_vars.pad(num_padded_vars)
        } else {
            assignment_vars.clone()
        };
        padded_vars
    };


    let mut inputs = vec![Scalar::zero().to_bytes(); num_inputs];
    let assignment_inputs = InputsAssignment::new(&inputs).unwrap();

    let res = inst.is_sat(&assignment_vars, &assignment_inputs);
    assert_eq!(res.unwrap(), true);

    (
        num_cons,
        num_vars,
        num_inputs,
        num_non_zero_entries,
        inst,
        padded_vars_para,
        padded_vars_input,
        padded_vars,
        assignment_inputs
    )
}