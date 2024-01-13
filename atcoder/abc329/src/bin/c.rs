fn main() {
    proconio::input! {
        n: usize,
        s: String,
    }
    println!("{}", count(s))
}

fn count(s: String) -> usize {
    let mut char_seq_nums = vec![0; 26];

    let mut chars = s.chars();

    let mut pred = chars.next().unwrap();
    let mut count = 1;

    for c in chars {
        if c == pred {
            count += 1;
        } else {
            let char_num = pred as usize - 'a' as usize;
            char_seq_nums[char_num] = std::cmp::max(char_seq_nums[char_num], count);

            count = 1;
            pred = c;
        }
    }

    let char_num = pred as usize - 'a' as usize;
    char_seq_nums[char_num] = std::cmp::max(char_seq_nums[char_num], count);

    char_seq_nums.into_iter().sum()
}
