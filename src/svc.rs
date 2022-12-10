use libspartan::{Instance, VarsAssignment, InputsAssignment};
use curve25519_dalek::scalar::Scalar;
use crate::load_data::{load_data_i64, load_data_2d_i64, load_data_i128};
use crate::svc_compute_b;
use crate::svc_compute_d;
use crate::svc_compute_d::compute_d;

pub fn svc_gadget() -> (
    usize,
    usize,
    usize,
    usize,
    Instance,
    VarsAssignment,
    InputsAssignment
) {
    //set the parameters
    // dox is the dimension of x
    let dox = 9;
    // nos is the number of support vectors
    let nos = 545;
    let num_cons = 75756;
    // let num_vars = (2*nos+1) * dox + 130*nos + 3;
    let num_vars = 262144;
    let num_inputs = 0;
    let num_non_zero_entries = 262144;

    //encode the constraints into three matrices
    let mut A: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut B: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut C: Vec<(usize, usize, [u8; 32])> = Vec::new();

    // set the constants.
    //one
    let one = Scalar::one().to_bytes();
    // -one
    let minus_one = (-Scalar::one()).to_bytes();
    // Powers of two, Po2[i]=2^i
    let mut Po2 = Vec::new();
    let mut temp = Scalar::one();
    let two = Scalar::from(2u8);
    Po2.push(temp.to_bytes());
    for i in 0..63 {
        temp = temp * two;
        Po2.push(temp.to_bytes());
    }
    // Powers of e^(-1). Poe[i] = e^(2^l). 为了避免溢出，这里取e等于1
    let mut Poe = Vec::new();
    let mut e = Scalar::from(1u32);
    Poe.push(e.to_bytes());
    for i in 0..63 {
        e = e * e;
        Poe.push(e.to_bytes())
    }

    //construct the constraints
    // dox * nos - 1 => [0, 4904]
    for i in 0..nos {
        for j in 0..dox {
            A.push((i * dox + j, j, one));
            A.push((i * dox + j, (i+1)*dox + j, minus_one));
            B.push((i * dox + j, j, one));
            B.push((i * dox + j, (i+1)*dox + j, minus_one));
            C.push((i * dox + j, (nos+i+1)*dox+j, one));
        }
    }

    // dox * nos, (dox+1) * nos - 1 => [4905, 5449]
    for i in 0..nos {
        for j in 0..dox {
            A.push((nos * dox + i, (nos+i+1)*dox+j, one));
        }
        B.push((nos * dox + i, (2*nos+1)*dox + 130*nos+2, one));
        C.push((nos * dox + i, (2*nos+1)*dox+127*nos + i, one));
    }

    // (dox+1) * nos, (dox+2) * nos - 1 => [5450, 5994]
    for i in 0..nos {
        for t in 0..64 {
            A.push(((dox+1)*nos + i, (2*nos+1)*dox+64*i + t, Po2[t]));
        }
        B.push(((dox+1)*nos + i, num_vars, one));
        C.push(((dox+1)*nos + i, (2*nos+1)*dox+127*nos + i, one));
    }

    // define kp0 to denote the keypoint: (dox+2)*nos = 5995
    let kp0 = (dox + 2) * nos;
    // kp, kp+64*nos-1 => [5995, 40874]
    for i in 0..nos {
        for t in 0..64 {
            A.push((kp0+64*i+t, (2*nos+1)*dox+64*i+t, one));
            B.push((kp0+64*i+t, (2*nos+1)*dox+64*i+t, one));
            C.push((kp0+64*i+t, (2*nos+1)*dox+64*i+t, one));
        }
    }

    // define kp1 to denote the keypoint: (dox+66)*nos = 40875
    let kp1 = (dox+66) * nos;
    // kp1, kp1 + (nos+1) * 63 - 1 => [40875, 75209]
    for i in 0..nos {
        A.push((kp1+63*i, (2*nos+1)*dox + 64*i, Poe[0]));
        A.push((kp1+63*i, num_vars, one));
        A.push((kp1+63*i, (2*nos+1)*dox + 64*i, minus_one));
        B.push((kp1+63*i, (2*nos+1)*dox + 64*i + 1, Poe[1]));
        B.push((kp1+63*i, num_vars, one));
        B.push((kp1+63*i, (2*nos+1)*dox + 64*i + 1, minus_one));
        C.push((kp1+63*i, (2*nos+1)*dox + 64*nos + 63*i, one));
        for t in 1..63 {
            A.push((kp1+63*i+t, (2*nos+1)*dox + 64*nos + 63*i + t - 1, one));
            B.push((kp1+63*i+t, (2*nos+1)*dox + 64*i + t + 1, Poe[t+1]));
            B.push((kp1+63*i+t, num_vars, one));
            B.push((kp1+63*i+t, (2*nos+1)*dox + 64*i + t + 1, minus_one));
            C.push((kp1+63*i+t, (2*nos+1)*dox + 64*nos + 63*i + t, one));
        }
    }

    //define kp2 to denote the keypoint: 40875+nos*63 = 75210
    let kp2 = kp1 + nos * 63;
    //kp2, kp2+nos-1
    for i in 0..nos {
        A.push((kp2+i, (2*nos+1)*dox + 129*nos + i, one));
        B.push((kp2+i, (2*nos+1)*dox + 64*nos + 63*i + 62, one));
        C.push((kp2+i, (2*nos+1)*dox + 128*nos + i, one));
    }

    //constraint kp2 + nos = 75755
    for i in 0..nos {
        A.push((kp2 + nos, (2*nos+1) * dox + 128*nos + i, one));
    }
    A.push((kp2 + nos, (2*nos+1) * dox + 130 * nos, one));
    B.push((kp2 + nos, num_vars, one));
    C.push((kp2 + nos, (2*nos+1) * dox + 130 * nos + 1, one));

    ///
    ///



    // provide the satisfying assignments
    let inst = Instance::new(num_cons, num_vars, num_inputs, &A, &B, &C).unwrap();
    let mut vars = Vec::new();

    //import variables x
    let path1 = "./data/svc_fixpoint/svc_input_fixpoint.txt";
    let x = load_data_i64(path1);
    for ele in x {
        // println!("{:?}", ele);
        if ele < 0 {
            vars.push((-Scalar::from(-ele as u64)).to_bytes());
        }
        else { vars.push(Scalar::from(ele as u64).to_bytes()); }
    }

    //import variables sv
    let path2 = "./data/svc_fixpoint/support_vectors_fixpoint.txt";
    let sv = load_data_2d_i64(path2);
    for rows in sv {
        for ele in rows {
            if ele < 0 {
                vars.push((-Scalar::from(-ele as u64)).to_bytes());
            }
            else { vars.push(Scalar::from(ele as u64).to_bytes()) };
        }
    }

    //import variables temp
    let path3 = "./data/svc_fixpoint/svc_temp.txt";
    let temp = load_data_2d_i64(path3);
    for rows in temp {
        for ele in rows {
            if ele < 0 {
                vars.push((-Scalar::from(-ele as u64)).to_bytes());
            }
            else { vars.push(Scalar::from(ele as u64).to_bytes()) };
        }
    }

    //import dij
    // let path4 = "./data/svc_fixpoint/dij_fixpoint.txt";
    // let dij = load_data_2d_i64(path4);
    let dij = svc_compute_d::compute_dij();
    for rows in dij {
        for ele in rows {
            if ele < 0 {
                vars.push((-Scalar::from(-ele as u64)).to_bytes());
            }
            else { vars.push(Scalar::from(ele as u64).to_bytes()) };
        }
    }

//import mij
    let path5 = "./data/svc_fixpoint/svc_mij.txt";
    let mij = load_data_2d_i64(path5);
    for rows in mij {
        for ele in rows {
            if ele < 0 {
                vars.push((-Scalar::from(-ele as u64)).to_bytes());
            }
            else { vars.push(Scalar::from(ele as u64).to_bytes()) };
        }
    }

//import d
//     let path6 = "./data/svc_fixpoint/svc_d.txt";
//     let d = load_data_i64(path6);
    let d = svc_compute_d::compute_d();
    for ele in d {
        // println!("{:?}", ele);
        if ele < 0 {
            vars.push((-Scalar::from(-ele as u64)).to_bytes());
        }
        else { vars.push(Scalar::from(ele as u64).to_bytes()); }
    }


//import alpha
    let path7 = "./data/svc_fixpoint/alpha_fixpoint.txt";
    let alpha = load_data_i64(path7);
    for ele in alpha {
        // println!("{:?}", ele);
        if ele < 0 {
            vars.push((-Scalar::from(-ele as u64)).to_bytes());
        }
        else { vars.push(Scalar::from(ele as u64).to_bytes()); }
    }

//import dc
    let path8 = "./data/svc_fixpoint/dual_coef_fixpoint.txt";
    let dc = load_data_i64(path8);
    for ele in dc {
        // println!("{:?}", ele);
        if ele < 0 {
            vars.push((-Scalar::from(-ele as u64)).to_bytes());
        }
        else { vars.push(Scalar::from(ele as u64).to_bytes()); }
    }

    //import b
    let b = -211063;
    vars.push((-Scalar::from(-b as u32)).to_bytes());

    //import y
    let y = svc_compute_b::compute_out();

    if y < 0 {
        vars.push((-Scalar::from(-y as u128)).to_bytes());
    }
    else { vars.push(Scalar::from(y as u128).to_bytes()); }

    //import lambda
    let lambda = 29127;
    vars.push(Scalar::from(lambda as u32).to_bytes());



    ///
    ///





    let assignment_vars = VarsAssignment::new(&vars).unwrap();
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
        assignment_vars,
        assignment_inputs
        )
}