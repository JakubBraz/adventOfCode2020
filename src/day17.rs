use std::cmp::max;
use std::time::{Instant, Duration};

pub fn part1() {
    let s = input();
    let steps = 6;
    let field = parse_input(&s, steps, 3);

    let final_field = (0..steps).into_iter().fold(field, |f1, f2| step(&f1));
    let result = final_field.points.iter().fold(0, |v1, v2| if *v2 { v1 + 1 } else { v1 });
    println!("{}", result);
}

pub fn part2() {
    let time = Instant::now();
    let s = input();
    let steps = 6;
    let field = parse_input(&s, steps, 4);

    let final_field = (0..steps).into_iter().fold(field, |f1, f2| step(&f1));
    let result = final_field.points.iter().fold(0, |v1, v2| if *v2 { v1 + 1 } else { v1 });
    println!("{}", result);
    println!("Duration {:?}", time.elapsed());
}

struct FieldNd {
    dimension: usize,
    size_dimension: usize,
    points: Vec<bool>,
}

fn step(field: &FieldNd) -> FieldNd {
    let mut new_field = create_field(field.size_dimension, field.dimension);
    let tt = Instant::now();
    let xx = &get_all_cooridnates(field.size_dimension, field.dimension);
    // println!("len {}; dur: {:?}", xx.len(), tt.elapsed());
    let mut d1 = Duration::default();
    let mut d2 = Duration::default();
    let t = Instant::now();
    for coords in xx {
        let mut t1 = Instant::now();
        let n = count_neighbours(field, coords);
        d1 += t1.elapsed();
        t1 = Instant::now();
        if get_value(field, &coords.into_iter().map(|&i| i as i32).collect()) {
            if n == 2 || n == 3 { new_field.points[to_index(field.size_dimension, coords)] = true }
        } else {
            if n == 3 { new_field.points[to_index(field.size_dimension, coords)] = true }
        }
        d2 += t1.elapsed();
    }
    // println!("TIMINGI {:?} {:?} {:?}", t.elapsed(), d1, d2);
    new_field
}


fn count_neighbours(field: &FieldNd, point: &Vec<usize>) -> u32 {
    let ttt = Instant::now();
    let res = generate_neighbours_coords(field.dimension).iter()
        .map(|x| {
            let tt = Instant::now();
            let r = x.iter().zip(point).map(|v| (v.0 + (*v.1 as i32))).collect();
            // println!("ile map? {:?}", tt.elapsed());
            r
        })
        .filter(|v| {
            let tt = Instant::now();
            let r = get_value(field, v);
            // println!("ile filter {:?}", tt.elapsed());
            r
        }).count() as u32;
    // println!("cala funkcja {:?} dla {:?}", ttt.elapsed(), point);
    res
}

fn generate_neighbours_coords(dim: usize) -> Vec<Vec<i32>> {
    let tt = Instant::now();
    let res: Vec<Vec<i32>> = get_all_cooridnates(3, dim).iter()
        .map(|v| v.iter().map(|&vv| vv as i32 - 1).collect())
        .filter(|p: &Vec<i32>| !p.iter().all(|&i| i == 0))
        .collect();
    // println!("jak dlugo generuje coords sasiadow? {:?}", tt.elapsed());
    res
}

fn get_all_cooridnates(size_dimension: usize, dimension: usize) -> Vec<Vec<usize>> {
    (0..size_dimension.pow(dimension as u32)).into_iter().map(|i| {
        from_index(i, size_dimension, dimension)
    }).collect()
}

fn neighbours_count(dim: usize) -> usize {
    ((3 as u32).pow(dim as u32) - 1) as usize
}

fn create_field(size: usize, dimension: usize) -> FieldNd {
    FieldNd { dimension: dimension, size_dimension: size, points: vec![false; size.pow(dimension as u32)] }
}

fn get_value_index(field: &FieldNd, ind: usize) -> bool {
    match field.points.get(ind) {
        Some(v) => *v,
        None => false
    }
}

fn get_value(field: &FieldNd, point: &Vec<i32>) -> bool {
    if point.iter().any(|&v| v < 0) { return false; }
    let ind: usize = to_index(field.size_dimension, &point.iter().map(|&x| x as usize).collect());
    get_value_index(field, ind)
}

fn set_values(field: &FieldNd, vals: &Vec<(bool, Vec<usize>)>) -> FieldNd {
    let mut new_field = create_field(field.size_dimension, field.dimension);
    for (v, dims) in vals {
        let ind: usize = to_index(new_field.size_dimension, &dims);
        new_field.points[ind] = *v;
    }
    new_field
}

fn from_index(ind: usize, dim_size: usize, dimensions: usize) -> Vec<usize> {
    let mut v: Vec<usize> = vec![];
    let mut curr = ind;
    for _i in 0..dimensions {
        v.push(curr % dim_size);
        curr = curr / dim_size;
    }
    v
}

fn to_index(dim_size: usize, point: &Vec<usize>) -> usize {
    let r: usize = point.iter().enumerate()
        .fold((0, 0), |v1, v2| (v2.0, v1.1 + v2.1 * dim_size.pow(v2.0 as u32))).1;
    r
}

fn parse_input(s: &str, step_number: usize, dimensions: usize) -> FieldNd {
    let inp_size: usize = max(s.chars().take_while(|&c| c != '\n').count(), s.split('\n').count());
    let dimension_size: usize = inp_size + 2 * step_number;
    let vals: Vec<(bool, Vec<usize>)> = s.chars()
        .filter(|&c| c == '.' || c == '#')
        .enumerate()
        .map(|(ind, c)| {
            let mut v = vec![ind % inp_size + step_number, ind / inp_size + step_number];
            for i in 0..(dimensions - 2) {
                v.push(dimension_size / 2);
            }
            (c == '#', v)
        })
        .collect();
    let field = create_field(dimension_size, dimensions);
    set_values(&field, &vals)
}

fn input() -> String {
//     ".#.
// ..#
// ###".to_string()
    ".##.####
.#.....#
#.###.##
#####.##
#...##.#
#######.
##.#####
.##...#.".to_string()
}
