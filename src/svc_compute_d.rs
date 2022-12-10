use crate::load_data::*;
use std::path::Prefix::Verbatim;

pub fn compute_d() -> Vec<i64> {
    let temp = load_data_2d_i64("./data/svc_fixpoint/svc_temp.txt");
    let lambda = 29127;
    let mut d = Vec::new();
    for i in 0..545 {
        let mut sum = 0;
        for j in 0..9 {
            sum = sum + temp[i][j];
        }
        d.push(sum * lambda)
    }
    println!("{:?}", d);

    d
}

pub fn compute_dij() -> Vec<Vec<i64>> {
    let mut d = compute_d();
    let mut dij = Vec::new();
    for i in 0..545 {
        let mut binary_vec = vec![0i64; 64];
        // println!("{:?}", binary_vec);
        let mut tag:bool = true;
        let mut j = 0;
        while tag {
            let bit = d[i] % 2;
            binary_vec[j] = bit;
            d[i] = (d[i] - bit) / 2;
            j = j + 1;
            if d[i] == 1 {
                binary_vec[j] = 1;
                tag = false;
            }
        }
        if i == 0 {
            println!("{:?}", binary_vec)
        }
        // binary_vec.push(1);

        dij.push(binary_vec);
    }
    dij
}