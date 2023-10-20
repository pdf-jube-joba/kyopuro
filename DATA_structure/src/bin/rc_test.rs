use std::rc::Rc;

fn main() {
    // rc を作る
    let mut rc1 = Rc::new(1);

    // rc へのアクセスは * をつかう
    assert_eq!(*rc1, 1);

    // rc 内部を変更するには get_mut を使う
    *Rc::get_mut(&mut rc1).unwrap() = 2;
    assert_eq!(*rc1, 2);

    // *rc1 = 2 ではいけない
    // rc は TRPL では複数の所有権と説明されているが、
    // 所有しているにもかかわらず mut をつけても変更することができない
    // （ライフタイムのなくなった）共有された参照の方が近い

    // rc をクローンする
    let rc2 = Rc::clone(&rc1);

    // 書き換えは 2 つ以上の参照があると失敗する
    assert!(Rc::get_mut(&mut rc1).is_none());
}