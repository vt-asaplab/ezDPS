use curve25519_dalek::digest::consts::{False, True};
use crate::load_data::*;

pub fn compute_miu_temp() -> (
    i64,
    Vec<i128>,
    Vec<i128>,
    i128,
    i64,
    i128,
    i64
) {
    let x = load_data_i64("./src/data/nl_fixpoint/nl_fixpoint_x.txt");
    let sigma:i64 = 940871;
    let inv_sigma:i64 = 1168609;
    let inv_m:i64 = 1398;  //1/m
    let one = 1048576;
    let mut sum = 0;
    for ele in x.clone() {
        sum = sum + ele;
    }
    let miu = sum * inv_m;
    let mut vec_temp = vec![];
    let mut vec_y = vec![];
    let mut sum2 = 0;
    for ele in x {
        let temp = ((ele * one ) as i128 - miu as i128) * ((ele * one) as i128 - miu as i128);
        vec_temp.push(temp);
        vec_y.push(((ele * one) as i128 - miu as i128) * inv_sigma as i128);
        sum2 = sum2 + temp
    }

    let epsilon1 = sum2 * inv_m as i128 - sigma as i128 * (one as i128)^5;
    let epsilon2 = one^2 - sigma * inv_sigma;

    let r1_ep1_2n:i128 = 104 * (one as i128)^4 - epsilon1 + 2^86;
    let r2_ep2_2N:i64 = 10 * one - epsilon2 + 2^24;

    println!("{:?}", epsilon1);
    println!("{:?}", epsilon2);


    (
        miu,
        vec_temp,
        vec_y,
        epsilon1,
        epsilon2,
        r1_ep1_2n,
        r2_ep2_2N
        )
}

#[cfg(test)]
mod tests {
    use crate::compute_nl_outputs::compute_miu_temp;

    #[test]
    fn test_output() {
        let (miu, temp, y, epsilon1, epsilon2, r1_ep1_2n, r2_ep2_2N) = compute_miu_temp();
    }
}


