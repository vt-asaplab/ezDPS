use std::fs::File;
use std::io::Read;

pub fn load_data_i64(path: &str) -> Vec<i64> {
    let mut f = File::open(path).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    let mut split = buf.split("\n");
    let mut V: Vec<&str> = split.collect();
    let mut result_vec = Vec::new();
    for str in V {
        if str == "" {
            continue;
        }
        else {
            let temp = str.parse::<i64>().unwrap();
            result_vec.push(temp);
        }
    }
    result_vec
}

pub fn load_data_cd(path: &str) -> Vec<i64> {
    let mut f = File::open(path).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    let mut split = buf.split(" ");
    let mut V: Vec<&str> = split.collect();
    let mut result_vec = Vec::new();
    for str in V {
        if str == "" {
            continue;
        }
        else {
            let temp = str.parse::<i64>().unwrap();
            result_vec.push(temp);
        }
    }
    result_vec
}

pub fn load_data_i128(path: &str) -> Vec<i128> {
    let mut f = File::open(path).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    let mut split = buf.split("\n");
    let mut V: Vec<&str> = split.collect();
    let mut result_vec = Vec::new();
    for str in V {
        if str == "" {
            continue;
        }
        else {
            let temp = str.parse::<i128>().unwrap();
            result_vec.push(temp);
        }
    }
    result_vec
}

pub fn load_data_2d_i64(path: &str) -> Vec<Vec<i64>> {

    // 这个result_vec应该是二维容器， 我不知道这吊毛语言怎么声明类型 Vec<Vec<i64>>吗，还是Vec<&Vec<i64>>
    let mut result_vec = Vec::new();

    let mut f = std::fs::File::open(path).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    // split第一次，V里面的每一个元素是一行
    let mut split = buf.split("\n");
    let mut V: Vec<&str> = split.collect();
    // println!("{:?}", V[0]);

    for str_row in V {
        if str_row == "" {
            continue;
        }
        // 一维的i64容器，Vec<i64>？
        let mut result_vec_row = Vec::new();
        // println!("{:?}", str_row);
        // split第二次，V_col里面是的每行中的元素
        let mut split_row = str_row.split(" ");
        let mut V_row: Vec<&str> = split_row.collect();
        for str in V_row {
            if str == "" {
                continue;
            }
            else {
                // println!("{:?}", str);
                let temp = str.parse::<i64>().unwrap();
                result_vec_row.push(temp);
            }
        }
        result_vec.push(result_vec_row);
    }

    result_vec
}

pub fn load_data_2d_i128(path: &str) -> Vec<Vec<i128>> {

    // 这个result_vec应该是二维容器， 我不知道这吊毛语言怎么声明类型 Vec<Vec<i64>>吗，还是Vec<&Vec<i64>>
    let mut result_vec = Vec::new();

    let mut f = std::fs::File::open(path).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    // split第一次，V里面的每一个元素是一行
    let mut split = buf.split("\n");
    let mut V: Vec<&str> = split.collect();
    // println!("{:?}", V[0]);

    for str_row in V {
        if str_row == "" {
            continue;
        }
        // 一维的i128容器，Vec<i128>
        let mut result_vec_row = Vec::new();
        // println!("{:?}", str_row);
        // split第二次，V_col里面是的每行中的元素
        let mut split_row = str_row.split(" ");
        let mut V_row: Vec<&str> = split_row.collect();
        for str in V_row {
            if str == "" {
                continue;
            }
            else {
                // println!("{:?}", str);
                let temp = str.parse::<i128>().unwrap();
                result_vec_row.push(temp);
            }
        }
        result_vec.push(result_vec_row);
    }

    result_vec
}