fn main() {
    proconio::input! {
        n: usize,
        mut s: usize,
    }

    let a: Vec<usize> = {
        let mut vec = Vec::new();
        (0..n).for_each(|_|{
            vec.push(s % 10);
            s = s / 10;
        });
        vec
    };

    println!("{}", count(&a));

}

fn count(a: &[usize]) -> usize {
    println!("{:?}", a);
    let mut count = 0;
    for i in 0..10 {
        for j in 0..10 {
            for k in 0..10 {
                let result =
                    a.to_owned().into_iter()
                    .skip_while(|x|{*x != i})
                    .skip(1)
                    .skip_while(|x|{*x != j})
                    .skip(1)
                    .skip_while(|x|{*x != k})
                    .peekable().peek().is_some();
//                println!("{} {} {} {}", i, j, k, result);
                if result {count += 1;}   
            }
        }
    };
    count
}

#[test]
fn example1() {
    assert_eq!(count(&[0,2,2,4]), 3);
    assert_eq!(count(&[1,2,3,1,2,3]), 17);
    assert_eq!(count(&[3,1,4,1,5,9,2,6,5,3,5,8,9,7,9,3,2,3,8]), 329);
}
