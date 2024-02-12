use either::Either;
use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize, m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let (a, b) = merge_mark(a, b);
    println!("{}", a.iter().join(" "));
    println!("{}", b.iter().join(" "));
}

fn merge_mark(mut a: Vec<usize>, mut b: Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    a.reverse();
    b.reverse();
    let mut merge: Vec<Either<(), ()>> = vec![];

    while let (Some(ai), Some(bi)) = (a.last(), b.last()) {
        if ai < bi {
            let _ = a.pop();
            merge.push(Either::Left(()))
        } else {
            let _ = b.pop();
            merge.push(Either::Right(()))
        }
    }
    // a or b is empty()

    while a.pop().is_some() {
        merge.push(Either::Left(()))
    }

    while b.pop().is_some() {
        merge.push(Either::Right(()))
    }

    let (mut a, mut b): (Vec<usize>, Vec<usize>) = (vec![], vec![]);
    for (i, mi) in merge.iter().enumerate() {
        match mi {
            Either::Left(_) => {
                a.push(i + 1);
            }
            Either::Right(_) => {
                b.push(i + 1);
            }
        }
    }

    (a, b)
}
