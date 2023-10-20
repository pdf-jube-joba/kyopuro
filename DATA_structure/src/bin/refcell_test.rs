use std::cell::RefCell;
fn main() {
    {
        // refcell を作る
        let refcell = RefCell::new(1);

        // refcell へのアクセスは borrow して出てきた Ref<_> を deref する
        assert_eq!(*refcell.borrow(), 1);

        // refcell の中身の書き換え
        // refcell が mut な変数でないことに注意
        *refcell.borrow_mut() = 2;
    }

    {
        // mut をつけてない
        let refcell = RefCell::new(1);

        // & を 2 つ持ちそれぞれから変更する
        // これを両方 &mut にはできない
        let r1 = &refcell;
        let r2 = &refcell;

        // 不変参照を通じて変更できる
        *r1.borrow_mut() = 3;
        *r2.borrow_mut() = 4;
    }
    let mut refcell = RefCell::new(1);

    let r1 = refcell.borrow();
    let r2 = refcell.borrow();
    let r3 = refcell.borrow_mut();

}