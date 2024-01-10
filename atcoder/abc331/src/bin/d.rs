use proconio::marker::Chars;

struct Problem {
    p: Vec<Vec<bool>>,
    qs: Vec<(usize, usize, usize, usize)>,
}

impl Problem {
    fn from(p: Vec<Vec<char>>, qs: Vec<(usize, usize, usize, usize)>) -> Self {
        let p: Vec<Vec<bool>> = p
            .into_iter()
            .map(|chars| {
                chars
                    .into_iter()
                    .map(|char| match char {
                        'B' => true,
                        'W' => false,
                        _ => unreachable!("{char}?"),
                    })
                    .collect()
            })
            .collect();
        Self { p, qs }
    }
    fn from_stdin() -> Self {
        proconio::input! {
            n: usize, q: usize,
            p: [Chars; n],
            qs: [(usize, usize, usize, usize); q],
        }
        Problem::from(p, qs)
    }
    fn from_str(str: &str) -> Self {
        let source = proconio::source::once::OnceSource::from(str);
        proconio::input! {
            from source,
            n: usize, q: usize,
            p: [Chars; n],
            qs: [(usize, usize, usize, usize); q],
        }
        Problem::from(p, qs)
    }
    fn preprocess(self) -> Preprocessed {
        let n = self.p.len();
        // sum[i in 0..=n][j in 0..=n] = num of true in { a[i'][j'] | i' in 0..i, j' in 0..j }

        let p_sum: Vec<Vec<usize>> = {
            let mut sum = vec![vec![0; n + 1]; n + 1];
            for i in 0..n {
                for j in 0..n {
                    sum[i + 1][j + 1] =
                        (if self.p[i][j] { 1 } else { 0 }) + sum[i][j + 1] + sum[i + 1][j]
                            - sum[i][j];
                }
            }
            sum
        };
        Preprocessed { p_sum, qs: self.qs }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Preprocessed {
    p_sum: Vec<Vec<usize>>,
    qs: Vec<(usize, usize, usize, usize)>,
}

impl Preprocessed {
    // g((i,j): (usize, usize)) = num of true in { a[i'][j'] | i' in 0..i, j' in 0..j }
    fn g(&self, (c, d): (usize, usize)) -> usize {
        let n = self.p_sum.len() - 1;
        if c <= n && d <= n {
            return self.p_sum[c][d];
        }
        let (hq, hr) = (c / n, c % n);
        let (wq, wr) = (d / n, d % n);
        self.g((n, n)) * hq * wq + self.g((hr, n)) * wq + self.g((n, wr)) * hq + self.g((hr, wr))
    }
    fn f(&self, (a, b): (usize, usize), (c, d): (usize, usize)) -> usize {
        self.g((c, d)) + self.g((a, b)) - self.g((c, b)) - self.g((a, d))
    }
    fn ans(&self) -> Vec<usize> {
        self.qs
            .iter()
            .map(|&(a, b, c, d)| self.f((a, b), (c + 1, d + 1)))
            .collect()
    }
}

fn main() {
    let problem = Problem::from_stdin();
    let preprocessed = problem.preprocess();
    for ans in preprocessed.ans() {
        println!("{}", ans);
    }
}
