use std::fs;
use std::cmp;

type Layout = Vec<usize>;
type DiskMap = (Layout, Layout);

fn main() {
    let (mut files, spaces) = parse("input");
    println!("p1 {}", checksum(&mut files, &spaces));
    println!("p2 {}", checksum_whole(&mut files, &spaces));
}

fn parse(input: &'static str) -> DiskMap {
    let diskmap = fs::read_to_string(input)
        .unwrap()
        .trim_end()
        .to_string();

    let mut spaces = vec![];
    let mut files = vec![];

    for i in 0..diskmap.len() {
        let c = diskmap.chars().nth(i).unwrap();
        let d = c.to_digit(10).unwrap() as usize;

        if i % 2 == 0 {
            files.push(d);
        } else {
            spaces.push(d);
        }
    }

    (files, spaces)
}

fn checksum(files: &mut Layout, spaces: &Layout) -> usize {
    let mut compressed = vec![];
    let mut findex = 0;
    let mut frindex = files.len() - 1;
    let mut sindex = 0;

    loop {
        let l = files[findex];

        extend(&mut compressed, findex, l);
        files[findex] = 0;
        findex += 1;

        if files.iter().all(|&fl| fl == 0) {
            break;
        }

        let sl = spaces[sindex];
        let fl = files[frindex];

        if sl >= fl {
            let mut dl = sl;

            while frindex > 0 && dl > 0 {
                let ffl = cmp::min(files[frindex], dl);

                extend(&mut compressed, frindex, ffl);
                files[frindex] -= ffl;
                dl -= ffl;

                if files[frindex] == 0 {
                    frindex -= 1;
                }
            }
        } else {
            extend(&mut compressed, frindex, sl);
            files[frindex] -= sl;
        }
        sindex += 1;

        if files.iter().all(|&fl| fl == 0) {
            break;
        }
    }

    (0..compressed.len())
        .map(|i| compressed[i] * i)
        .sum()
}

fn extend(compressed: &mut Layout, n: usize, length: usize) {
    for _ in 0..length {
        compressed.push(n)
    }
}

fn checksum_whole(files: &mut Layout, spaces: &Layout) -> usize {
    let mut compressed: Vec<usize> = vec![];
    (0..compressed.len())
        .map(|i| compressed[i] * i)
        .sum()
}

#[test]
fn test_expand_compress_easy() {
    let (mut files, spaces) = parse("1");
    assert_eq!(checksum(&mut files, &spaces), 60);
}

#[test]
fn test_expand_compress() {
    let (mut files, spaces) = parse("2");
    assert_eq!(checksum(&mut files, &spaces), 1928);

    let (mut files, spaces) = parse("3");
    assert_eq!(checksum(&mut files, &spaces), 2132);

    let (mut files, spaces) = parse("4");
    assert_eq!(checksum(&mut files, &spaces), 275);
}
