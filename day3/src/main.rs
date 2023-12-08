use std::io::BufRead;

fn isDigit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}
fn isDot(c: u8) -> bool {
    c == b'.'
}
fn movements() -> Vec<(i32, i32)> {
    vec![
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ]
}

fn main() {
    let mut fs = std::fs::File::open("data.txt").unwrap();
    let reader = std::io::BufReader::new(fs);

    //prepare data
    let mut matrix = Vec::<Vec<u8>>::new();
    for mut line in reader.lines().filter_map(|elem| elem.ok()) {
        line = line.trim().to_string();
        line.push('.');
        line.insert(0, '.');
        matrix.push(line.as_bytes().to_vec());
    }
    matrix.insert(0, vec![b'.'; matrix.first().unwrap().len()]);
    matrix.push(vec![b'.'; matrix.first().unwrap().len()]);
    //prepare data end
    let mut total = 0;
    let mut number: u32 = 0;
    let mut is_surrounded = false;
    for (line, x) in matrix
        .iter()
        .skip(1)
        .take(matrix.len() - 2)
        .zip(1..matrix.len())
    {
        if is_surrounded == true {
            total += number;
        }
        number = 0;
        is_surrounded = false;
        for (c, y) in line.iter().skip(1).take(line.len() - 2).zip(1..line.len()) {
            if isDigit(*c) {
                number *= 10;
                number += (*c - b'0') as u32;
                for (mx, my) in movements() {
                    let xx = (x as i32 - mx) as usize;
                    let yy = (y as i32 - my) as usize;
                    if !isDot(matrix[xx][yy]) && !isDigit(matrix[xx][yy]) {
                        is_surrounded = true;
                    }
                }
            } else {
                if is_surrounded == true {
                    total += number;
                }
                number = 0;
                is_surrounded = false;
            }
        }
    }
    if is_surrounded == true {
        total += number;
    }
    println!("{}", total);
}

mod test {}
