const TOU_NUM: usize = 4;
const KAI_NUM: usize = 3;
const HEYA_NUM: usize = 10;
const KUGIRI: &str = "####################";

fn main() {
    let mut kosya: Vec<Vec<Vec<isize>>> = vec![vec![vec![0; HEYA_NUM]; KAI_NUM]; TOU_NUM];
    let inshitao = input();
    for (tou, kai, heya, ninzu) in inshitao {
        kosya[tou][kai][heya] += ninzu;
    }

    for tou in 0..TOU_NUM {
        for kai in 0..KAI_NUM {
            for heya in 0..HEYA_NUM {
                print!(" {}", kosya[tou][kai][heya]);
            }
            println!();
        }
        if tou != TOU_NUM- 1 {
            println!("{}", KUGIRI);
        }
    }
}

fn input() -> Vec<(usize, usize, usize, isize)> {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();

    let n = string.trim().parse::<usize>().unwrap();

    let mut vec: Vec<_> = Vec::new();

    for _ in 0..n {
        string = String::new();
        std::io::stdin().read_line(&mut string).unwrap();
        let vec_str: Vec<&str> = string.split_whitespace().collect();
        let b: usize = vec_str[0].parse().unwrap();
        let f: usize = vec_str[1].parse().unwrap();
        let r: usize = vec_str[2].parse().unwrap();
        let v: isize = vec_str[3].parse().unwrap();
        vec.push((b - 1, f - 1, r - 1, v))
    }

    vec

}