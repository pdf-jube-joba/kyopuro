#[derive(Debug, Clone, PartialEq)]
enum BinTreeNode {
    DummyProb(f64),
    Node(Box<BinTree>),
}

impl BinTreeNode {
    fn dummy(dummy_prob: f64) -> BinTreeNode {
        BinTreeNode::DummyProb(dummy_prob)
    }
    fn node(key_prob: f64, left: BinTreeNode, right: BinTreeNode) -> BinTreeNode {
        BinTreeNode::Node(Box::new(BinTree::new(key_prob, left, right)))
    }
}

#[derive(Debug, Clone, PartialEq)]
struct BinTree {
    key_prob: f64,
    left: BinTreeNode,
    right: BinTreeNode,
}

impl BinTree {
    fn new(key_prob: f64, left: BinTreeNode, right: BinTreeNode) -> BinTree {
        BinTree {
            key_prob,
            left,
            right,
        }
    }
}

fn compute_expected_cost_node(tree: &BinTreeNode, depth: usize) -> f64 {
    match tree {
        BinTreeNode::DummyProb(prob) => (depth + 1) as f64 * prob,
        BinTreeNode::Node(tree) => compute_expected_cost(tree.as_ref(), depth),
    }
}

fn compute_expected_cost(tree: &BinTree, depth: usize) -> f64 {
    let BinTree {
        key_prob,
        left,
        right,
    } = tree;
    (depth + 1) as f64 * key_prob
        + compute_expected_cost_node(left, depth + 1)
        + compute_expected_cost_node(right, depth + 1)
}

fn construct_optimal_tree_rec(key_prob: &[f64], dummy_prob: &[f64]) -> BinTreeNode {
    let key_len = key_prob.len();
    debug_assert!(key_len + 1 == dummy_prob.len());
    if key_len == 0 {
        return BinTreeNode::DummyProb(dummy_prob[0]);
    }
    let tree = (0..key_len)
        .map(|i| {
            let left_tree = construct_optimal_tree_rec(&key_prob[0..i], &dummy_prob[0..=i]);
            let right_tree =
                construct_optimal_tree_rec(&key_prob[i + 1..key_len], &dummy_prob[i + 1..=key_len]);
            BinTree {
                key_prob: key_prob[i],
                left: left_tree,
                right: right_tree,
            }
        })
        .min_by(|tree1, tree2| {
            let exp1 = compute_expected_cost(tree1, 0);
            let exp2 = compute_expected_cost(tree2, 0);
            debug_assert!(!exp1.is_nan(), !exp2.is_nan());
            if exp1 < exp2 {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        })
        .unwrap();
    BinTreeNode::Node(Box::new(tree))
}

fn construct_optimal_tree(key_prob: &[f64], dummy_prob: &[f64]) -> BinTreeNode {
    let key_len = key_prob.len();
    debug_assert!(key_len + 1 == dummy_prob.len());

    // dp[i][j] = opt_tree of (key_prob[i..i+j], dummy_prob[i..=i+j])
    // 0 <= i < key_len, 0 <= j <= key_len - i
    let mut dp: Vec<Vec<BinTreeNode>> = vec![vec![]; key_len + 1];

    // j = 0 case
    // dp[i][0] = one dummy_key
    for i in 0..=key_len {
        dp[i].push(BinTreeNode::dummy(dummy_prob[i]));
    }

    // recursion
    // dp[i][j] = opt_tree of (kp[i..i+j], dp[i..=i+j])
    // = min_{0 <= k <= j-1} (kp[i..i+k], dp[i..=i+k]) + (kp[i+k+1..i+j], dp[i+k+1..=i+j])
    // = min_{0 <= k <= j-1} dp[i][k] + dp[i+k+1][j-(k+1)]
    for j in 1..=key_len {
        for i in 0..=key_len - j {
            let opt_tree = (0..j)
                .map(|k| {
                    let key_prob = key_prob[i+k];
                    let left = dp[i][k].clone();
                    let right = dp[i + k + 1][j - (k + 1)].clone();
                    BinTreeNode::node(key_prob, left, right)
                })
                .min_by(|tree1, tree2| {
                    let exp1 = compute_expected_cost_node(tree1, 0);
                    let exp2 = compute_expected_cost_node(tree2, 0);
                    debug_assert!(!exp1.is_nan(), !exp2.is_nan());
                    if exp1 < exp2 {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                })
                .unwrap();
            dp[i].push(opt_tree);
        }
    }

    dp[0][key_len].clone()
}

fn main() {
    let (key_prob, dummy_prob) = input();
    let opt_tree = construct_optimal_tree(&key_prob, &dummy_prob);
    let exp_cost = compute_expected_cost_node(&opt_tree, 0);
    println!("{}", exp_cost);
}

fn input() -> (Vec<f64>, Vec<f64>) {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let _n = {
        stdin.read_line(&mut buf).unwrap();
        buf.trim().parse::<usize>().unwrap()
    };

    let p = {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        buf.split_whitespace()
            .map(|str| str.parse::<f64>().unwrap())
            .collect()
    };

    let q = {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        buf.split_whitespace()
            .map(|str| str.parse::<f64>().unwrap())
            .collect()
    };

    (p, q)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn expected_value_test() {
        let tree = BinTreeNode::node(0.5, BinTreeNode::dummy(0.25), BinTreeNode::dummy(0.25));

        assert!({
            let res = compute_expected_cost_node(&tree, 0);
            let exp = 1.5;
            (res - exp).abs() < 0.01
        });

        let tree = BinTreeNode::node(
            0.2,
            BinTreeNode::node(
                0.2,
                BinTreeNode::DummyProb(0.1),
                BinTreeNode::DummyProb(0.1),
            ),
            BinTreeNode::node(
                0.2,
                BinTreeNode::DummyProb(0.1),
                BinTreeNode::DummyProb(0.1),
            ),
        );

        assert!({
            let res = compute_expected_cost_node(&tree, 0);
            let exp = 2.2;
            (res - exp).abs() < 0.01
        });
    }
    #[test]
    fn rec_case_test() {
        let opt_tree = construct_optimal_tree_rec(&vec![], &vec![1_f64]);
        let exp_tree = BinTreeNode::dummy(1_f64);

        assert_eq!(opt_tree, exp_tree);

        let opt_tree = construct_optimal_tree_rec(&vec![0.5_f64], &vec![0.25_f64, 0.25_f64]);
        let exp_tree = BinTreeNode::node(
            0.5_f64,
            BinTreeNode::dummy(0.25_f64),
            BinTreeNode::dummy(0.25_f64),
        );

        assert_eq!(opt_tree, exp_tree);
    }
    #[test]
    fn dp_case_test() {
        let opt_tree = construct_optimal_tree(&vec![], &vec![1_f64]);
        let exp_tree = BinTreeNode::dummy(1_f64);

        assert_eq!(opt_tree, exp_tree);

        let opt_tree = construct_optimal_tree(&vec![0.5_f64], &vec![0.25_f64, 0.25_f64]);
        let exp_tree = BinTreeNode::node(
            0.5_f64,
            BinTreeNode::dummy(0.25_f64),
            BinTreeNode::dummy(0.25_f64),
        );

        assert_eq!(opt_tree, exp_tree);

        fn case1_tree(k: (f64, f64), d: (f64, f64, f64)) -> BinTreeNode {
            BinTreeNode::node(
                k.0,
                BinTreeNode::dummy(d.0),
                BinTreeNode::node(k.1, BinTreeNode::dummy(d.1), BinTreeNode::dummy(d.2)),
            )
        }

        fn case2_tree(k: (f64, f64), d: (f64, f64, f64)) -> BinTreeNode {
            BinTreeNode::node(
                k.1,
                BinTreeNode::node(k.0, BinTreeNode::dummy(d.0), BinTreeNode::dummy(d.1)),
                BinTreeNode::dummy(d.2),
            )
        }

        let k = vec![0.2_f64, 0.1_f64];
        let d = vec![0.1_f64, 0.5_f64, 0.1_f64];

        let opt_tree =
            construct_optimal_tree(&k, &d);

        assert!(
            opt_tree == case1_tree((k[0], k[1]), (d[0], d[1], d[2])) ||
            opt_tree == case2_tree((k[0], k[1]), (d[0], d[1], d[2])) 
        );
    }
}
