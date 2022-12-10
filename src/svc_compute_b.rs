use crate::load_data::*;
pub fn compute_out() -> i128 {
    let alpha = load_data_i128("./data/svc_fixpoint/alpha_fixpoint.txt");
    let b = -211063;
    let mut sum  = 0;
    for i in 0..545 {
        sum = sum + alpha[i];
    }
    let result = sum + b;
    result
}