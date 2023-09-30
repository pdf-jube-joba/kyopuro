fn main() {
    let (mut a, mi) = input();
    let n = a.len();
    a.sort();
    let sum: usize = a.iter().sum();
    for m in mi {
        let b = {
            if sum < m {
               false 
            } else {
                (0..2_usize.pow(n as u32)).any(|i|{
                    let mut sum = 0;
                    for j in 0..n {
                        if (1 << j & i) != 0 {
                            sum += a[j];
                        }
                        if m == sum {
                            return true;
                        }
                        if m < sum {
                            return false;
                        }
                    }
                    false
                })
            }
        };
        if b {
            println!("yes");
        } else {
            println!("no");
        }
    }
}

fn input() -> (Vec<usize>, Vec<usize>) {
    let _n = readline().trim().parse::<usize>().unwrap();
    let a = readline()
        .split_whitespace()
        .map(|str| str.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let _m = readline().trim().parse::<usize>().unwrap();
    let mi = readline()
        .split_whitespace()
        .map(|str| str.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    (a, mi)
}

fn readline() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}