use std::fs::File;
use std::io::{prelude::*, BufReader, Error, ErrorKind, Result};
use std::mem;

fn get_lines(file1: String, file2: String) -> Result<(Vec<String>, Vec<String>)> {
    let file = File::open(file1)?;
    let lines = BufReader::new(file).lines();
    let mut c1 = Vec::<String>::new();
    for line in lines.flatten() {
        c1.push(line);
    }

    let file = File::open(file2)?;
    let lines = BufReader::new(file).lines();
    let mut c2 = Vec::<String>::new();
    for line in lines.flatten() {
        c2.push(line);
    }

    Ok((c1, c2))
}

fn lcs_dp(c1: Vec<String>, c2: Vec<String>) -> Result<Vec<Vec<u32>>> {
    let m: usize = c1.len();
    let n: usize = c2.len();
    let mut dp = Vec::<Vec<u32>>::new();

    let mut i = 0;
    while i <= m {
        dp.push(vec![0; n + 1]);
        i += 1;
    }

    let mut i = 1;
    while i <= m {
        let mut j = 1;
        while j <= n {
            if c1.get(i - 1).unwrap() == c2.get(j - 1).unwrap() {
                let v = dp.get(i - 1).unwrap().get(j - 1).unwrap();
                dp[i][j] = v + 1;
                // let mut arr = dp.get(i).unwrap().to_vec();
                // mem::replace(arr[j], (1+v));
                // arr.insert(j, 1 + v);
                // mem::replace(dp[i], arr.to_vec());
                // dp.insert(i, arr.to_vec());
            } else {
                let mut max: u32;
                if dp[i][j - 1] > dp[i - 1][j] {
                    max = *dp.get(i).unwrap().get(j - 1).unwrap();
                } else {
                    max = *dp.get(i - 1).unwrap().get(j).unwrap();
                }
                // let mut arr = dp.get(i).unwrap().to_vec();
                // arr.insert(j, max);
                // dp.insert(i, arr.to_vec));
                dp[i][j] = max;
            }
            j += 1;
        }
        i += 1;
    }

    Ok(dp)
}

fn common_lines(
    lines1: Vec<String>,
    lines2: Vec<String>,
    dp: Vec<Vec<u32>>,
) -> Result<(Vec<u32>, Vec<u32>)> {
    let mut i = (dp.len() - 1) as u32;
    if i == 0 {
        return Err(Error::new(ErrorKind::Other, "empty file1"));
    }

    let mut j = (dp.get(0).unwrap().to_vec().len() - 1) as u32;

    if j == 0 {
        return Err(Error::new(ErrorKind::Other, "empty file2"));
    }

    let mut indices1 = Vec::<u32>::new(); // includes lines1
    let mut indices2 = Vec::<u32>::new(); // includes lines2

    while i > 0 && j > 0 {
        println!("i: {}, j: {}", i, j);
        let s1 = lines1.get((i - 1) as usize).unwrap().to_string();
        let s2 = lines2.get((j - 1) as usize).unwrap().to_string();
        if s1 == s2 {
            indices1.insert(0, i - 1);
            indices2.insert(0, j - 1);
            i -= 1;
            j -= 1;
        } else {
            let vi = dp.get(i as usize).unwrap().get((j - 1) as usize).unwrap();
            let vj = dp.get((i - 1) as usize).unwrap().get(j as usize).unwrap();
            if vi > vj {
                j -= 1;
            } else {
                i -= 1;
            }
        }
    }

    Ok((indices1, indices2))
}

fn main() -> Result<()> {
    let tup = get_lines("./file1".to_string(), "./file2".to_string())?;
    let c1 = tup.0;
    let c2 = tup.1;
    let dp = lcs_dp(c1.clone(), c2.clone())?;
    println!("dp: {:?}", dp);
    // println!("LCS: {}", dp[m][n]);
    let cls = common_lines(c1, c2, dp);
    println!("Common lines: {:?}", cls);
    Ok(())
}
