mod plusti {
    use prusti_contracts::*;

    #[requires(n <= 1000 && l <= 1000)]
    fn sums(n: usize, l: usize) -> usize {
        (n..n + l).sum()
    }

    #[requires(n <= 1000 && l <= 1000)]
    #[ensures(result == sums(n, l))]
    fn sums2(n: usize, l: usize) -> usize {
        (n + l) * l / 2 - (l / 2)
    }
}

mod kani {
    fn estimate_size(x: u32) -> u32 {
        if x < 256 {
            if x < 128 {
                1
            } else {
                3
            }
        } else if x < 1024 {
            if x > 1022 {
                panic!("Oh no, a failing corner case!");
            } else {
                return 5;
            }
        } else if x < 2048 {
            return 7;
        } else {
            return 9;
        }
    }

    #[cfg(kani)]
    #[kani::proof]
    fn check_estimate_size() {
        let x: u32 = kani::any();
        estimate_size(x);
    }
}
