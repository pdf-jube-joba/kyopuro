const MODINT: u128 = (1 << 61) - 1;


// - a_{i,j}    ,...,a_{i,j+c-1}
//      :               :
//   a_{i+r-1,j},...,a_{i+r-1,j+c-1}
// の hash は \sum_{0 <= l <= r-1} \sum_{0 <= k <= c-1} a_{i+l,j+k} * b^{r*c - c(l-1) - (k-1)}
fn find_substr_2_61_1(hw: Vec<Vec<u8>>, rc: Vec<Vec<u8>>, bases: &[u128]) -> Vec<usize> {
    let h = hw.len();
    let w = hw[0].len(); // ok by constrait
    let r = rc.len();
    let c = rc[0].len(); // ok by constrait

    if h < r || w < c {
        return vec![];
    }

    let target_hash = bases.iter().map(|base|{
        let mut sum = 0;
        for i in 0..r {
            for j in 0..c {
                sum *= base;
                sum %= MODINT;
                sum += rc[i][j] as u128;
                sum %= MODINT;
            }
        }
        sum
    }).collect::<Vec<_>>();

    let mut hashes_of_0_0 = bases.iter().map(|base| {
        let mut sum = 0;
        for i in 0..r {
            for j in 0..c {
                sum *= base;
                sum %= MODINT;
                sum += rc[i][j] as u128;
                sum %= MODINT;
            }
        }
        sum
    }).collect::<Vec<_>>();

    let mut v = vec![];

    

    v

}

fn main() {
    let (h, w) = input();

}

fn input() -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut buf = String::new();
    let stdin = std::io::stdin();

    stdin.read_line(&mut buf).unwrap();

    let hw = buf.lines().map(|line| line.trim().as_bytes().to_vec()).collect();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();

    let rc = buf.lines().map(|line| line.trim().as_bytes().to_vec()).collect();

    (hw, rc)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tests]
    fn find_subpattern_test() {

    }
}