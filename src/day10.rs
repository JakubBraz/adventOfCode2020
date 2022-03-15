use std::collections::{HashSet, HashMap, VecDeque};

pub fn part1() {
    let i: HashSet<u64> = parse_input();
    let mut v: Vec<u64> = i.into_iter().collect();
    v.push(0);
    v.sort();
    v.push(v.last().unwrap() + 3);
    let diffs = get_differences(&v);
    let result = count_differences(&diffs);
    println!("{:?} {}", result, result.get(&1).unwrap() * result.get(&3).unwrap());
}

pub fn part2() {
    let mut i: Vec<u64> = parse_input().iter().cloned().collect();
    i.sort();
    let mut i: VecDeque<u64> = i.into_iter().collect();
    i.push_back(i.back().unwrap().clone() + 3);
    i.push_front(0);
    let memory: &mut HashMap<(VecDeque<u64>, u64), u64> = &mut HashMap::new();
    let result = find_possibilities(memory, &i, 0);
    println!("{:?}", result);
}

fn find_possibilities(mem: &mut HashMap<(VecDeque<u64>, u64), u64>, nums: &VecDeque<u64>, current_num: u64) -> u64 {
    if mem.contains_key(&(nums.clone(), current_num)) { return mem.get(&(nums.clone(), current_num)).unwrap().clone(); }
    if nums.len() == 1 { return 1; }
    let v1: u64 = if nums.contains(&(current_num + 1)) { find_possibilities(mem, &remove(&nums, current_num), current_num + 1) } else { 0 };
    let v2: u64 = if nums.contains(&(current_num + 2)) { find_possibilities(mem, &remove(&nums, current_num), current_num + 2) } else { 0 };
    let v3: u64 = if nums.contains(&(current_num + 3)) { find_possibilities(mem, &remove(&nums, current_num), current_num + 3) } else { 0 };
    let r = v1 + v2 + v3;
    mem.insert((nums.clone(), current_num), r);
    r
}

fn remove(set: &VecDeque<u64>, val: u64) -> VecDeque<u64> {
    // set.iter().filter(|&&x| x>val).cloned().collect()
    set.iter().skip_while(|&&x| x <= val).cloned().collect()
}

fn count_differences(diffs: &Vec<u64>) -> HashMap<u64, usize> {
    // let iter = diffs.iter().cloned();
    let s: HashSet<u64> = diffs.iter().cloned().collect();
    s.iter().map(|&i| (i, diffs.iter().filter(|&&e| e == i).count())).collect()
}

fn get_differences(nums: &Vec<u64>) -> Vec<u64> {
    let iter1 = nums.iter().cloned();
    let iter2 = nums.iter().cloned();
    iter1.skip(1).zip(iter2).map(|(x1, x2)| x1 - x2).collect()
}

fn parse_input() -> HashSet<u64> {
    input().split('\n').map(|line| line.parse().unwrap()).collect()
}

fn input() -> String {
    // "1".to_string()
    // "1\n2".to_string()
//     return "16
// 10
// 15
// 5
// 1
// 11
// 7
// 19
// 6
// 12
// 4".to_string();
//     "28
// 33
// 18
// 42
// 31
// 14
// 46
// 20
// 48
// 47
// 24
// 23
// 49
// 45
// 19
// 38
// 39
// 11
// 1
// 32
// 25
// 35
// 8
// 17
// 7
// 9
// 4
// 2
// 34
// 10
// 3".to_string()
    "47
61
131
15
98
123
32
6
137
111
25
28
107
20
99
36
2
97
88
124
138
75
112
52
122
78
46
110
41
64
63
16
93
104
105
91
27
45
119
14
1
65
62
118
37
79
77
19
71
35
130
69
5
44
9
48
125
136
103
140
53
126
106
55
129
139
87
68
21
85
76
31
113
12
100
24
96
82
13
70
72
86
26
117
58
132
114
40
54
133
51
92".to_string()
}
