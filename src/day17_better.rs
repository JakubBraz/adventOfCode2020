use std::cmp::max;
use std::time::{Instant, Duration};
use std::collections::{HashSet, HashMap};
use std::intrinsics::transmute;

pub fn part1() {
    let s = input();
    let steps = 6;
    let field = parse_input(&s, steps, 3);

    let final_field = (0..steps).into_iter().fold(field, |f1, f2| step(&f1));
    let result = final_field.points.iter().count();
    println!("{}", result);
}

pub fn part2() {
    let time = Instant::now();
    let s = input();
    let steps = 6;
    let field = parse_input(&s, steps, 5);

    let final_field = (0..steps).into_iter().fold(field, |f1, f2| step(&f1));
    let result = final_field.points.iter().count();
    println!("{}", result);
    println!("Duration {:?}", time.elapsed());
}

struct FieldNd {
    points: HashSet<Vec<usize>>
}

fn step(field: &FieldNd) -> FieldNd {
    let mut new_points: HashSet<Vec<usize>> = HashSet::new();
    let mut empty_counter: HashMap<Vec<usize>, u32> = HashMap::new();
    for coords in &field.points {
        let neighbour = generate_neighbours_coords(field.points.iter().next().unwrap().len(), &coords);
        let n = neighbour.iter().filter(|&x| field.points.contains(x)).count();
        if n == 2 || n == 3 { new_points.insert(coords.clone()); }
        let empties: HashSet<Vec<usize>> = neighbour.iter()
            .filter(|&x| !field.points.contains(x)).cloned().collect();
        for x in &empties {
            if empty_counter.contains_key(x) {
                let count = *empty_counter.get(x).unwrap();
                empty_counter.insert(x.clone(), count + 1);
            } else { empty_counter.insert(x.clone(), 1); }
        }
    }
    let to_be_alive: HashSet<Vec<usize>> = empty_counter.iter().filter(|&(k, v)| *v == 3).map(|(v, c)| v).cloned().collect();
    new_points.extend(to_be_alive);
    create_field(new_points)
}

fn generate_neighbours_coords(dim: usize, point: &Vec<usize>) -> HashSet<Vec<usize>> {
    get_all_cooridnates(3, dim).iter()
        .map(|v| v.iter().map(|&vv| vv as i32 - 1).collect())
        .filter(|p: &Vec<i32>| !p.iter().all(|&i| i == 0))
        .map(|x: Vec<i32>| x.iter().zip(point).map(|v| (v.0 + (*v.1 as i32)) as usize).collect())
        .collect()
}

fn get_all_cooridnates(size_dimension: usize, dimension: usize) -> Vec<Vec<usize>> {
    (0..size_dimension.pow(dimension as u32)).into_iter().map(|i| {
        from_index(i, size_dimension, dimension)
    }).collect()
}

fn create_field(vec: HashSet<Vec<usize>>) -> FieldNd {
    FieldNd { points: vec }
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

fn parse_input(s: &str, step_number: usize, dimensions: usize) -> FieldNd {
    let inp_size: usize = max(s.chars().take_while(|&c| c != '\n').count(), s.split('\n').count());
    let dimension_size: usize = inp_size + 2 * step_number;
    let vals: HashSet<Vec<usize>> = s.chars()
        .filter(|&c| c == '#' || c == '.')
        .enumerate()
        .filter(|c| c.1 == '#')
        .map(|(ind, c)| {
            let mut v = vec![ind % inp_size + step_number, ind / inp_size + step_number];
            for i in 0..(dimensions - 2) {
                v.push(dimension_size / 2);
            }
            v
        })
        .collect();
    create_field(vals)
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
