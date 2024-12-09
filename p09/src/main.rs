use std::fs;

type DiskMap = String;
type Layout = Vec<Option<u32>>;

fn main() {
    println!("Hello, world!");
}

fn parse(input: &'static str) -> DiskMap {
    fs::read_to_string(input)
        .unwrap()
        .trim_end()
        .to_string()
}

fn expand(diskmap: &DiskMap) -> Layout {
    let mut list = vec![];
    let mut id = 0;

    for i in 0..diskmap.len() {
        let c = diskmap.chars().nth(i).unwrap();
        let d = c.to_digit(10).unwrap();

        if i % 2 == 0 {
            for _ in 0..d { list.push(Some(id)) }
            id += 1;
        } else {
            for _ in 0..d { list.push(None) }
        }
    }

    list
}

fn compress(layout: &mut Layout) {
    loop {
        let last = layout.pop().unwrap();
        if layout.iter().all(|n| n.is_some()) {
            break;
        }

        let pos = layout.iter().position(|&n| n.is_none()).unwrap();
        layout[pos] = last;

        if layout.iter().all(|n| n.is_some()) {
            break;
        }
    }
}

fn checksum(layout: &Layout) -> u32 {
    (0..layout.len())
        .map(|i| (i as u32) * layout[i].unwrap())
        .sum()
}


#[test]
fn test_expand_compress_easy() {
    let example = String::from("12345");
    let mut list = expand(&example);

    assert_eq!(list[0], Some(0));
    assert_eq!(list[list.len() - 1], Some(2));

    compress(&mut list);
    assert_eq!(list.len(), 9);
    assert_eq!(checksum(&list), 60);
}

#[test]
fn test_expand_compress() {
    let dm = parse("1");
    let mut list = expand(&dm);

    assert_eq!(list[0], Some(0));
    assert_eq!(list[list.len() - 1], Some(9));

    compress(&mut list);
    assert_eq!(list.len(), 28);
    assert_eq!(checksum(&list), 1928);
}
