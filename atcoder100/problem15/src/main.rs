fn main() {
    proconio::input! {
        n: usize,
        xy: [(usize, usize); n],
    }

    println!("{}", average(&xy));

}

fn next(a: &mut [usize]) -> Option<()> {
    let n = a.len();
    let max_index = match (0..n-1).filter(|i|{a[*i] < a[i+1]}).max() {
        Some(i) => i,
        None => {return None;}
    };
    let swap_index = {
        (max_index+1..n).filter(|i|{a[*i] > a[max_index]}).max().unwrap()
    };
    let temp = a[max_index];
    a[max_index] = a[swap_index];
    a[swap_index] = temp;
    a[max_index+1..n].reverse();
    Some(())
}

#[test]
fn next_test() {
    let mut a = vec![1,2,3];
    next(&mut a).unwrap();
    assert_eq!(a, vec![1,3,2]);
    next(&mut a).unwrap();
    assert_eq!(a, vec![2,1,3]);
    next(&mut a).unwrap();
    assert_eq!(a, vec![2,3,1]);
    next(&mut a).unwrap();
    assert_eq!(a, vec![3,1,2]);
    next(&mut a).unwrap();
    assert_eq!(a, vec![3,2,1]);
    assert_eq!(next(&mut a), None);
    assert_eq!(a, vec![3,2,1]);
}

fn fact(acc: usize, n: usize) -> usize {
    if n == 0 {acc} else {fact(n * acc , n - 1)}
}

fn average(xy: &[(usize, usize)]) -> f32 {
    let n = xy.len();
    let mut sum = 0.0;
    let mut perm = (0..n).collect::<Vec<usize>>();
    loop {
        println!("{:?}", perm);
        (0..n-1).for_each(|i|{
            let (x1, y1) = xy[perm[i]];
            let (x2, y2) = xy[perm[i+1]];
            let distance = ((x1.abs_diff(x2).pow(2) + y1.abs_diff(y2).pow(2)) as f32).sqrt();
            sum += distance;
            println!("1:{:?} 2:{:?} d:{}", (x1, y1), (x2, y2), distance);
        });
        if next(&mut perm).is_none() {break;}
    }
    sum / fact(1, n) as f32
}

#[test]
fn average_test_1() {
    let xy = vec![(0,0), (0,1), (1,0)];
    println!("{:?}", average(&xy));
    panic!()
}
