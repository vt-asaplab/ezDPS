use crate::load_data::*;
pub fn compute_x_prime() -> Vec<i128> {
    let data = load_data_i128("./data/dwt_fixpoint/test.txt");
    let IhIg = load_data_i128("./data/dwt_fixpoint/dwt_IhIg_fixpoint.txt");
    let n = 170;
    let half = 85;
    let halfplus = 86;
    let mut result = Vec::new();
    let temp0 = data[half-1] * IhIg[0] + data[n-1] * IhIg[1] + data[0] * IhIg[2] + data[half] * IhIg[3];
    let temp1 = data[half - 1] * IhIg[4] + data[n - 1] * IhIg[5] + data[0] * IhIg[6] + data[half] * IhIg[7];
    result.push(temp0);
    result.push(temp1);
    for i in 0..(half-1) {
        let temp = data[i] * IhIg[0] + data[i+half] * IhIg[1] + data[i+1] * IhIg[2] + data[i+halfplus] * IhIg[3];
        result.push(temp);
        let temp1 = data[i] * IhIg[4] + data[i + half] * IhIg[5] + data[i + 1] * IhIg[6] + data[i + halfplus] * IhIg[7];
        result.push(temp1);
    }
    result
}