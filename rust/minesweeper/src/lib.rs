pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut results = vec![];
    if minefield.is_empty() {
        return results;
    }

    let rows = minefield.len();
    let cols = minefield[0].len();

    for x in 0..rows {
        let mut out_row = vec![];
        for y in 0..cols {
            out_row.push(count_mines(x, y, minefield, rows, cols));
        }
        results.push(out_row.concat());
    }

    results
}

const DIRS: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, 1),
    (0, -1),
    (1, -1),
    (-1, 1),
    (1, 1),
    (-1, -1),
];

fn count_mines(x: usize, y: usize, minefield: &[&str], rows: usize, cols: usize) -> String {
    if minefield[x].as_bytes()[y] == ('*' as u8) {
        return format!("*");
    }

    let mut mines = 0;

    for (i, j) in DIRS.iter() {
        let x2: i32 = x as i32 + i;
        let y2: i32 = y as i32 + j;

        if x2 < 0 || x2 >= (rows as i32) || y2 < 0 || y2 >= (cols as i32) {
            continue;
        }

        if minefield[x2 as usize].as_bytes()[y2 as usize] == ('*' as u8) {
            mines += 1;
        }
    }

    if mines == 0 {
        format!(" ")
    } else {
        format!("{}", mines)
    }
}
