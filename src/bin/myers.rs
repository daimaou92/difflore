use std::fs::File;
use std::io::{prelude::*, BufReader, Error, ErrorKind, Result};

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

fn edit_g(s1: Vec<String>, s2: Vec<String>) -> Result<Vec<Vec<i8>>> {
    // println!("Preparing edit_graph");
    let m = s2.len();
    let n = s1.len();

    let mut mat = Vec::<Vec<i8>>::new();
    let mut row: Vec<i8>;
    let mut i = 0;
    while i <= m {
        let mut j = 0;
        row = Vec::<i8>::new();
        while j <= n {
            if i >= m || j >= n {
                row.push(0);
                j += 1;
                continue;
            }
            if let Some(t1) = s1.get(j) {
                if let Some(t2) = s2.get(i) {
                    if *t1 == *t2 {
                        row.push(-1)
                    } else {
                        row.push(0)
                    }
                } else {
                    return Err(Error::new(ErrorKind::Other, "new file index overflow"));
                }
            } else {
                return Err(Error::new(ErrorKind::Other, "old file index overflow"));
            }
            j += 1
        }
        mat.push(row);
        i += 1;
    }
    // println!("returning mat: {:?}", mat);
    Ok(mat)
}

enum PathOp {
    EQ,
    DEL,
    INS,
}

struct PathElem {
    i: u32,
    j: u32,
    op: PathOp,
}

impl PathElem {
    fn new(i: u32, j: u32, op: PathOp) -> Self {
        PathElem { i, j, op }
    }
}

fn ec_dist(i1: u32, j1: u32, i2: u32, j2: u32) -> u64 {
    return (((i2 - i1) * (i2 - i1) + (j2 - j1) * (j2 - j1)) as f64).sqrt() as u64;
}

fn edits(s1: Vec<String>, s2: Vec<String>) -> Result<Vec<PathElem>> {
    let eg = edit_g(s1.clone(), s2.clone())?;

    let m = s2.len() as u32;
    let n = s1.len() as u32;

    let mut path = Vec::<PathElem>::new();

    let mut q: Vec<(u32, u32)> = vec![(0, 0)];
    while !q.is_empty() {
        let (mut i, mut j) = q[0];
        q.remove(0);

        while eg[i as usize][j as usize] == -1 && j < n && i < m {
            path.push(PathElem::new(i, j, PathOp::EQ));
            i += 1;
            j += 1;
        }

        if j >= n && i >= m {
            // No movement possible
            break;
        }

        // Right
        let mut ir = i;
        let mut jr = j;
        if jr < n {
            jr += 1;
            while eg[ir as usize][jr as usize] == -1 && ir < m && jr < n {
                ir += 1;
                jr += 1;
            }
        }
        // Down
        let mut id = i;
        let mut jd = j;
        if id < m {
            id += 1;
            while eg[id as usize][jd as usize] == -1 && id < m && jd < n {
                id += 1;
                jd += 1;
            }
        }

        if ec_dist(ir, jr, m, n) <= ec_dist(id, jd, m, n) {
            path.push(PathElem::new(i, j, PathOp::DEL));
            j += 1;
        } else {
            path.push(PathElem::new(i, j, PathOp::INS));
            i += 1;
        }
        q.push((i, j))
    }

    Ok(path)
}

fn main() -> Result<()> {
    // println!("Diffing now...");
    let tup = get_lines("./file1".to_string(), "./file2".to_string())?;
    let c1 = tup.0;
    let c2 = tup.1;
    let path = edits(c1.clone(), c2.clone())?;

    for pe in path.iter() {
        match pe.op {
            PathOp::EQ => {
                println!("  {}", c1[pe.j as usize]);
            }
            PathOp::DEL => {
                println!("- {}", c1[pe.j as usize]);
            }
            PathOp::INS => {
                println!("+ {}", c2[pe.i as usize]);
            }
        };
    }

    Ok(())
}
