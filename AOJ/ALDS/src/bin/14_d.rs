mod suffix_array {
    use std::collections::BTreeMap;

    // range[c] = (l, r) <=> if s is sorted then any menber of (suffix array of s)[l, r) is start at c
    fn get_bucket_range<T: Ord + std::hash::Hash + Clone>(s: &[T]) -> BTreeMap<T, (usize, usize)> {
        let mut map = BTreeMap::new();
        for c in s {
            map.entry(c.clone())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        let mut range = BTreeMap::new();
        let mut count = 1;
        for (c, s) in map {
            range.insert(c, (count, count + s));
            count += s;
        }
        range
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    enum SLType {
        S,
        L,
    }

    fn construct_sl_type<T: Ord>(s: &[T]) -> Vec<SLType> {
        let m = s.len();
        let mut prev = SLType::S;
        let mut v = (0..=m)
            .rev()
            .map(|i| {
                let this = if i == m {
                    SLType::S
                } else if s.get(i) == s.get(i + 1) {
                    prev
                } else if s.get(i) < s.get(i + 1) {
                    SLType::S
                } else {
                    SLType::L
                };
                prev = this;
                this
            })
            .collect::<Vec<_>>();
        v.reverse();
        v
    }

    // input suffix_array filled by none
    // output suffix_array filled all of LMS-type index by reverse
    fn first_lms_insert<T: Ord + Clone>(
        s: &[T],
        suffix_array: &mut [Option<usize>],
        sl_type_array: &[SLType],
        bucket_range: &BTreeMap<T, (usize, usize)>,
    ) {
        let m = s.len();
        let is_lms =
            |i: usize| i > 0 && sl_type_array[i] == SLType::S && sl_type_array[i - 1] == SLType::L;

        // remember index of where we putted LMS in suffix array
        // lms_back_index[c] = i <=> for any i,
        // - suffix_array[l, i) is None
        // - suffix_array[i, r) consists of all LMS index larger than i starting at c,
        // where bucket_range[c] = [l, r)
        let mut lms_back_index: BTreeMap<T, usize> = BTreeMap::new();
        for (c, (_start, end)) in bucket_range {
            lms_back_index.insert(c.clone(), *end);
        }

        // at any head of loop
        for i in (0..m).rev() {
            if is_lms(i) {
                let next_insert = *lms_back_index.get(&s[i]).unwrap() - 1;
                suffix_array[next_insert] = Some(i);
                *lms_back_index.get_mut(&s[i]).unwrap() -= 1;
            }
        }

        // this holds by definition
        suffix_array[0] = Some(m);
    }

    // input suffix_array with lms
    // output suffix_array with l-type (lms is annihilated)
    fn ltype_from_lms<T: Ord + Clone>(
        s: &[T],
        suffix_array: &mut [Option<usize>],
        sl_type_array: &[SLType],
        bucket_range: &BTreeMap<T, (usize, usize)>,
    ) {
        let m = s.len();
        // remember index for which we put L-type
        // ltype_front_index[c] = i <=> for any i,
        // - suffix_array[l, i) is putted L-type index
        // - suffix_array[i, r) is none or LMS-type,
        // where bucket_range[c] = [l, r)
        let mut ltype_front_index: BTreeMap<T, usize> = BTreeMap::new();
        for (c, (start, _end)) in bucket_range {
            ltype_front_index.insert(c.clone(), *start);
        }

        // insert L-type index
        for i in 0..=m {
            if let Some(index) = suffix_array[i] {
                if index > 0 && sl_type_array[index - 1] == SLType::L {
                    let target_index = index - 1;
                    let next_insert = *ltype_front_index.get(&s[target_index]).unwrap();
                    suffix_array[next_insert] = Some(target_index);
                    *ltype_front_index.get_mut(&s[target_index]).unwrap() += 1;
                }
            }
        }

        // condition hold after loop
        // so we remove lms-type by using ltype_front_index[c] = i (suffix_array[i, r) should be None
        // annihilate lms
        for (c, index) in ltype_front_index {
            let r = bucket_range.get(&c).unwrap().1;
            for i in index..r {
                suffix_array[i] = None;
            }
        }

        suffix_array[0] = None;
    }

    // input suffix_array which is inserted of all L-type index
    // output
    fn stype_from_ltype<T: Ord + Clone>(
        s: &[T],
        suffix_array: &mut [Option<usize>],
        sl_type_array: &[SLType],
        bucket_range: &BTreeMap<T, (usize, usize)>,
    ) {
        let m = s.len();

        // remember index for which we put S-type
        // stype_back_index[c] = i <=> for any i,
        // - suffix_array[l, i) is L-type index or None
        // - suffix_array[i, r) is S-type
        // where bucket_range[c] = [l, r)
        let mut stype_back_index: BTreeMap<T, usize> = BTreeMap::new();
        for (c, (_start, end)) in bucket_range {
            stype_back_index.insert(c.clone(), *end);
        }

        // insert S-type
        for i in (0..=m).rev() {
            if let Some(index) = suffix_array[i] {
                if index > 0 && sl_type_array[index - 1] == SLType::S {
                    let target_index = index - 1;
                    let next_insert = *stype_back_index.get(&s[target_index]).unwrap() - 1;
                    suffix_array[next_insert] = Some(target_index);
                    *stype_back_index.get_mut(&s[target_index]).unwrap() -= 1;
                }
            }
        }

        suffix_array[0] = Some(m);
    }

    // input suffix_array_filled_with_SL-type
    // return LMS-index with same order
    fn collect_lms_block<T: Ord + Clone>(
        s: &[T],
        suffix_array: &mut [Option<usize>],
        sl_type_array: &[SLType],
        bucket_range: &BTreeMap<T, (usize, usize)>,
    ) {
        let m = s.len();
        let is_lms =
            |i: usize| i > 0 && sl_type_array[i] == SLType::S && sl_type_array[i - 1] == SLType::L;

        let ordered_lms_index: Vec<usize> = (0..=m).filter(|i| is_lms(*i)).collect();

        todo!()
    }
    pub struct SuffixArray<'a, T: Ord + Clone> {
        s: &'a [T],
        suffix_array: Vec<usize>,
    }

    impl<'a, T> SuffixArray<'a, T> where T: Ord + Clone + std::hash::Hash {
        // using SA-IS
        pub fn new(s: &'a [T]) -> Self {
            let m = s.len();

            let sl_type_array: Vec<SLType> = construct_sl_type(s);
            let is_lms = |i: usize| {
                if i == 0 {
                    false
                } else {
                    sl_type_array[i] == SLType::S && sl_type_array[i - 1] == SLType::L
                }
            };

            let mut suffix_array: Vec<Option<usize>> = vec![None; m + 1];

            let bucket_range = get_bucket_range(s);

            // 1. put LMS index at bucket in suffix_array by reverse order
            first_lms_insert(s, &mut suffix_array, &sl_type_array, &bucket_range);
            ltype_from_lms(s, &mut suffix_array, &sl_type_array, &bucket_range);
            stype_from_ltype(s, &mut suffix_array, &sl_type_array, &bucket_range);

            Self {
                s,
                suffix_array: suffix_array.into_iter().map(|i| i.unwrap()).collect(),
            }
        }
        pub fn find_substr(&self, substr: &'a [u8]) -> Option<usize> {
            if substr.is_empty() {
                return Some(0);
            }
            unimplemented!()
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_sl_type() {
            let a = b"";
            assert_eq!(construct_sl_type(a), vec![SLType::S]);

            let a = b"1";
            assert_eq!(construct_sl_type(a), vec![SLType::L, SLType::S]);

            let a = b"ab";
            assert_eq!(construct_sl_type(a), vec![SLType::S, SLType::L, SLType::S]);

            let a = b"ba";
            assert_eq!(construct_sl_type(a), vec![SLType::L, SLType::L, SLType::S]);

            let a = b"aabbbaacc";
            assert_eq!(
                construct_sl_type(a),
                vec![
                    SLType::S,
                    SLType::S,
                    SLType::L,
                    SLType::L,
                    SLType::L,
                    SLType::S,
                    SLType::S,
                    SLType::L,
                    SLType::L,
                    SLType::S
                ]
            );
        }
        #[test]
        fn test_get_bucket() {
            let a = b"";
            assert_eq!(get_bucket_range(a).iter().collect::<Vec<_>>(), vec![]);

            let a = b"a";
            assert_eq!(
                get_bucket_range(a).into_iter().collect::<Vec<_>>(),
                vec![(b'a', (1, 2)),]
            );

            let a = b"aabbacc";
            assert_eq!(
                get_bucket_range(a).into_iter().collect::<Vec<_>>(),
                vec![(b'a', (1, 4)), (b'b', (4, 6)), (b'c', (6, 8)),]
            );
        }
        #[test]
        fn insert_test() {
            fn test(
                s: &[u8],
                expect_after_lms_insert: Vec<Option<usize>>,
                expect_after_ltype: Vec<Option<usize>>,
                expect_after_stype: Vec<Option<usize>>,
            ) {
                let m = s.len();
                let mut suffix_array: Vec<Option<usize>> = vec![None; m + 1];
                let sl_type_array = construct_sl_type(s);
                let bucket_range = get_bucket_range(s);
                first_lms_insert(s, &mut suffix_array, &sl_type_array, &bucket_range);
                assert_eq!(suffix_array, expect_after_lms_insert);
                ltype_from_lms(s, &mut suffix_array, &sl_type_array, &bucket_range);
                assert_eq!(suffix_array, expect_after_ltype);
                stype_from_ltype(s, &mut suffix_array, &sl_type_array, &bucket_range);
                assert_eq!(suffix_array, expect_after_stype);
            }

            let s = b"";
            //                  S
            let v1 = vec![Some(0)];
            let v2 = vec![None];
            let v3 = vec![Some(0)];
            test(s, v1, v2, v3);

            let s = b"a";
            //                  LS
            // LMS               *

            // $a
            // 1-
            // -0
            // 10
            let v1 = vec![Some(1), None];
            let v2 = vec![None, Some(0)];
            let v3 = vec![Some(1), Some(0)];
            test(s, v1, v2, v3);

            let s = b"ba";
            //                  LLS
            // LMS                *

            // $ab
            // 2--
            // -10
            // 210
            let v1 = vec![Some(2), None, None];
            let v2 = vec![None, Some(1), Some(0)];
            let v3 = vec![Some(2), Some(1), Some(0)];
            test(s, v1, v2, v3);

            let s = b"ab";
            //                  SLS
            // LMS                *

            // $ab
            // 2--
            // --1
            // 201
            let v1 = vec![Some(2), None, None];
            let v2 = vec![None, None, Some(1)];
            let v3 = vec![Some(2), Some(0), Some(1)];
            test(s, v1, v2, v3);

            let s = b"bab";
            //                  LSLS
            // LMS               1 3

            // $abb
            // 31--
            // --20
            // 3120
            let v1 = vec![Some(3), Some(1), None, None];
            let v2 = vec![None, None, Some(2), Some(0)];
            let v3 = vec![Some(3), Some(1), Some(2), Some(0)];
            test(s, v1, v2, v3);

            let s = b"babba";
            //                  LSLLLS
            // LMS               1   5

            // $aabbb
            // 5-1---
            // -4-302
            // 541302
            let v1 = vec![Some(5), None, Some(1), None, None, None];
            let v2 = vec![None, Some(4), None, Some(3), Some(0), Some(2)];
            let v3 = vec![Some(5), Some(4), Some(1), Some(3), Some(0), Some(2)];
            test(s, v1, v2, v3);

            let s = b"acbcaca";
            //                  SLSLSLLS
            // LMS                2 4  7

            // $aaabccc
            // 7--42---
            // -6---531
            // 76402531
            let v1 = vec![Some(7), None, None, Some(4), Some(2), None, None, None];
            let v2 = vec![None, Some(6), None, None, None, Some(5), Some(3), Some(1)];
            let v3 = vec![
                Some(7),
                Some(6),
                Some(4),
                Some(0),
                Some(2),
                Some(5),
                Some(3),
                Some(1),
            ];
            test(s, v1, v2, v3);
        }
        #[test]
        fn test_suffix_array() {
            let a = b"";
            let _ = SuffixArray::new(a);

            let a = b"a";
            let _ = SuffixArray::new(a);
        }
    }
}

fn main() {
    use suffix_array::*;
    let (t, pis) = input();
    let suffix_array = SuffixArray::new(&t);
    for pi in pis {
        if suffix_array.find_substr(&pi).is_some() {
            println!("1");
        } else {
            println!("0");
        }
    }
}

fn input() -> (Vec<u8>, Vec<Vec<u8>>) {
    use std::io::BufRead;
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let t = {
        stdin.read_line(&mut buf).unwrap();
        buf.trim().as_bytes().to_vec()
    };

    let q = {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        buf.trim().parse::<usize>().unwrap()
    };

    let pi = (0..q)
        .map(|_| {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            buf.trim().as_bytes().to_vec()
        })
        .collect();

    (t, pi)
}
