use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");

    let filename = "data/input.txt";
    let expansion = 1000000;
    let mut result = Vec::new();
    result = read_lines(filename);

    let mut points_vec:Vec<(i128, i128)> = Vec::new();
    let mut exp_points_vec:Vec<(i128, i128)> = Vec::new();
    let mut y = 0;
    let mut x = 0;
    
    for line in result {
        x = 0;
        for char in line.chars() {
            if char == '#' {
                points_vec.push((x, y));
                
            }
            x += 1;
        }
        y += 1;
    }

    let mut notexp_x = HashMap::new();
    let mut notexp_y = HashMap::new();

    for p in &points_vec {
        notexp_x.insert(p.0, 1);
        notexp_y.insert(p.1, 1);
    }

    for p in points_vec {
        let mut exp_x = 0;
        let mut exp_y = 0;
        for x in 0..p.0 {
            if !notexp_x.contains_key(&x) {
                exp_x += expansion - 1;
            }
        }

        for y in 0..p.1 {
            if !notexp_y.contains_key(&y) {
                exp_y += expansion - 1;
            }
        }
        exp_points_vec.push((p.0 + exp_x, p.1 + exp_y));
    }

    let mut sum = 0;
    for p1 in &exp_points_vec {
        for p2 in &exp_points_vec {
            sum += distance(*p1, *p2);
        }
    }

    let half_sum = sum as f64 / 2.0;

    println!("sum: {half_sum}")
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn distance(a: (i128, i128), b:(i128, i128)) -> i128 {
   let x = a.0;
   let y = a.1;

   let xx = b.0;
   let yy = b.1;

   return (xx - x).abs() + (yy - y).abs();
}