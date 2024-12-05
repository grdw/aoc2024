use std::fs;
use std::cmp::Ordering;

type OrderRules = (u32, u32);
type Pages = Vec<u32>;

fn main() {
    let (ordered_rules, pages) = parse("input");

    println!("p1 {}", ordered_pages(&ordered_rules, &pages));
    println!("p2 {}", unordered_pages(&ordered_rules, &pages));
}

fn parse(input: &'static str) -> (Vec<OrderRules>, Vec<Pages>) {
    let vector = fs::read_to_string(input).unwrap();
    let (r_order_rules, r_pages) = vector.split_once("\n\n").unwrap();

    let order_rules = r_order_rules
        .split("\n")
        .map(|rules| {
            let (rl, rr) = rules.split_once("|").unwrap();
            let l = rl.parse::<u32>().unwrap();
            let r = rr.parse::<u32>().unwrap();

            (l, r)
        })
        .collect();

    let pages = r_pages
        .split_terminator("\n")
        .map(|pages| {
            pages
                .split(",")
                .map(|i| i.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    (order_rules, pages)
}

fn ordered_pages(rules: &Vec<OrderRules>, book: &Vec<Pages>) -> u32 {
    book
        .iter()
        .filter(|pages| is_ordered(rules, pages))
        .map(|pages| pages[pages.len() / 2])
        .sum()
}

#[test]
fn test_count_ordered_pages() {
    let (ordered_rules, pages) = parse("1");

    assert_eq!(ordered_pages(&ordered_rules, &pages), 143);
}

fn unordered_pages(rules: &Vec<OrderRules>, book: &Vec<Pages>) -> u32 {
    let unordered_pages: Vec<Pages> = book
        .into_iter()
        .filter(|pages| !is_ordered(rules, pages))
        .map(|pages| pages.clone())
        .collect();

    let mut total = 0;
    for mut t in unordered_pages.into_iter() {
        //let mut t = u.clone();

        t.sort_by(|a, b| {
            let lr = rules
                .iter()
                .find(|&&(l, r)| l == *a && r == *b);

            if lr.is_some() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        total += t[t.len() / 2];
    }

    total
}


fn is_ordered(rules: &Vec<OrderRules>, pages: &Pages) -> bool {
    rules.iter().all(|rule| {
        let lf = pages.iter().position(|&p| p == rule.0);
        let rf = pages.iter().position(|&p| p == rule.1);

        match (lf, rf) {
            (Some(l), Some(r)) => r > l,
            _ => true
        }
    })
}

#[test]
fn test_count_unordered_pages() {
    let (ordered_rules, pages) = parse("1");

    assert_eq!(unordered_pages(&ordered_rules, &pages), 123);
}
