use curve25519_dalek::digest::consts::{False, True};
use crate::load_data::*;
pub fn compute_x_prime() -> Vec<i128> {
    let data = load_data_i128("./data/dwt_fixpoint/test.txt");
    let hg = load_data_i128("./data/dwt_fixpoint/dwt_fixpoint_HG.txt");
    let n = 750;
    let half = 375;
    let halfplus = 376;
    let mut result = Vec::new();
    let Ih:[i128;4] = [hg[8], hg[9], hg[10], hg[11]];
    let Ig:[i128;4] = [hg[12], hg[13], hg[14], hg[15]];
    // println!("{:?}", Ih);
    let temp0 = data[half-1] * Ih[0] + data[n-1] * Ih[1] + data[0] * Ih[2] + data[half] * Ih[3];
    let temp1 = data[half - 1] * Ig[0] + data[n - 1] * Ig[1] + data[0] * Ig[2] + data[half] * Ig[3];
    result.push(temp0);
    result.push(temp1);
    for i in 0..(half-1) {
        let temp = data[i] * Ih[0] + data[i+half] * Ih[1] + data[i+1] * Ih[2] + data[i+halfplus] * Ih[3];
        result.push(temp);
        let temp1 = data[i] * Ig[0] + data[i + half] * Ig[1] + data[i + 1] * Ig[2] + data[i + halfplus] * Ig[3];
        result.push(temp1);
    }
    result
}

//实际上没用
pub fn compute_dij() -> Vec<i64> {
    let abs_y = load_data_i64("./data/dwt_fixpoint/dwt_fixpoint_y_prime.txt");
    let lambda:i64 = 21989687296;
    let p:i128 = 18446744073709551616;
    let mut result = vec![];


    for ele in abs_y {
        let mut temp = ele as i128 - lambda as i128 + p;
        let mut binary_vec_temp:Vec<i64> = vec![];
        let mut tag  = 1;
        while tag == 1 {
            let bit = temp % 2;
            binary_vec_temp.push(bit as i64);
            temp = (temp - bit) / 2;
            if temp == 1 || temp == 0 {
                tag = 0;
            }
        }
        binary_vec_temp.push(1);
        for i in 0..binary_vec_temp.len() {
            result.push(binary_vec_temp[i]);
        }
    }
    result
}

pub fn comupute_temp8to15() -> Vec<i128> {
    let y = load_data_i128("./data/dwt_fixpoint/test.txt");
    let hg = load_data_i128("./data/dwt_fixpoint/dwt_fixpoint_HG.txt");
    let halfplus = 375;
    let mut result = Vec::new();
    let Ih:[i128;4] = [hg[8], hg[9], hg[10], hg[11]];
    let Ig:[i128;4] = [hg[12], hg[13], hg[14], hg[15]];
    let mut alp = 1;
    let mut alpha = vec![];
    alpha.push(alp);
    for i in 0..halfplus {
        alp = alp * 1;
        alpha.push(alp);
    }
    let mut sum1 = 0;
    let mut sum2 = 0;
    for i in 0..halfplus {
        sum1 = sum1 + y[i] * alpha[i];
        sum2 = sum2 + y[i + halfplus] * alpha[i];
    }
    result.push((alpha[1] * Ih[0] + Ih[2]) * sum1);
    result.push((alpha[1] * Ih[1] + Ih[3]) * sum2);
    result.push((alpha[1] * Ig[0] + Ig[2]) * sum1);
    result.push((alpha[1] * Ig[1] + Ig[3]) * sum2);
    result.push((alpha[halfplus] - 1) * Ih[2] * y[0]);
    result.push((alpha[halfplus] - 1) * Ih[3] * y[halfplus]);
    result.push((alpha[halfplus] - 1) * Ig[2] * y[0]);
    result.push((alpha[halfplus] - 1) * Ig[3] * y[halfplus]);

    result
}

#[cfg(test)]
mod tests {
    use crate::compute_dwt_output::compute_x_prime;

    #[test]
    fn test_output() {
        let result = compute_x_prime();
        println!("{:?}", result)
    }
}