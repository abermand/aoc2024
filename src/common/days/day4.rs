use itertools::Itertools;

macro_rules! idx {
    ($x:expr, $y:expr, $w:expr) => {
        $x + ($y * $w)
    };
}

macro_rules! coord {
    ($i:expr, $w:expr) => {
        ($i % $w, $i / $w)
    };
}

fn is_xmas(
    map: &Vec<char>,
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    dx: i32,
    dy: i32,
) -> bool {
    // x will be out of bounds:
    let mx = x as i32 + 3 * dx;
    if mx < 0 || (mx >= width as i32) {
        return false;
    }
    // y will be out of bounds:
    let my = y as i32 + 3 * dy;
    if my < 0 || (my >= height as i32) {
        return false;
    }

    let expected: Vec<char> = "XMAS".chars().collect();

    for i in 0..expected.len() {
        let nx = x.saturating_add_signed(i as isize * dx as isize);
        let ny = y.saturating_add_signed(i as isize * dy as isize);
        // println!(
        //     "R({x},{y}) -> ({dx},{dy}) i={i} Checking at ({nx},{ny}) char {} expected {}",
        //     map[idx!(nx, ny, width)],
        //     expected[i]
        // );
        if map[idx!(nx, ny, width)] != expected[i] {
            return false;
        }
    }
    return true;
}

fn is_xxmas(map: &Vec<char>, width: usize, height: usize, x: usize, y: usize) -> bool {
    // MAS will be out of bounds:
    if x == 0 || x >= width - 1 || y == 0 || y >= height - 1 {
        return false;
    }

    let m1 = map.get(idx!(x - 1, y - 1, width));
    let s1 = map.get(idx!(x + 1, y + 1, width));
    match (m1, s1) {
        (Some('M'), Some('S')) | (Some('S'), Some('M')) => {
            let m2 = map.get(idx!(x - 1, y + 1, width));
            let s2 = map.get(idx!(x + 1, y - 1, width));
            match (m2, s2) {
                (Some('M'), Some('S')) | (Some('S'), Some('M')) => {
                    // all good now, we have a true 'X'
                    let display = [
                        [
                            map[idx!(x - 1, y - 1, width)],
                            map[idx!(x, y - 1, width)],
                            map[idx!(x + 1, y - 1, width)],
                        ],
                        [
                            map[idx!(x - 1, y, width)],
                            map[idx!(x, y, width)],
                            map[idx!(x + 1, y, width)],
                        ],
                        [
                            map[idx!(x - 1, y + 1, width)],
                            map[idx!(x, y + 1, width)],
                            map[idx!(x + 1, y + 1, width)],
                        ],
                    ];
                    println!("FOUND @({x},{y}): {:?}", display);
                    return true;
                }
                _ => return false,
            }
        }
        _ => return false,
    }
}

const DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn count_xmas(map: Vec<char>, width: usize, height: usize) -> u32 {
    let xs: Vec<(usize, &char)> = map.iter().enumerate().filter(|(_, c)| **c == 'X').collect();
    // println!("XS: {:?}", xs);
    let mut count = 0;

    for (index, _) in xs {
        let (x, y) = coord!(index, width);
        for (dx, dy) in DIRECTIONS {
            if is_xmas(&map, width, height, x, y, dx, dy) {
                count += 1;
            }
        }
    }

    return count;
}

fn count_xxmas(map: Vec<char>, width: usize, height: usize) -> u32 {
    let xs: Vec<(usize, &char)> = map.iter().enumerate().filter(|(_, c)| **c == 'A').collect();
    // println!("XS: {:?}", xs);
    let mut count = 0;

    for (index, _) in xs {
        let (x, y) = coord!(index, width);
        // println!("INSPECTING IDX {index} @({x},{y}): {:?}", &map.get(index));
        if is_xxmas(&map, width, height, x, y) {
            count += 1;
        }
    }

    return count;
}

fn input_to_vec(input: &str) -> (Vec<char>, usize, usize) {
    let mut lines = input.lines().peekable();
    let width = lines.peek().unwrap().len();

    let chars: Vec<char> = lines
        .map(|line| line.chars().filter(|c| !c.is_whitespace()))
        .flatten()
        .collect();
    // println!("CHARS: {:?}", chars);
    let height = chars.len() / width;
    // println!("W={width} H={height}");
    (chars, width, height)
}

pub fn part1(input: &str) -> anyhow::Result<()> {
    let (chars, width, height) = input_to_vec(input);
    let total = count_xmas(chars, width, height);

    println!("TOTAL: {total}");
    Ok(())
}
pub fn part2(input: &str) -> anyhow::Result<()> {
    let (chars, width, height) = input_to_vec(input);
    let total = count_xxmas(chars, width, height);

    println!("TOTAL: {total}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(0, 0, 5, 0)]
    #[case(0, 1, 5, 5)]
    #[case(1, 2, 5, 11)]
    #[case(4, 3, 5, 19)]
    fn idx(#[case] x: usize, #[case] y: usize, #[case] w: usize, #[case] expected: usize) {
        assert_eq!(idx!(x, y, w), expected);
    }

    #[rstest]
    #[case("...XMAS...")]
    #[case("...SAMX...")]
    #[case(
        "...X...
        ...M...
        ...A...
        ...S..."
    )]
    fn it_works(#[case] sample: &str) {
        let (chars, width, height) = input_to_vec(sample);
        println!("MAP: w={width}, h={height}");
        assert_eq!(count_xmas(chars, width, height), 1);
    }

    #[test]
    fn it_works_sample() {
        let sample = "MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX";
        let (chars, width, height) = input_to_vec(sample);
        println!("MAP: w={width}, h={height}");
        assert_eq!(count_xmas(chars, width, height), 18)
    }

    #[test]
    fn it_works_xxmas_sample() {
        let sample = ".M.S......
        ..A..MSMS.
        .M.S.MAA..
        ..A.ASMSM.
        .M.S.M....
        ..........
        S.S.S.S.S.
        .A.A.A.A..
        M.M.M.M.M.
        ..........";
        let (chars, width, height) = input_to_vec(sample);
        println!("MAP: w={width}, h={height}");
        assert_eq!(count_xxmas(chars, width, height), 9)
    }
    #[test]
    fn it_works_xxmas_sample2() {
        let sample = "MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX";
        let (chars, width, height) = input_to_vec(sample);
        println!("MAP: w={width}, h={height}");
        assert_eq!(count_xxmas(chars, width, height), 9)
    }
}
