use std::{collections::HashMap, ops::Range};

fn main() {
    let (rules, parts) = input();
    println!("{}", compute_part2(rules, parts));
}

fn compute_part1(rules: Vec<(String, String)>, parts: Vec<(usize, usize, usize, usize)>) -> usize {
    let rules = get_rules_map(rules);
    parts
        .into_iter()
        .map(|part| Part {
            x: part.0,
            m: part.1,
            a: part.2,
            s: part.3,
        })
        .filter(|part| is_accepted(&rules, part))
        .map(|part| part.x + part.m + part.a + part.s)
        .sum()
}

fn compute_part2_input(rules: &HashMap<String, Rule>) -> usize {
    let mut range_work = vec![(
        PartRange {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        },
        "in".to_owned(),
    )];
    let mut accepted: Vec<PartRange> = vec![];
    while let Some((range, now)) = range_work.pop() {
        if now == "A" {
            accepted.push(range);
            continue;
        } else if now == "R" {
            continue;
        }
        let rule = rules.get(&now).unwrap();
        let ranges = rule.next_rule_name_range(&range);
        range_work.extend(ranges);
    }
    accepted.into_iter().map(|part_range| {
        let x_len = part_range.x.end - part_range.x.start;
        let m_len = part_range.m.end - part_range.m.start;
        let a_len = part_range.a.end - part_range.a.start;
        let s_len = part_range.s.end - part_range.s.start;
        x_len * m_len * a_len * s_len
    }).sum()
}

fn compute_part2(rules: Vec<(String, String)>, _parts: Vec<(usize, usize, usize, usize)>) -> usize {
    let rules = get_rules_map(rules);
    compute_part2_input(&rules)
}

fn is_accepted(rules: &HashMap<String, Rule>, part: &Part) -> bool {
    let mut now: String = "in".to_owned();
    while now != "A" && now != "R" {
        let rule = rules.get(&now).unwrap();
        now = rule.next_rule_name(part);
    }
    now == "A"
}

fn get_rules_map(rules: Vec<(String, String)>) -> HashMap<String, Rule> {
    rules
        .into_iter()
        .map(|(name, rule)| (name, Rule::new(rule)))
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PartRange {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

impl PartRange {
    fn is_valid(&self) -> bool {
        !self.x.is_empty() && !self.m.is_empty() && !self.a.is_empty() && !self.s.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RuleBranch {
    XLneq(usize),
    XGneq(usize),
    MLneq(usize),
    MGneq(usize),
    ALneq(usize),
    AGneq(usize),
    SLneq(usize),
    SGneq(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule {
    branch: Vec<(RuleBranch, String)>,
    finally: String,
}

impl Rule {
    fn new(rule_string: String) -> Self {
        let mut branch = vec![];
        let mut finally = String::new();
        for rule in rule_string.split(',') {
            if let Some(i) = rule.find(':') {
                let val = rule[2..i].parse::<usize>().unwrap();
                let goto = rule[i + 1..].to_owned();
                let rule = match (&rule[0..1], &rule[1..2]) {
                    ("x", "<") => RuleBranch::XLneq(val),
                    ("x", ">") => RuleBranch::XGneq(val),
                    ("m", "<") => RuleBranch::MLneq(val),
                    ("m", ">") => RuleBranch::MGneq(val),
                    ("a", "<") => RuleBranch::ALneq(val),
                    ("a", ">") => RuleBranch::AGneq(val),
                    ("s", "<") => RuleBranch::SLneq(val),
                    ("s", ">") => RuleBranch::SGneq(val),
                    _ => unreachable!("{:?} {:?}", &rule[0..1], &rule[1..2]),
                };
                branch.push((rule, goto));
            } else {
                finally = rule.to_owned();
                break;
            }
        }
        Self { branch, finally }
    }
    fn next_rule_name(&self, part: &Part) -> String {
        for (rule, goto) in &self.branch {
            match rule {
                RuleBranch::XLneq(val) if part.x < *val => {
                    return goto.to_owned();
                }
                RuleBranch::XGneq(val) if part.x > *val => {
                    return goto.to_owned();
                }
                RuleBranch::MLneq(val) if part.m < *val => {
                    return goto.to_owned();
                }
                RuleBranch::MGneq(val) if part.m > *val => {
                    return goto.to_owned();
                }
                RuleBranch::ALneq(val) if part.a < *val => {
                    return goto.to_owned();
                }
                RuleBranch::AGneq(val) if part.a > *val => {
                    return goto.to_owned();
                }
                RuleBranch::SLneq(val) if part.s < *val => {
                    return goto.to_owned();
                }
                RuleBranch::SGneq(val) if part.s > *val => {
                    return goto.to_owned();
                }
                _ => {}
            }
        }
        self.finally.clone()
    }
    fn next_rule_name_range(&self, part_range: &PartRange) -> Vec<(PartRange, String)> {
        let mut ranges: Vec<(PartRange, String)> = vec![];
        let mut else_range: PartRange = part_range.clone();
        for (rule, goto) in &self.branch {
            match rule {
                RuleBranch::XLneq(val) => {
                    ranges.push((
                        PartRange {
                            x: else_range.x.start..*val,
                            ..else_range.clone()
                        },
                        goto.to_owned(),
                    ));
                    else_range = PartRange {
                        x: *val..else_range.x.end,
                        ..else_range
                    };
                }
                RuleBranch::XGneq(val) => {
                    ranges.push((
                        PartRange {
                            x: *val + 1..else_range.x.end,
                            ..else_range.clone()
                        },
                        goto.to_owned(),
                    ));
                    else_range = PartRange {
                        x: else_range.x.start..*val + 1,
                        ..else_range
                    };
                }
                RuleBranch::MLneq(val) => {
                    ranges.push((
                        PartRange {
                            m: else_range.m.start..*val,
                            ..else_range.clone()
                        },
                        goto.to_owned(),
                    ));
                    else_range = PartRange {
                        m: *val..else_range.m.end,
                        ..else_range
                    };
                }
                RuleBranch::MGneq(val) => {
                    ranges.push((
                        PartRange {
                            m: *val + 1..else_range.m.end,
                            ..else_range.clone()
                        },
                        goto.to_owned(),
                    ));
                    else_range = PartRange {
                        m: else_range.m.start..*val + 1,
                        ..else_range
                    };
                }
                RuleBranch::ALneq(val) => {
                    ranges.push((
                        PartRange {
                            a: else_range.a.start..*val,
                            ..else_range.clone()
                        },
                        goto.to_owned(),
                    ));
                    else_range = PartRange {
                        a: *val..else_range.a.end,
                        ..else_range
                    };
                }
                RuleBranch::AGneq(val) => {
                    ranges.push((
                        PartRange {
                            a: *val + 1..else_range.a.end,
                            ..else_range.clone()
                        },
                        goto.to_owned(),
                    ));
                    else_range = PartRange {
                        a: else_range.a.start..*val + 1,
                        ..else_range
                    };
                }
                RuleBranch::SLneq(val) => {
                    ranges.push((
                        PartRange {
                            s: else_range.s.start..*val,
                            ..else_range.clone()
                        },
                        goto.to_owned(),
                    ));
                    else_range = PartRange {
                        s: *val..else_range.s.end,
                        ..else_range
                    };
                }
                RuleBranch::SGneq(val) => {
                    ranges.push((
                        PartRange {
                            s: *val + 1..else_range.s.end,
                            ..else_range.clone()
                        },
                        goto.to_owned(),
                    ));
                    else_range = PartRange {
                        s: else_range.s.start..*val + 1,
                        ..else_range
                    };
                }
            }
        }
        ranges.push((else_range, self.finally.to_owned()));
        ranges
            .into_iter()
            .filter(|(part_range, _)| part_range.is_valid())
            .collect::<Vec<_>>()
    }
}

fn input() -> (Vec<(String, String)>, Vec<(usize, usize, usize, usize)>) {
    let string = std::fs::read_to_string("inout/day19.in").unwrap();
    let mut lines = string.lines();
    let mut rules = vec![];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let i = line.find('{').unwrap();
        let first = line[..i].to_owned();
        let v: String = (&line[i + 1..line.len() - 1]).to_owned();
        rules.push((first, v));
    }

    let mut parts = vec![];
    while let Some(line) = lines.next() {
        let tuples: Vec<usize> = (&line[1..line.len() - 1])
            .split(',')
            .map(|str| match str[2..].parse::<usize>() {
                Ok(i) => i,
                Err(_) => unreachable!("{}", &str[2..]),
            })
            .collect();
        parts.push((tuples[0], tuples[1], tuples[2], tuples[3]));
    }
    (rules, parts)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rule_test() {
        let rule = Rule::new("m>3771:A,a>2552:R,A".to_string());
        let part = Part {
            x: 100,
            m: 100,
            a: 100,
            s: 100,
        };
        assert_eq!(rule.next_rule_name(&part), "A");
        let part = Part {
            x: 100,
            m: 3772,
            a: 100,
            s: 100,
        };
        assert_eq!(rule.next_rule_name(&part), "A");
        let part = Part {
            x: 100,
            m: 3770,
            a: 2553,
            s: 100,
        };
        assert_eq!(rule.next_rule_name(&part), "R");
    }
    #[test]
    fn compute_part1_test() {
        let rules = get_rules_map(vec![
            ("px".to_owned(), "a<2006:qkq,m>2090:A,rfg".to_owned()),
            ("pv".to_owned(), "a>1716:R,A".to_owned()),
            ("lnx".to_owned(), "m>1548:A,A".to_owned()),
            ("rfg".to_owned(), "s<537:gd,x>2440:R,A".to_owned()),
            ("qs".to_owned(), "s>3448:A,lnx".to_owned()),
            ("qkq".to_owned(), "x<1416:A,crn".to_owned()),
            ("crn".to_owned(), "x>2662:A,R".to_owned()),
            ("in".to_owned(), "s<1351:px,qqz".to_owned()),
            ("qqz".to_owned(), "s>2770:qs,m<1801:hdj,R".to_owned()),
            ("gd".to_owned(), "a>3333:R,R".to_owned()),
            ("hdj".to_owned(), "m>838:A,pv".to_owned()),
        ]);
        let parts = vec![
            Part {
                x: 787,
                m: 2655,
                a: 1222,
                s: 2876,
            },
            Part {
                x: 1679,
                m: 44,
                a: 2067,
                s: 496,
            },
            Part {
                x: 2036,
                m: 264,
                a: 79,
                s: 2244,
            },
            Part {
                x: 2461,
                m: 1339,
                a: 466,
                s: 291,
            },
            Part {
                x: 2127,
                m: 1623,
                a: 2188,
                s: 1013,
            },
        ];
        assert_eq!(
            parts
                .into_iter()
                .map(|part| is_accepted(&rules, &part))
                .collect::<Vec<_>>(),
            vec![true, false, true, false, true]
        );
    }
    #[test]
    fn rule_range_test() {
        let one_rule = Rule::new("x<100:a,s>100:b,c".to_owned());
        let range = PartRange {
            x: 1..200,
            m: 1..200,
            a: 1..200,
            s: 1..200,
        };
        let ranges = one_rule.next_rule_name_range(&range);
        let expect = vec![
            (
                PartRange {
                    x: 1..100,
                    m: 1..200,
                    a: 1..200,
                    s: 1..200,
                },
                "a".to_owned(),
            ),
            (
                PartRange {
                    x: 100..200,
                    m: 1..200,
                    a: 1..200,
                    s: 101..200,
                },
                "b".to_owned(),
            ),
            (
                PartRange {
                    x: 100..200,
                    m: 1..200,
                    a: 1..200,
                    s: 1..101,
                },
                "c".to_owned(),
            ),
        ];
        assert_eq!(ranges, expect);

        let range = PartRange {
            x: 1..100,
            m: 1..200,
            a: 1..200,
            s: 1..200,
        };
        let ranges = one_rule.next_rule_name_range(&range);
        let expect = vec![(
            PartRange {
                x: 1..100,
                m: 1..200,
                a: 1..200,
                s: 1..200,
            },
            "a".to_owned(),
        )];
        assert_eq!(ranges, expect);

        let range = PartRange {
            x: 100..200,
            m: 1..200,
            a: 1..200,
            s: 1..101,
        };
        let ranges = one_rule.next_rule_name_range(&range);
        let expect = vec![(
            PartRange {
                x: 100..200,
                m: 1..200,
                a: 1..200,
                s: 1..101,
            },
            "c".to_owned(),
        )];
        assert_eq!(ranges, expect);
    }
    #[test]
    fn compute_part2_test() {
        let rules = get_rules_map(vec![
            ("px".to_owned(), "a<2006:qkq,m>2090:A,rfg".to_owned()),
            ("pv".to_owned(), "a>1716:R,A".to_owned()),
            ("lnx".to_owned(), "m>1548:A,A".to_owned()),
            ("rfg".to_owned(), "s<537:gd,x>2440:R,A".to_owned()),
            ("qs".to_owned(), "s>3448:A,lnx".to_owned()),
            ("qkq".to_owned(), "x<1416:A,crn".to_owned()),
            ("crn".to_owned(), "x>2662:A,R".to_owned()),
            ("in".to_owned(), "s<1351:px,qqz".to_owned()),
            ("qqz".to_owned(), "s>2770:qs,m<1801:hdj,R".to_owned()),
            ("gd".to_owned(), "a>3333:R,R".to_owned()),
            ("hdj".to_owned(), "m>838:A,pv".to_owned()),
        ]);
        let res = compute_part2_input(&rules);
        assert_eq!(res, 167409079868000);
    }
}
