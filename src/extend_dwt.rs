use libspartan::{Instance, VarsAssignment, InputsAssignment};
use curve25519_dalek::scalar::Scalar;
use crate::load_data::*;
use crate::compute_dwt_output;
use std::env::join_paths;
use crate::compute_dwt_output::{compute_dij, comupute_temp8to15};

pub fn dwt_gadget() -> (
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
    let half_m = (m-1) / 2;
    let num_cons = 158 + 68 * m;
    let num_vars = 168 + 70 * m + (m-1)/2;
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
    //Alpha is the coefficients in dwt decomp and recons.
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
    for i in 0..half_m+1 {
        power_of_alp = power_of_alp * alp;
        Alpha.push(power_of_alp.to_bytes());
        if i < 64 {
            power_of_two = power_of_two * two;
            Po2.push(power_of_two.to_bytes());
        }
        if i == half_m {
            let alp_last_minus_1 = power_of_alp - Scalar::one();
            Alpha.push(alp_last_minus_1.to_bytes());
        }
    }
    //constraint 0
    A.push((0, 0, Alpha[1]));
    A.push((0, 2, one));
    for k in 0..half_m+1 {
        B.push((0, 81 + 2 * k, Alpha[k]));
    }
    C.push((0, 83 + 2 * m, one));

    //constraint 1
    A.push((1, 1, Alpha[1]));
    A.push((1, 3, one));
    for k in 0..half_m+1 {
        B.push((1, 82 + 2 * k, Alpha[k]));
    }
    C.push((1, 84+2*m, one));

    //constraint 2
    A.push((2, 4, Alpha[1]));
    A.push((2, 6, one));
    for k in 0..half_m+1 {
        B.push((2, 81 + 2 * k, Alpha[k]));
    }
    C.push((2, 85+2*m, one));

    //constraint 3
    A.push((3, 5, Alpha[1]));
    A.push((3, 7, one));
    for k in 0..half_m+1 {
        B.push((3, 82 + 2 * k, Alpha[k]));
    }
    C.push((3, 86+2*m, one));

    //constraint 4
    A.push((4, 2, Alpha[half_m+2]));
    B.push((4, 81, one));
    C.push((4, 87 + 2*m, one));

    //constraint 5
    A.push((5, 3, Alpha[half_m+2]));
    B.push((5, 82, one));
    C.push((5, 88+2*m, one));

    //constraint 6
    A.push((6, 6, Alpha[half_m+2]));
    B.push((6, 81, one));
    C.push((6, 89+2*m, one));

    //constraint 7
    A.push((7, 7, Alpha[half_m+2]));
    B.push((7, 82, one));
    C.push((7, 90+2*m, one));

    //constraint 8
    A.push((8, 83+2*m, one));
    A.push((8, 84+2*m, one));
    A.push((8, 87+2*m, one));
    A.push((8, 88+2*m, one));
    B.push((8, num_vars, one));
    for i in 0..half_m+1 {
        C.push((8, 82+m+i, Alpha[i + 1]));
    }

    //constraint 9
    A.push((9, 85+2*m, one));
    A.push((9, 86+2*m, one));
    A.push((9, 89+2*m, one));
    A.push((9, 90+2*m, one));
    B.push((9, num_vars, one));
    for i in 0..half_m+1 {
        C.push((9, m+half_m+83+i, Alpha[i + 1]));
    }

    //constraint 10
    for i in 0..64 {
        A.push((10, 16 + i, Po2[i]));
    }
    B.push((10, num_vars, one));
    C.push((10, 80, one));

    //constraint 11 - constraint 74
    for i in 0..64 {
        A.push((i+11, 16+i, one));
        B.push((i+11, 16+i, one));
        C.push((i+11, 16+i, one));
    }

    //constraint 75 - constraint 75+half_m
    for i in 0..half_m+1 {
        A.push((75+i, 91+2*m+i, one));
        A.push((75+i, m+half_m+83+i, minus_one));
        B.push((75+i, 91+2*m+i, one));
        B.push((75+i, m+half_m+83+i, one));
        C.push((75+i, num_vars, zero));
    }

    //constraint 76+half_m - constraint 75+m
    for i in 0..half_m+1 {
        for j in 0..64 {
            A.push((76+half_m+i, 92+3*m+half_m*j+j+i, Po2[j]));
        }
        B.push((76+half_m+i, num_vars, one));
        C.push((76+half_m+i, 91+2*m+i, one));
    }

    //constraint 76+m - constraint 76+m+half_m
    for i in 0..half_m+1 {
        for j in 0..65 {
            A.push((76+m+i, 124+35*m+half_m*j+j+i, Po2[j]));
        }
        B.push((76+m+i, num_vars, one));
        C.push((76+m+i, 91+2*m+i, one));
        C.push((76+m+i, 80, minus_one));
        //TODO
        C.push((76+m+i, num_vars, Po2[64]));
    }

    //constraint 77+m+half_m - constraint 76+2m
    for i in 0..half_m+1 {
        A.push((77+m+half_m+i, 92+2*m+half_m+i, one));
        B.push((77+m+half_m+i, 91+2*m+i, one));
        B.push((77+m+half_m+i, 80, minus_one));
        C.push((77+m+half_m+i, 157+67*m+half_m+i, one));
    }

    //constraint 77+2*m - constraint 77+2*m+half_m
    for i in 0..half_m+1 {
        A.push((77+2*m+i, 156+67*m+i, one));
        B.push((77+2*m+i, 158+69*m+i, one));
        B.push((77+2*m+i, 157+67*m+half_m+i, minus_one));
        C.push((77+2*m+i, 157+68*m+i, one));
    }

    //constraint 78+2*m+half_m - constraint 77+3*m
    for i in 0..half_m+1 {
        A.push((78+2*m+half_m+i, num_vars, one));
        A.push((78+2*m+half_m+i, 156+67*m+i, minus_one));
        B.push((78+2*m+half_m+i, 158+69*m+i, one));
        C.push((78+2*m+half_m+i, 158+68*m+half_m+i, one));
    }

    //constraint 78+3m - constraint 78+3*m+half_m
    for i in 0..half_m+1 {
        A.push((78+3*m+i, 157+68*m+i, one));
        A.push((78+3*m+i, 158+68*m+half_m+i, one));
        B.push((78+3*m+i, num_vars, one));
        C.push((78+3*m+i, num_vars, zero));
    }

    //constraint 79+3*m+half_m - constraint 110+35*m+half_m
    for i in 0..(half_m+1)*129{
        A.push((79+3*m+half_m+i, 92+3*m+i, one));
        B.push((79+3*m+half_m+i, 92+3*m+i, one));
        C.push((79+3*m+half_m+i, 92+3*m+i, one));
    }

    //reconstruction
    //constraint 143+68*m
    A.push((143+68*m, 8, Alpha[1]));
    A.push((143+68*m, 10, one));
    for i in 0..half_m+1 {
        B.push((143+68*m, 82+m+i, Alpha[i]))
    }
    C.push((143+68*m, 160+70*m+half_m, one));

    //constraint 144+68*m
    A.push((144+68*m, 9, Alpha[1]));
    A.push((144+68*m, 11, one));
    for i in 0..half_m+1 {
        B.push((144+68*m, 158+69*m+i, Alpha[i]))
    }
    C.push((144+68*m, 161+70*m+half_m, one));

    //constraint 145+68*m
    A.push((145+68*m, 12, Alpha[1]));
    A.push((145+68*m, 14, one));
    for i in 0..half_m+1 {
        B.push((145+68*m, 82+m+i, Alpha[i]))
    }
    C.push((145+68*m, 162+70*m+half_m, one));

    //constraint 146+68*m
    A.push((146+68*m, 13, Alpha[1]));
    A.push((146+68*m, 15, one));
    for i in 0..half_m+1 {
        B.push((146+68*m, 158+69*m+i, Alpha[i]))
    }
    C.push((146+68*m, 163+70*m+half_m, one));

    //constraint 147+68*m
    A.push((147+68*m, 10, Alpha[half_m+2]));
    B.push((147+68*m, 82+m, one));
    C.push((147+68*m, 164+70*m+half_m, one));

//constraint 148+68*m
    A.push((148+68*m, 11, Alpha[half_m+2]));
    B.push((148+68*m, 158+69*m, one));
    C.push((148+68*m, 165+70*m+half_m, one));

//constraint 149+68*m
    A.push((149+68*m, 14, Alpha[half_m+2]));
    B.push((149+68*m, 82+m, one));
    C.push((149+68*m, 166+70*m+half_m, one));

//constraint 150+68*m
    A.push((150+68*m, 15, Alpha[half_m+2]));
    B.push((150+68*m, 158+69*m, one));
    C.push((150+68*m, 167+70*m+half_m, one));


//constraint 151+68*m
    A.push((151+68*m, 160+70*m+half_m, one));
    A.push((151+68*m, 161+70*m+half_m, one));
    A.push((151+68*m, 164+70*m+half_m, one));
    A.push((151+68*m, 165+70*m+half_m, one));
    B.push((151+68*m, num_vars, one));
    for k in 1..half_m+1 {
        C.push((151+68*m, 159+69*m+half_m+2*k, Alpha[k]));
    }
    C.push((151+68*m, 159+69*m+half_m, Alpha[half_m+1]));

    //constraint 152+68*m
    A.push((152+68*m, 162+70*m+half_m, one));
    A.push((152+68*m, 163+70*m+half_m, one));
    A.push((152+68*m, 166+70*m+half_m, one));
    A.push((152+68*m, 167+70*m+half_m, one));
    B.push((152+68*m, num_vars, one));
    for k in 1..half_m+1 {
        C.push((152+68*m, 159+69*m+half_m+2*k+1, Alpha[k]));
    }
    C.push((152+68*m, 160+69*m+half_m, Alpha[85]));



    //provide the satisfying assignments
    let inst = Instance::new(num_cons, num_vars, num_inputs, &A, &B, &C).unwrap();

    let mut vars_para = vec![];
    let mut vars_input = vec![Scalar::zero().to_bytes(); 81];
    let mut vars = vec![];

    //PARA 0-15, HG, IHIG
    let path1 = "./data/dwt_fixpoint/dwt_fixpoint_HG.txt";
    let HG = load_data_i64(path1);
    for ele in HG {
        // println!("{:?}", ele);
        if ele < 0 {
            vars_para.push((-Scalar::from(-ele as u64)).to_bytes());
            vars.push((-Scalar::from(-ele as u64)).to_bytes());
        }
        else {
            vars_para.push(Scalar::from(ele as u64).to_bytes());
            vars.push(Scalar::from(ele as u64).to_bytes());
        }
    }

    //PARA 16 - 80, lambda
    let path2 = "./data/dwt_fixpoint/dwt_fixpoint_lambda_i.txt";
    let lambda = load_data_i64(path2);
    for ele in lambda {
        // println!("{:?}", ele);
        if ele < 0 {
            vars_para.push((-Scalar::from(-ele as u64)).to_bytes());
            vars.push((-Scalar::from(-ele as u64)).to_bytes());
        }
        else {
            vars_para.push(Scalar::from(ele as u64).to_bytes());
            vars.push((Scalar::from(ele as u64)).to_bytes());
        }
    }

    //pad zeros
    for i in 0..num_vars - 81 {
        vars_para.push(Scalar::zero().to_bytes());
    }



    //INPUT_VAR 81 - 81+m
    let path3 = "./data/dwt_fixpoint/dwt_fixpoint_x.txt";
    let x = load_data_i64(path3);
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

    //INPUT_VAR 82+m - 82+2m
    let path4 = "./data/dwt_fixpoint/dwt_fixpoint_y.txt";
    let y = load_data_i64(path4);
    for ele in y {
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

    //INPUT_VAR 83+2m - 90+2m
    let path5 = "./data/dwt_fixpoint/dwt_fixpoint_temp0to7.txt";
    let temp0to7 = load_data_i64(path5);
    for ele in temp0to7 {
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

    //INPUT_VAR 91+2m - 91+2m+half_m
    let path6 = "./data/dwt_fixpoint/dwt_fixpoint_abs_y.txt";
    let absy = load_data_i64(path6);
    for ele in absy {
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

    //INPUT_VAR 92+2m+half_m - 91+3m
    let path7 = "./data/dwt_fixpoint/dwt_fixpoint_sign_y.txt";
    let signy = load_data_i64(path7);
    for ele in signy {
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

    //INPUT_VAR 92+3m - 123+35m
    let path8 = "./data/dwt_fixpoint/dwt_fixpoint_cij_new.txt";
    let cij = load_data_cd(path8);
    for i in 0..64 {
        for j in 0..(half_m+1) {
            let ele = cij[j * 64 + i];
            // println!("{:?}", ele);
            vars_input.push(Scalar::from(ele as u64).to_bytes());
            vars.push(Scalar::from(ele as u64).to_bytes());
        }
    }

    //INPUT_VAR 124+35m - 156+67m+half_m
    let path9 = "./data/dwt_fixpoint/dwt_fixpoint_dij_new.txt";
    let dij = load_data_cd(path9);
    // let dij = compute_dij();
    // println!("{:?}", dij);
    for i in 0..65 {
        for j in 0..(half_m+1) {
            let ele = dij[j * 65 + i];
            vars_input.push(Scalar::from(ele as u64).to_bytes());
            vars.push(Scalar::from(ele as u64).to_bytes());
        }
    }

    //INPUT_VAR 157+67m+half_m - 156+68m
    let path10 = "./data/dwt_fixpoint/dwt_fixpoint_e1.txt";
    let e1 = load_data_i64(path10);
    for ele in e1 {
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

    //INPUT_VAR 157+68m - 157+68m+half_m
    let path11 = "./data/dwt_fixpoint/dwt_fixpoint_e2.txt";
    let e2 = load_data_i64(path11);
    for ele in e2 {
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

    //INPUT_VAR 158+68m+half_m - 157+69m
    let path12 = "./data/dwt_fixpoint/dwt_fixpoint_e3.txt";
    let e3 = load_data_i64(path12);
    for ele in e3 {
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

    //INPUT_VAR 158+69m - 158+69m+half_m
    let path13 = "./data/dwt_fixpoint/dwt_fixpoint_y_prime.txt";
    let y_prime = load_data_i64(path13);
    for ele in y_prime {
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

    //INPUT_VAR 159+69m+half_m - 159+70m+half_m
    let x_prime = compute_dwt_output::compute_x_prime();
    println!("{:?}", x_prime.len());
    for ele in x_prime {
        if ele < 0 {
            vars_input.push((-Scalar::from(-ele as u128)).to_bytes());
            vars.push((-Scalar::from(-ele as u128)).to_bytes());
        }
        else {
            vars_input.push(Scalar::from(ele as u128).to_bytes());
            vars.push(Scalar::from(ele as u128).to_bytes());
        }
    }


    //INPUT_VAR 160+70m+half_m - 167+60m+half_m
    // let path14 = "./data/dwt_fixpoint/dwt_fixpoint_temp8to15.txt";
    // let temp8to15 = load_data_i128(path14);
    let temp8to15 = comupute_temp8to15();
    for ele in temp8to15 {
        println!("{:?}", ele);
        if ele < 0 {
            vars_input.push((-Scalar::from(-ele as u128)).to_bytes());
            vars.push((-Scalar::from(-ele as u128)).to_bytes());
        }
        else {
            vars_input.push(Scalar::from(ele as u128).to_bytes());
            vars.push(Scalar::from(ele as u128).to_bytes());
        }
    }

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