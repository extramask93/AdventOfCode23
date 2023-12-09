use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};
fn isStar(c: u8) -> bool {
    c == b'*'
}
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
    let mut stars: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let mut nearby_stars: HashSet<(usize, usize)> = HashSet::new();
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
            nearby_stars.iter().for_each(|(xx, yy)| {
                if stars.contains_key(&(*xx, *yy)) {
                    stars.get_mut(&(*xx, *yy)).unwrap().push(number);
                } else {
                    stars.insert((*xx, *yy), vec![number]);
                }
            });
        }
        nearby_stars.clear();
        number = 0;
        is_surrounded = false;
        for (c, y) in line.iter().skip(1).take(line.len() - 2).zip(1..line.len()) {
            if isDigit(*c) {
                number *= 10;
                number += (*c - b'0') as u32;
                for (mx, my) in movements() {
                    let xx = (x as i32 - mx) as usize;
                    let yy = (y as i32 - my) as usize;
                    if isStar(matrix[xx][yy]) {
                        nearby_stars.insert((xx, yy));
                    }
                    if !isDot(matrix[xx][yy]) && !isDigit(matrix[xx][yy]) {
                        is_surrounded = true;
                    }
                }
            } else {
                if is_surrounded == true {
                    total += number;
                    nearby_stars.iter().for_each(|(xx, yy)| {
                        if stars.contains_key(&(*xx, *yy)) {
                            stars.get_mut(&(*xx, *yy)).unwrap().push(number);
                        } else {
                            stars.insert((*xx, *yy), vec![number]);
                        }
                    });
                }
                number = 0;
                is_surrounded = false;
                nearby_stars.clear();
            }
        }
    }
    if is_surrounded == true {
        nearby_stars.iter().for_each(|(xx, yy)| {
            if stars.contains_key(&(*xx, *yy)) {
                stars.get_mut(&(*xx, *yy)).unwrap().push(number);
            } else {
                stars.insert((*xx, *yy), vec![number]);
            }
        });
        total += number;
    }
    println!("{}", total);
    stars.iter().for_each(|(key,val)| {
        print!("({},{}) = ",key.0,key.1);
        val.iter().for_each(|v|{print!("{},",v)});
        println!("");
    });
    println!("{}", stars.iter().filter(|(key,val)| {
        val.len() == 2
    }).fold(0, |acc, (key,val)| {
        acc + val.iter().fold(1, |sum, cv| {sum*cv})
    }));
}

mod test {}
