fn main() {
    proconio::input! {
        k: usize, g: usize, m: usize,
    }
    let res = much_after_op(k, g, m);
    println!("{} {}", res.0, res.1)
}

fn much_after_op(k: usize, g: usize, m: usize) -> (usize, usize) {
    let mut g_much = 0;
    let mut m_much = 0;
    for _ in 0..k {
        if g_much == g {
            g_much = 0;
        } else if m_much == 0 {
            m_much = m;
        } else {
            let remain_g = g - g_much;
            if remain_g <= m_much {
                g_much = g;
                m_much -= remain_g;
            } else {
                g_much += m_much;
                m_much = 0;
            }
        }
    }
    (g_much, m_much)
}
