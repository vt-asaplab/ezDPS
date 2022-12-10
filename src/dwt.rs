use libspartan::{Instance, VarsAssignment, InputsAssignment};
use curve25519_dalek::scalar::Scalar;
use crate::load_data::*;
use crate::compute_dwt_output;
use std::env::join_paths;

pub fn dwt_gadget() -> (
    usize,
    usize,
    usize,
    usize,
    Instance,
    VarsAssignment,
    InputsAssignment
) {
    //set the parameters
    let num_cons = 11645;
    let num_vars = 12082;
    let num_inputs = 0;
    let num_non_zero_entries = 32768;

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
    //Alpha is the coefficients in dwt decomp and recons. 1^0, 1^1, ..., 1^85, (1^85-1)
    let mut Alpha = Vec::new();
    Alpha.push(Scalar::one().to_bytes());
    let alp = Scalar::one();
    let two = Scalar::from(2u8);
    let mut power_of_alp = Scalar::one();
    let mut power_of_two = Scalar::one();
    //Po2 is the powers of two used in the binary vectors
    let mut Po2 = Vec::new();
    Po2.push(Scalar::one().to_bytes());
    //push the powers of alpha and two in the vectors respectively.
    for i in 0..85 {
        power_of_alp = power_of_alp * alp;
        Alpha.push(power_of_alp.to_bytes());
        if i < 64 {
            power_of_two = power_of_two * two;
            Po2.push(power_of_two.to_bytes());
        }
        if i == 84 {
            let alp_85_minus_1 = power_of_alp - Scalar::one();
            Alpha.push(alp_85_minus_1.to_bytes());
        }
    }
    //constraint 0
    A.push((0, 340, Alpha[1]));
    A.push((0, 342, one));
    for k in 0..85 {
        B.push((0, 2 * k, Alpha[k]));
    }
    C.push((0, 348, one));

    //constraint 1
    A.push((1, 341, Alpha[1]));
    A.push((1, 343, one));
    for k in 0..85 {
        B.push((1, 2 * k + 1, Alpha[k]));
    }
    C.push((1, 349, one));

    //constraint 2
    A.push((2, 344, Alpha[1]));
    A.push((2, 346, one));
    for k in 0..85 {
        B.push((2, 2 * k, Alpha[k]));
    }
    C.push((2, 350, one));

    //constraint 3
    A.push((3, 345, Alpha[1]));
    A.push((3, 347, one));
    for k in 0..85 {
        B.push((3, 2 * k + 1, Alpha[k]));
    }
    C.push((3, 351, one));

    //constraint 4
    A.push((4, 342, Alpha[86]));
    B.push((4, 0, one));
    C.push((4, 352, one));

    //constraint 5
    A.push((5, 343, Alpha[86]));
    B.push((5, 1, one));
    C.push((5, 353, one));

    //constraint 6
    A.push((6, 346, Alpha[86]));
    B.push((6, 0, one));
    C.push((6, 354, one));

    //constraint 7
    A.push((7, 347, Alpha[86]));
    B.push((7, 1, one));
    C.push((7, 355, one));

    //constraint 8
    A.push((8, 348, one));
    A.push((8, 349, one));
    A.push((8, 352, one));
    A.push((8, 353, one));
    B.push((8, num_vars, one));
    for i in 0..85 {
        C.push((8, 170 + i, Alpha[i + 1]));
    }

    //constraint 9
    A.push((9, 350, one));
    A.push((9, 351, one));
    A.push((9, 354, one));
    A.push((9, 355, one));
    B.push((9, num_vars, one));
    for i in 0..85 {
        C.push((9, 255 + i, Alpha[i + 1]));
    }

    //constraint 10
    for i in 0..64 {
        A.push((10, 356 + i, Po2[i]));
    }
    B.push((10, num_vars, one));
    C.push((10, 420, one));

    //constraint 11 - constraint 74
    for i in 0..64 {
        A.push((i+11, 356+i, one));
        B.push((i+11, 356+i, one));
        C.push((i+11, 356+i, one));
    }

    //constraint 75 - constraint 159
    for i in 0..85 {
        A.push((75+i, 421+i, one));
        A.push((75+i, 255+i, minus_one));
        B.push((75+i, 421+i, one));
        B.push((75+i, 255+i, one));
        C.push((75+i, num_vars, zero));
    }

    //constraint 160 - constraint 244
    for i in 0..85 {
        for j in 0..64 {
            A.push((160+i, 64*i+591+j, Po2[j]));
        }
        B.push((160+i, num_vars, one));
        C.push((160+i, 421 + i, one));
    }

    //constraint 245 - constraint 329
    for i in 0..85 {
        for j in 0..65 {
            A.push((245+i, 65*i+6031+j, Po2[j]));
        }
        B.push((245+i, num_vars, one));
        C.push((245+i, 421+i, one));
        C.push((245+i, 420, minus_one));
        //TODO
        C.push((245+i, num_vars, Po2[64]));
    }

    //constraint 330 - constraint 414
    for i in 0..85 {
        A.push((330+i, 506+i, one));
        B.push((330+i, 421+i, one));
        B.push((330+i, 420, minus_one));
        C.push((330+i, 11556+i, one));
    }

    //constraint 415 - constraint 499
    for i in 0..85 {
        A.push((415+i, 6095+65*i, one));
        B.push((415+i, 11811+i, one));
        B.push((415+i, 11556+i, minus_one));
        C.push((415+i, 11641+i, one));
    }

    //constraint 500 - constraint 584
    for i in 0..85 {
        A.push((500+i, num_vars, one));
        A.push((500+i, 6095 + 65*i, minus_one));
        B.push((500+i, 11811+i, one));
        C.push((500+i, 11726+i, one));
    }

    //constraint 585 - constraint 669
    for i in 0..85 {
        A.push((585+i, 11641+i, one));
        B.push((585+i, 11726+i, one));
        C.push((585+i, num_vars, zero));
    }

    //constraint 670 - constraint 6109
    for i in 0..5440 {
        A.push((670+i, 591+i, one));
        B.push((670+i, 591+i, one));
        C.push((670+i, 591+i, one));
    }

    //constraint 6110 - constraint 11634
    for i in 0..5525 {
        A.push((6110+i, 6031+i, one));
        B.push((6110+i, 6031+i, one));
        C.push((6110+i, 6031+i, one));
    }

    //constraint 11635
    A.push((11635, 11896, Alpha[1]));
    A.push((11635, 11898, one));
    for i in 0..85 {
        B.push((11635, 170+i, Alpha[i]))
    }
    C.push((11635, 11904, one));

    //constraint 11636
    A.push((11636, 11897, Alpha[1]));
    A.push((11636, 11899, one));
    for i in 0..85 {
        B.push((11636, 11811+i, Alpha[i]))
    }
    C.push((11636, 11905, one));

    //constraint 11637
    A.push((11637, 11900, Alpha[1]));
    A.push((11637, 11902, one));
    for i in 0..85 {
        B.push((11637, 170+i, Alpha[i]))
    }
    C.push((11637, 11906, one));

    //constraint 11638
    A.push((11638, 11901, Alpha[1]));
    A.push((11638, 11903, one));
    for i in 0..85 {
        B.push((11638, 11811+i, Alpha[i]))
    }
    C.push((11638, 11907, one));

    //constraint 11639
    A.push((11639, 11898, Alpha[86]));
    B.push((11639, 170, one));
    C.push((11639, 11908, one));

//constraint 11640
    A.push((11640, 11899, Alpha[86]));
    B.push((11640, 11811, one));
    C.push((11640, 11909, one));

//constraint 11641
    A.push((11641, 11902, Alpha[86]));
    B.push((11641, 170, one));
    C.push((11641, 11910, one));

//constraint 11642
    A.push((11642, 11903, Alpha[86]));
    B.push((11642, 11811, one));
    C.push((11642, 11911, one));


//constraint 11643
    A.push((11643, 11904, one));
    A.push((11643, 11905, one));
    A.push((11643, 11908, one));
    A.push((11643, 11909, one));
    B.push((11643, num_vars, one));
    for k in 1..85 {
        C.push((11643, 11912+2*k, Alpha[k]));
    }
    C.push((11643, 11912, Alpha[85]));

    //constraint 11644
    A.push((11644, 11906, one));
    A.push((11644, 11907, one));
    A.push((11644, 11910, one));
    A.push((11644, 11911, one));
    B.push((11644, num_vars, one));
    for k in 1..85 {
        C.push((11644, 11912+2*k+1, Alpha[k]));
    }
    C.push((11644, 11913, Alpha[85]));



    //provide the satisfying assignments
    let inst = Instance::new(num_cons, num_vars, num_inputs, &A, &B, &C).unwrap();
    let mut vars = Vec::new();
    //import variables 0-169
    let path1 = "./data/dwt_fixpoint/dwt_input_fixpoint.txt";
    let x = load_data_i64(path1);
    for ele in x {
        // println!("{:?}", ele);
        if ele < 0 {
            vars.push((-Scalar::from(-ele as u64)).to_bytes());
        }
        else { vars.push(Scalar::from(ele as u64).to_bytes()); }
    }

    let path2 = "./data/dwt_fixpoint/dwt_dec_result_fixpoint.txt";
    let y = load_data_i64(path2);
    for ele in y {
        if ele < 0 {
            vars.push((-Scalar::from(-ele as u64)).to_bytes());
        }
        else { vars.push(Scalar::from(ele as u64).to_bytes()); }
    }

    let path3 = "./data/dwt_fixpoint/dwt_hg_fixpoint.txt";
    let hg = load_data_i64(path3);
    for ele in hg {
        if ele < 0 {
            vars.push((-Scalar::from(-ele as u64)).to_bytes());
        }
        else { vars.push(Scalar::from(ele as u64).to_bytes()); }
    }

    let path4 = "./data/dwt_fixpoint/dwt_temp0to7_fixpoint.txt";
    let temp0to7 = load_data_i64(path4);
    for ele in temp0to7 {
        if ele < 0 {
            vars.push((-Scalar::from(-ele as u64)).to_bytes());
        }
        else { vars.push(Scalar::from(ele as u64).to_bytes()); }
    }

    let path5 = "./data/dwt_fixpoint/lambda_bit.txt";
    let lambda0to63 = load_data_i64(path5);
    for ele in lambda0to63 {
        if ele < 0 {
            vars.push((-Scalar::from(-ele as u64)).to_bytes());
        }
        else { vars.push(Scalar::from(ele as u64).to_bytes()); }
    }

    let lambda:i64 = 3355443;
    vars.push(Scalar::from(lambda as u64).to_bytes());

    let path6 = "./data/dwt_fixpoint/abs_y_prime.txt";
    let abs_y = load_data_i64(path6);
    for ele in abs_y {
        if ele < 0 {
            vars.push((-Scalar::from(-ele as u64)).to_bytes());
        }
        else { vars.push(Scalar::from(ele as u64).to_bytes()); }
    }

    let path7 = "./data/dwt_fixpoint/sign_y_prime.txt";
    let sign_y = load_data_i64(path7);
    for ele in sign_y {
        if ele < 0 {
            vars.push((-Scalar::from(-ele as u64)).to_bytes());
        }
        else { vars.push(Scalar::from(ele as u64).to_bytes()); }
    }

    let path8 = "./data/dwt_fixpoint/cij_fixpoint/cij_new.txt";
    let cij = load_data_cd(path8);
    for ele in cij {
        if ele < 0 {
            vars.push((-Scalar::from(-ele as u64)).to_bytes());
        }
        else { vars.push(Scalar::from(ele as u64).to_bytes()); }
    }

    let path9 = "./data/dwt_fixpoint/dij_fixpoint/dij_new.txt";
    let dij = load_data_cd(path9);
    for ele in dij {
        if ele < 0 {
            vars.push((-Scalar::from(-ele as u64)).to_bytes());
        }
        else { vars.push(Scalar::from(ele as u64).to_bytes()); }
    }

    let path10 = "./data/dwt_fixpoint/dwt_e1.txt";
    let e1 = load_data_i64(path10);
    for ele in e1 {
        if ele < 0 {
            vars.push((-Scalar::from(-ele as u64)).to_bytes());
        }
        else { vars.push(Scalar::from(ele as u64).to_bytes()); }
    }

    let path11 = "./data/dwt_fixpoint/dwt_e2.txt";
    let e2 = load_data_i64(path11);
    for ele in e2 {
        if ele < 0 {
            vars.push((-Scalar::from(-ele as u64)).to_bytes());
        }
        else { vars.push(Scalar::from(ele as u64).to_bytes()); }
    }

    let path12 = "./data/dwt_fixpoint/dwt_e3.txt";
    let e3 = load_data_i64(path12);
    for ele in e3 {
        if ele < 0 {
            vars.push((-Scalar::from(-ele as u64)).to_bytes());
        }
        else { vars.push(Scalar::from(ele as u64).to_bytes()); }
    }

    let path13 = "./data/dwt_fixpoint/dwt_thr_result_fixpoint.txt";
    let y_prime = load_data_i64(path13);
    for ele in y_prime {
        if ele < 0 {
            vars.push((-Scalar::from(-ele as u64)).to_bytes());
        }
        else { vars.push(Scalar::from(ele as u64).to_bytes()); }
    }

    let path14 = "./data/dwt_fixpoint/dwt_IhIg_fixpoint.txt";
    let IhIg = load_data_i64(path14);
    for ele in IhIg {
        if ele < 0 {
            vars.push((-Scalar::from(-ele as u64)).to_bytes());
        }
        else { vars.push(Scalar::from(ele as u64).to_bytes()); }
    }

    let path15 = "./data/dwt_fixpoint/dwt_temp8to15_fixpoint.txt";
    let temp8to15 = load_data_i128(path15);
    for ele in temp8to15 {
        println!("{:?}", ele);
        if ele < 0 {
            vars.push((-Scalar::from(-ele as u128)).to_bytes());
        }
        else { vars.push(Scalar::from(ele as u128).to_bytes()); }
    }


    // let path16 = "./data/dwt_fixpoint/dwt_recons_result_fixpoint.txt";
    // let x_prime = load_data_i128(path16);
    let x_prime = compute_dwt_output::compute_x_prime();
    for ele in x_prime {
        if ele < 0 {
            vars.push((-Scalar::from(-ele as u128)).to_bytes());
        }
        else { vars.push(Scalar::from(ele as u128).to_bytes()); }
    }


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