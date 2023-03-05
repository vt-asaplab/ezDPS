use std::env::var;
use std::iter;
use curve25519_dalek::ristretto::CompressedRistretto;
// use curve25519_dalek::scalar::Scalar;
use libspartan::{SNARKGens, sparse_mlpoly, dense_mlpoly, random, Instance, VarsAssignment, SNARK, InputsAssignment, ComputationDecommitment, ComputationCommitment};
use libspartan::dense_mlpoly::{DensePolynomial, EqPolynomial, PolyCommitment, PolyCommitmentBlinds, PolyCommitmentGens, PolyEvalProof};
use libspartan::r1csproof::{R1CSGens, R1CSProof};
use libspartan::random::RandomTape;
use merlin::Transcript;
use libspartan::scalar::Scalar;
use libspartan::transcript::{ProofTranscript, AppendToTranscript};
use libspartan::timer::Timer;
use libspartan::r1csinstance::{
    R1CSCommitment, R1CSCommitmentGens, R1CSDecommitment, R1CSEvalProof, R1CSInstance,
};
use libspartan::nizk::{KnowledgeProof, ProductProof, EqualityProof};
use libspartan::sparse_mlpoly::{SparsePolyEntry, SparsePolynomial};
use libspartan::errors::ProofVerifyError;
use libspartan::group::{GroupElement, VartimeMultiscalarMul};
use libspartan::commitments::Commitments;

#[cfg(test)]
#[test]
fn tests() -> () {

    let mut random_tape_1 = RandomTape::new_update(b"", Scalar::from(2u64));
    let mut random_tape_2 = RandomTape::new_update(b"", Scalar::from(4u64));

    //The constrains and AC
    let num_cons = 4;
    let num_vars = 16;
    let num_inputs = 0;
    let num_non_zero_entries = 4;


    let mut A: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut B: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut C: Vec<(usize, usize, [u8; 32])> = Vec::new();

    let one = Scalar::one().to_bytes();

    //Constraint 0: x_1 * w_1 = y_1
    A.push((0, 3, one));
    B.push((0, 0, one));
    C.push((0, 6, one));

    //Constraint 1: x_2 * w_2 = y_2
    A.push((1, 4, one));
    B.push((1, 1, one));
    C.push((1, 7, one));

    //Constraint 2: x_3 * w_3 = y_3
    A.push((2, 5, one));
    B.push((2, 2, one));
    C.push((2, 8, one));

    let gens = SNARKGens::new(num_cons, num_vars, num_inputs, num_non_zero_entries);
    let inst = Instance::new(num_cons, num_vars, num_inputs, &A, &B, &C).unwrap();

    let (comm, decomm) = SNARK::encode(&inst, &gens);

    //satisfying assignments, instances for var_para
    let w1 = Scalar::one();
    let w2 = Scalar::from(2u64);
    let w3 = Scalar::from(3u64);

    //create the var_para Assignments
    let mut vars_para = vec![Scalar::zero().to_bytes(); 9];
    vars_para[0] = w1.to_bytes();
    vars_para[1] = w2.to_bytes();
    vars_para[2] = w3.to_bytes();
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

    //commit to the var_para assignments
    let poly_vars_para = DensePolynomial::new(padded_vars_para.assignment.clone());
    let (comm_vars_para, blind_vars_para) = poly_vars_para.commit(&gens.gens_r1cs_sat.gens_pc, Some(&mut random_tape_1));

    //Now the verifier sends the inputs
    let x1 = Scalar::one();
    let x2 = Scalar::from(2u64);
    let x3 = Scalar::from(3u64);

    //Prover computes the ML and gets the auxiliary witnesses
    let y1 = Scalar::one();
    let y2 = Scalar::from(4u64);
    let y3 = Scalar::from(9u64);

    //create the var_inputs Assignments
    let mut vars_input = vec![Scalar::zero().to_bytes(); 9];
    vars_input[3] = x1.to_bytes();
    vars_input[4] = x2.to_bytes();
    vars_input[5] = x3.to_bytes();
    vars_input[6] = y1.to_bytes();
    vars_input[7] = y2.to_bytes();
    vars_input[8] = y3.to_bytes();
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

    //commit to the inputs
    let poly_vars_inputs = DensePolynomial::new(padded_vars_input.assignment.clone());
    let (comm_vars_input, blind_vars_input) = poly_vars_inputs.commit(&gens.gens_r1cs_sat.gens_pc, Some(&mut random_tape_1));

    // create a VarsAssignment
    let mut vars = vec![Scalar::zero().to_bytes(); num_vars];
    vars[0] = w1.to_bytes();
    vars[1] = w2.to_bytes();
    vars[2] = w3.to_bytes();
    vars[3] = x1.to_bytes();
    vars[4] = x2.to_bytes();
    vars[5] = x3.to_bytes();
    vars[6] = y1.to_bytes();
    vars[7] = y2.to_bytes();
    vars[8] = y3.to_bytes();
    let assignment_vars = VarsAssignment::new(&vars).unwrap();

    let padded_vars = {
        let num_padded_vars = inst.inst.get_num_vars();
        let num_vars = assignment_vars.assignment.len();
        let padded_vars = if num_padded_vars > num_vars {
            assignment_vars.pad(num_padded_vars)
        } else {
            assignment_vars
        };
        padded_vars
    };

    let poly_vars = DensePolynomial::new(padded_vars.assignment.clone());

    let mut inputs = vec![Scalar::zero().to_bytes(); num_inputs];
    let assignment_inputs = InputsAssignment::new(&inputs).unwrap();


    ///从这里开始应该重写polynomial commitment以提取blinds方便之后相加
    ///
    let blind_para = blind_vars_para.blinds;
    let blind_input = blind_vars_input.blinds;

    let (comm_vars, blind_vars) = my_dense_mlpoly_commit(&poly_vars, &gens.gens_r1cs_sat.gens_pc,
                                                         blind_para, blind_input);
    println!("{:?}", blind_vars.blinds);



    ///这个assert证明了经过padding的多项式可以相加，结果跟直接构建多项式是一致的
    ///
    let mut poly_prime = poly_vars.clone();
    for i in 0..poly_vars.len {
        poly_prime.Z[i] = poly_vars_para.Z[i] + poly_vars_inputs.Z[i]
    }
    // poly_prime.Z = poly_vars_para.Z + poly_vars_inputs.Z;
    assert_eq!(poly_prime.num_vars, poly_vars.num_vars);


    ///这个assertation证明了commitment的可组合性
    let a = comm_vars_para.C[0].decompress().unwrap();
    let b = comm_vars_input.C[0].decompress().unwrap();
    let c = a + b;
    let c_prime = comm_vars.C[0].decompress().unwrap();
    assert_eq!(c, c_prime);

    ///开始组合commitment
    let mut combine_comm_vars = vec![];
    for i in 0..comm_vars_para.C.len() {
        combine_comm_vars.push((comm_vars_para.C[i].decompress().unwrap() + comm_vars_input.C[i].decompress().unwrap()).compress());
    };

    let combine_commitment = PolyCommitment { C: combine_comm_vars };

    ///输入到证明里, 这里用到的证明函数都是重写过的，只输入合并过后的commitment
    //rewrite the proving procedure
    let mut prover_transcript = Transcript::new(b"combine_example");
    let proof = my_lib_prove(
        &inst,
        &decomm,
        padded_vars,
        &assignment_inputs,
        &gens,
        &mut prover_transcript,
        poly_vars,
        combine_commitment,
        blind_vars,
    );


    ///Verification
    // verify the proof of satisfiability
    let mut verifier_transcript = Transcript::new(b"combine_example");
    assert!(my_lib_verify(proof, &comm, &assignment_inputs, &mut verifier_transcript, &gens, comm_vars_para, comm_vars_input)
        .is_ok());

}

pub fn my_dense_mlpoly_commit(
    poly: &DensePolynomial,
    gens: &PolyCommitmentGens,
    blind_1: Vec<Scalar>,
    blind_2: Vec<Scalar>,
) -> (PolyCommitment, PolyCommitmentBlinds) {
    let n = poly.Z.len();
    let ell = poly.get_num_vars();
    assert_eq!(n, pow2(ell));

    let (left_num_vars, right_num_vars) = EqPolynomial::compute_factored_lens(ell);
    let L_size = pow2(left_num_vars);
    let R_size = pow2(right_num_vars);
    assert_eq!(L_size * R_size, n);

    assert_eq!(blind_1.len(), blind_2.len());
    let len_blind = blind_1.len();
    let mut blind_sum = vec![];
    for i in 0..blind_1.len() {
        blind_sum.push(blind_1[i] + blind_2[i]);
    }

    let blinds = {
        PolyCommitmentBlinds {
            //修改这句
            // blinds: random_tape.unwrap().random_vector(b"poly_blinds", L_size),
            blinds: blind_sum,
        }
    };

    (poly.commit_inner(&blinds.blinds, &gens.gens.gens_n), blinds)
}

fn pow2(ell: usize) -> usize {
    let base: usize = 2;
    base.pow(ell as u32)
}

pub fn my_lib_prove(
    inst: &Instance,
    decomm: &ComputationDecommitment,
    vars: VarsAssignment,
    inputs: &InputsAssignment,
    gens: &SNARKGens,
    transcript: &mut Transcript,
    poly_vars: DensePolynomial,
    comm_vars: PolyCommitment,
    blinds_vars: PolyCommitmentBlinds,
) -> SNARK {
    let timer_prove = Timer::new("SNARK::prove");

    // we create a Transcript object seeded with a random Scalar
    // to aid the prover produce its randomness
    let mut random_tape = RandomTape::new(b"proof");
    transcript.append_protocol_name(SNARK::protocol_name());
    let (r1cs_sat_proof, rx, ry) = {
        let (proof, rx, ry) = {
            // we might need to pad variables
            let padded_vars = vars;

            my_R1CSProof_prove(
                &inst.inst,
                padded_vars.assignment,
                &inputs.assignment,
                &gens.gens_r1cs_sat,
                transcript,
                &mut random_tape,
                poly_vars,
                comm_vars,
                blinds_vars
            )
        };

        let proof_encoded: Vec<u8> = bincode::serialize(&proof).unwrap();
        Timer::print(&format!("len_r1cs_sat_proof {:?}", proof_encoded.len()));

        (proof, rx, ry)
    };
    // We send evaluations of A, B, C at r = (rx, ry) as claims
    // to enable the verifier complete the first sum-check
    let timer_eval = Timer::new("eval_sparse_polys");
    let inst_evals = {
        let (Ar, Br, Cr) = inst.inst.evaluate(&rx, &ry);
        Ar.append_to_transcript(b"Ar_claim", transcript);
        Br.append_to_transcript(b"Br_claim", transcript);
        Cr.append_to_transcript(b"Cr_claim", transcript);
        (Ar, Br, Cr)
    };
    timer_eval.stop();

    let r1cs_eval_proof = {
        let proof = R1CSEvalProof::prove(
            &decomm.decomm,
            &rx,
            &ry,
            &inst_evals,
            &gens.gens_r1cs_eval,
            transcript,
            &mut random_tape,
        );

        let proof_encoded: Vec<u8> = bincode::serialize(&proof).unwrap();
        Timer::print(&format!("len_r1cs_eval_proof {:?}", proof_encoded.len()));
        proof
    };

    timer_prove.stop();
    SNARK {
        r1cs_sat_proof,
        inst_evals,
        r1cs_eval_proof,
    }
}


pub fn my_R1CSProof_prove(
    inst: &R1CSInstance,
    vars: Vec<Scalar>,
    input: &[Scalar],
    gens: &R1CSGens,
    transcript: &mut Transcript,
    random_tape: &mut RandomTape,
    poly_vars: DensePolynomial,
    comm_vars: PolyCommitment,
    blinds_vars: PolyCommitmentBlinds
) -> (R1CSProof, Vec<Scalar>, Vec<Scalar>) {
    let timer_prove = Timer::new("R1CSProof::prove");
    transcript.append_protocol_name(R1CSProof::protocol_name());

    // we currently require the number of |inputs| + 1 to be at most number of vars
    assert!(input.len() < vars.len());

    let timer_commit = Timer::new("polycommit");

    comm_vars.append_to_transcript(b"poly_commitment", transcript);

    timer_commit.stop();

    let timer_sc_proof_phase1 = Timer::new("prove_sc_phase_one");

    // append input to variables to create a single vector z
    let z = {
        let num_inputs = input.len();
        let num_vars = vars.len();
        let mut z = vars;
        z.extend(&vec![Scalar::one()]); // add constant term in z
        z.extend(input);
        z.extend(&vec![Scalar::zero(); num_vars - num_inputs - 1]); // we will pad with zeros
        z
    };

    // derive the verifier's challenge tau
    let (num_rounds_x, num_rounds_y) = (my_log2(inst.get_num_cons()), my_log2(z.len()));
    let tau = transcript.challenge_vector(b"challenge_tau", num_rounds_x);
    // compute the initial evaluation table for R(\tau, x)
    let mut poly_tau = DensePolynomial::new(EqPolynomial::new(tau).evals());
    let (mut poly_Az, mut poly_Bz, mut poly_Cz) =
        inst.multiply_vec(inst.get_num_cons(), z.len(), &z);

    let (sc_proof_phase1, rx, _claims_phase1, blind_claim_postsc1) = R1CSProof::prove_phase_one(
        num_rounds_x,
        &mut poly_tau,
        &mut poly_Az,
        &mut poly_Bz,
        &mut poly_Cz,
        &gens.gens_sc,
        transcript,
        random_tape,
    );
    assert_eq!(poly_tau.len(), 1);
    assert_eq!(poly_Az.len(), 1);
    assert_eq!(poly_Bz.len(), 1);
    assert_eq!(poly_Cz.len(), 1);
    timer_sc_proof_phase1.stop();

    let (tau_claim, Az_claim, Bz_claim, Cz_claim) =
        (&poly_tau[0], &poly_Az[0], &poly_Bz[0], &poly_Cz[0]);
    let (Az_blind, Bz_blind, Cz_blind, prod_Az_Bz_blind) = (
        random_tape.random_scalar(b"Az_blind"),
        random_tape.random_scalar(b"Bz_blind"),
        random_tape.random_scalar(b"Cz_blind"),
        random_tape.random_scalar(b"prod_Az_Bz_blind"),
    );

    let (pok_Cz_claim, comm_Cz_claim) = {
        KnowledgeProof::prove(
            &gens.gens_sc.gens_1,
            transcript,
            random_tape,
            &Cz_claim,
            &Cz_blind,
        )
    };

    let (proof_prod, comm_Az_claim, comm_Bz_claim, comm_prod_Az_Bz_claims) = {
        let prod = Az_claim * Bz_claim;
        ProductProof::prove(
            &gens.gens_sc.gens_1,
            transcript,
            random_tape,
            &Az_claim,
            &Az_blind,
            &Bz_claim,
            &Bz_blind,
            &prod,
            &prod_Az_Bz_blind,
        )
    };

    comm_Az_claim.append_to_transcript(b"comm_Az_claim", transcript);
    comm_Bz_claim.append_to_transcript(b"comm_Bz_claim", transcript);
    comm_Cz_claim.append_to_transcript(b"comm_Cz_claim", transcript);
    comm_prod_Az_Bz_claims.append_to_transcript(b"comm_prod_Az_Bz_claims", transcript);

    // prove the final step of sum-check #1
    let taus_bound_rx = tau_claim;
    let blind_expected_claim_postsc1 = taus_bound_rx * (prod_Az_Bz_blind - Cz_blind);
    let claim_post_phase1 = (Az_claim * Bz_claim - Cz_claim) * taus_bound_rx;
    let (proof_eq_sc_phase1, _C1, _C2) = EqualityProof::prove(
        &gens.gens_sc.gens_1,
        transcript,
        random_tape,
        &claim_post_phase1,
        &blind_expected_claim_postsc1,
        &claim_post_phase1,
        &blind_claim_postsc1,
    );

    let timer_sc_proof_phase2 = Timer::new("prove_sc_phase_two");
    // combine the three claims into a single claim
    let r_A = transcript.challenge_scalar(b"challenege_Az");
    let r_B = transcript.challenge_scalar(b"challenege_Bz");
    let r_C = transcript.challenge_scalar(b"challenege_Cz");
    let claim_phase2 = r_A * Az_claim + r_B * Bz_claim + r_C * Cz_claim;
    let blind_claim_phase2 = r_A * Az_blind + r_B * Bz_blind + r_C * Cz_blind;

    let evals_ABC = {
        // compute the initial evaluation table for R(\tau, x)
        let evals_rx = EqPolynomial::new(rx.clone()).evals();
        let (evals_A, evals_B, evals_C) =
            inst.compute_eval_table_sparse(inst.get_num_cons(), z.len(), &evals_rx);

        assert_eq!(evals_A.len(), evals_B.len());
        assert_eq!(evals_A.len(), evals_C.len());
        (0..evals_A.len())
            .map(|i| r_A * evals_A[i] + r_B * evals_B[i] + r_C * evals_C[i])
            .collect::<Vec<Scalar>>()
    };

    // another instance of the sum-check protocol
    let (sc_proof_phase2, ry, claims_phase2, blind_claim_postsc2) = R1CSProof::prove_phase_two(
        num_rounds_y,
        &claim_phase2,
        &blind_claim_phase2,
        &mut DensePolynomial::new(z),
        &mut DensePolynomial::new(evals_ABC),
        &gens.gens_sc,
        transcript,
        random_tape,
    );
    timer_sc_proof_phase2.stop();

    let timer_polyeval = Timer::new("polyeval");
    let eval_vars_at_ry = poly_vars.evaluate(&ry[1..].to_vec());
    let blind_eval = random_tape.random_scalar(b"blind_eval");
    let (proof_eval_vars_at_ry, comm_vars_at_ry) = PolyEvalProof::prove(
        &poly_vars,
        Some(&blinds_vars),
        &ry[1..].to_vec(),
        &eval_vars_at_ry,
        Some(&blind_eval),
        &gens.gens_pc,
        transcript,
        random_tape,
    );
    timer_polyeval.stop();

    // prove the final step of sum-check #2
    let blind_eval_Z_at_ry = (Scalar::one() - ry[0]) * blind_eval;
    let blind_expected_claim_postsc2 = claims_phase2[1] * blind_eval_Z_at_ry;
    let claim_post_phase2 = claims_phase2[0] * claims_phase2[1];
    let (proof_eq_sc_phase2, _C1, _C2) = EqualityProof::prove(
        &gens.gens_pc.gens.gens_1,
        transcript,
        random_tape,
        &claim_post_phase2,
        &blind_expected_claim_postsc2,
        &claim_post_phase2,
        &blind_claim_postsc2,
    );

    timer_prove.stop();

    (
        R1CSProof {
            comm_vars,
            sc_proof_phase1,
            claims_phase2: (
                comm_Az_claim,
                comm_Bz_claim,
                comm_Cz_claim,
                comm_prod_Az_Bz_claims,
            ),
            pok_claims_phase2: (pok_Cz_claim, proof_prod),
            proof_eq_sc_phase1,
            sc_proof_phase2,
            comm_vars_at_ry,
            proof_eval_vars_at_ry,
            proof_eq_sc_phase2,
        },
        rx,
        ry,
    )
}

fn my_log2(ell: usize) -> usize {
    (ell as f64).log2() as usize
}

fn my_r1csproof_verify(
    inp: R1CSProof,
    num_vars: usize,
    num_cons: usize,
    input: &[Scalar],
    evals: &(Scalar, Scalar, Scalar),
    transcript: &mut Transcript,
    gens: &R1CSGens,
    comm_1: PolyCommitment,
    comm_2: PolyCommitment,
) -> Result<(Vec<Scalar>, Vec<Scalar>), ProofVerifyError> {
    transcript.append_protocol_name(R1CSProof::protocol_name());

    let n = num_vars;
    // add the commitment to the verifier's transcript
    let mut combine_comm_vars = vec![];
    for i in 0..inp.comm_vars.C.len() {
        combine_comm_vars.push((comm_1.C[i].decompress().unwrap() + comm_2.C[i].decompress().unwrap()).compress());
    };

    let combine_commitment = PolyCommitment { C: combine_comm_vars };
    combine_commitment.append_to_transcript(b"poly_commitment", transcript);

    let (num_rounds_x, num_rounds_y) = (my_log2(num_cons), my_log2(2 * num_vars));

    // derive the verifier's challenge tau
    let tau = transcript.challenge_vector(b"challenge_tau", num_rounds_x);

    // verify the first sum-check instance
    let claim_phase1 = Scalar::zero()
        .commit(&Scalar::zero(), &gens.gens_sc.gens_1)
        .compress();
    let (comm_claim_post_phase1, rx) = inp.sc_proof_phase1.verify(
        &claim_phase1,
        num_rounds_x,
        3,
        &gens.gens_sc.gens_1,
        &gens.gens_sc.gens_4,
        transcript,
    )?;
    // perform the intermediate sum-check test with claimed Az, Bz, and Cz
    let (comm_Az_claim, comm_Bz_claim, comm_Cz_claim, comm_prod_Az_Bz_claims) = &inp.claims_phase2;
    let (pok_Cz_claim, proof_prod) = &inp.pok_claims_phase2;

    assert!(pok_Cz_claim
        .verify(&gens.gens_sc.gens_1, transcript, &comm_Cz_claim)
        .is_ok());
    assert!(proof_prod
        .verify(
            &gens.gens_sc.gens_1,
            transcript,
            &comm_Az_claim,
            &comm_Bz_claim,
            &comm_prod_Az_Bz_claims
        )
        .is_ok());

    comm_Az_claim.append_to_transcript(b"comm_Az_claim", transcript);
    comm_Bz_claim.append_to_transcript(b"comm_Bz_claim", transcript);
    comm_Cz_claim.append_to_transcript(b"comm_Cz_claim", transcript);
    comm_prod_Az_Bz_claims.append_to_transcript(b"comm_prod_Az_Bz_claims", transcript);

    let taus_bound_rx: Scalar = (0..rx.len())
        .map(|i| rx[i] * tau[i] + (Scalar::one() - rx[i]) * (Scalar::one() - tau[i]))
        .product();
    let expected_claim_post_phase1 = (taus_bound_rx
        * (comm_prod_Az_Bz_claims.decompress().unwrap() - comm_Cz_claim.decompress().unwrap()))
        .compress();

    // verify proof that expected_claim_post_phase1 == claim_post_phase1
    assert!(inp
        .proof_eq_sc_phase1
        .verify(
            &gens.gens_sc.gens_1,
            transcript,
            &expected_claim_post_phase1,
            &comm_claim_post_phase1,
        )
        .is_ok());

    // derive three public challenges and then derive a joint claim
    let r_A = transcript.challenge_scalar(b"challenege_Az");
    let r_B = transcript.challenge_scalar(b"challenege_Bz");
    let r_C = transcript.challenge_scalar(b"challenege_Cz");

    // r_A * comm_Az_claim + r_B * comm_Bz_claim + r_C * comm_Cz_claim;
    let comm_claim_phase2 = GroupElement::vartime_multiscalar_mul(
        iter::once(&r_A)
            .chain(iter::once(&r_B))
            .chain(iter::once(&r_C)),
        iter::once(&comm_Az_claim)
            .chain(iter::once(&comm_Bz_claim))
            .chain(iter::once(&comm_Cz_claim))
            .map(|pt| pt.decompress().unwrap())
            .collect::<Vec<GroupElement>>(),
    )
        .compress();

    // verify the joint claim with a sum-check protocol
    let (comm_claim_post_phase2, ry) = inp.sc_proof_phase2.verify(
        &comm_claim_phase2,
        num_rounds_y,
        2,
        &gens.gens_sc.gens_1,
        &gens.gens_sc.gens_3,
        transcript,
    )?;

    // verify Z(ry) proof against the initial commitment
    assert!(inp
        .proof_eval_vars_at_ry
        .verify(
            &gens.gens_pc,
            transcript,
            &ry[1..].to_vec(),
            &inp.comm_vars_at_ry,
            &inp.comm_vars
        )
        .is_ok());

    let poly_input_eval = {
        // constant term
        let mut input_as_sparse_poly_entries = vec![SparsePolyEntry::new(0, Scalar::one())];
        //remaining inputs
        input_as_sparse_poly_entries.extend(
            (0..input.len())
                .map(|i| SparsePolyEntry::new(i + 1, input[i]))
                .collect::<Vec<SparsePolyEntry>>(),
        );
        SparsePolynomial::new(my_log2(n), input_as_sparse_poly_entries).evaluate(&ry[1..].to_vec())
    };

    // compute commitment to eval_Z_at_ry = (Scalar::one() - ry[0]) * self.eval_vars_at_ry + ry[0] * poly_input_eval
    let comm_eval_Z_at_ry = GroupElement::vartime_multiscalar_mul(
        iter::once(Scalar::one() - ry[0]).chain(iter::once(ry[0])),
        iter::once(&inp.comm_vars_at_ry.decompress().unwrap()).chain(iter::once(
            &poly_input_eval.commit(&Scalar::zero(), &gens.gens_pc.gens.gens_1),
        )),
    );

    // perform the final check in the second sum-check protocol
    let (eval_A_r, eval_B_r, eval_C_r) = evals;
    let expected_claim_post_phase2 =
        ((r_A * eval_A_r + r_B * eval_B_r + r_C * eval_C_r) * comm_eval_Z_at_ry).compress();
    // verify proof that expected_claim_post_phase1 == claim_post_phase1
    assert!(inp
        .proof_eq_sc_phase2
        .verify(
            &gens.gens_sc.gens_1,
            transcript,
            &expected_claim_post_phase2,
            &comm_claim_post_phase2,
        )
        .is_ok());

    Ok((rx, ry))
}

pub fn my_lib_verify(
    pf: SNARK,
    comm: &ComputationCommitment,
    input: &InputsAssignment,
    transcript: &mut Transcript,
    gens: &SNARKGens,
    com_1: PolyCommitment,
    com_2: PolyCommitment,
) -> Result<(), ProofVerifyError> {
    let timer_verify = Timer::new("SNARK::verify");
    transcript.append_protocol_name(SNARK::protocol_name());

    let timer_sat_proof = Timer::new("verify_sat_proof");
    assert_eq!(input.assignment.len(), comm.comm.get_num_inputs());
    let (rx, ry) = my_r1csproof_verify (
        pf.r1cs_sat_proof,
        comm.comm.get_num_vars(),
        comm.comm.get_num_cons(),
        &input.assignment,
        &pf.inst_evals,
        transcript,
        &gens.gens_r1cs_sat,
        com_1,
        com_2,
    )?;
    timer_sat_proof.stop();

    let timer_eval_proof = Timer::new("verify_eval_proof");
    let (Ar, Br, Cr) = &pf.inst_evals;
    Ar.append_to_transcript(b"Ar_claim", transcript);
    Br.append_to_transcript(b"Br_claim", transcript);
    Cr.append_to_transcript(b"Cr_claim", transcript);
    assert!(pf
        .r1cs_eval_proof
        .verify(
            &comm.comm,
            &rx,
            &ry,
            &pf.inst_evals,
            &gens.gens_r1cs_eval,
            transcript
        )
        .is_ok());
    timer_eval_proof.stop();
    timer_verify.stop();
    Ok(())
}
