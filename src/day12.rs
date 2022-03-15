pub fn part1() {
    let inp = parse_input();
    let ship = Ship { orientation: Dir::E, position: (0, 0) };
    let ships = vec![&ship; inp.len()];
    let final_ship = inp.iter().skip(1).chain(vec![inp.last().unwrap()])
        .zip(ships)
        .fold((&inp[0], ship.clone()), |(n1, s1), (n2, s2)| (n2, step(&s1, n1)));
    println!("{:?}", final_ship);
    println!("{}", final_ship.1.position.1 + final_ship.1.position.0);
}

pub fn part2() {
    let inp = parse_input();
    let ship = Ship { orientation: Dir::E, position: (0, 0) };
    let ships = vec![&ship; inp.len()];
    let final_ship = inp.iter().skip(1).chain(vec![inp.last().unwrap()])
        .zip(ships)
        .fold((&inp[0], (ship.clone(), (10, 1))), |(n1, (s1, (w1, w2))), (n2, s2)| (n2, waypoint_step((&s1, (w1, w2)), n1)));
    println!("{:?}", final_ship);
    println!("{}", final_ship.1.0.position.1.abs() + final_ship.1.0.position.0.abs());
}

#[derive(Debug, Clone)]
enum Dir {
    N,
    S,
    E,
    W,
}

#[derive(Debug, Clone)]
enum Action {
    Dir(Dir),
    L,
    R,
    F,
}

#[derive(Debug)]
struct Navigation {
    action: Action,
    val: i32,
}

#[derive(Debug, Clone)]
struct Ship {
    orientation: Dir,
    position: (i32, i32),
}

fn step(current: &Ship, action: &Navigation) -> Ship {
    match &action.action {
        Action::Dir(d) => Ship { orientation: current.orientation.clone(), position: move_ship(&current, d, action.val) },
        Action::F => Ship { orientation: current.orientation.clone(), position: move_ship(&current, &current.orientation, action.val) },
        turn => Ship { orientation: turn_ship(current, turn, action.val), position: current.position }
    }
}

fn waypoint_step(ship: (&Ship, (i32, i32)), action: &Navigation) -> (Ship, (i32, i32)) {
    match &action.action {
        Action::Dir(d) => (ship.0.clone(), move_waypoint(ship.1, action.val, d)),
        Action::F => move_ship_to_waypoint((ship.0, (ship.1.0, ship.1.1)), action.val),
        turn => (ship.0.clone(), turn_waypoint(ship.1.0, ship.1.1, turn, action.val))
    }
}

fn turn_waypoint(p_x: i32, p_y: i32, dir: &Action, val: i32) -> (i32, i32) {
    match dir {
        Action::L => turn_waypoint_left(p_x, p_y, val),
        _ => turn_waypoint_right(p_x, p_y, val)
    }
}

fn turn_waypoint_left(p_x: i32, p_y: i32, val: i32) -> (i32, i32) {
    if val <= 0 { return (p_x, p_y); }
    turn_waypoint_left(-p_y, p_x, val - 90)
}

fn turn_waypoint_right(p_x: i32, p_y: i32, val: i32) -> (i32, i32) {
    if val <= 0 { return (p_x, p_y); }
    turn_waypoint_right(p_y, -p_x, val - 90)
}

fn move_ship_to_waypoint(ship: (&Ship, (i32, i32)), val: i32) -> (Ship, (i32, i32)) {
    if val <= 0 { return (ship.0.clone(), ship.1); }
    let s = (ship.0).clone();
    move_ship_to_waypoint((&Ship { orientation: s.orientation, position: (s.position.0 + ship.1.0, s.position.1 + ship.1.1) }, ship.1), val - 1)
}

fn move_waypoint(waypoint: (i32, i32), val: i32, dir: &Dir) -> (i32, i32) {
    match dir {
        Dir::N => (waypoint.0, waypoint.1 + val),
        Dir::E => (waypoint.0 + val, waypoint.1),
        Dir::S => (waypoint.0, waypoint.1 - val),
        _ => (waypoint.0 - val, waypoint.1)
    }
}

fn increase_orientation(dir: &Dir, val: i32) -> Dir {
    if val <= 0 { return dir.clone(); }
    increase_orientation(&match dir {
        Dir::N => Dir::E,
        Dir::E => Dir::S,
        Dir::S => Dir::W,
        _ => Dir::N
    }, val - 90)
}

fn decrese_orientation(dir: &Dir, val: i32) -> Dir {
    if val <= 0 { return dir.clone(); }
    decrese_orientation(&match dir {
        Dir::N => Dir::W,
        Dir::W => Dir::S,
        Dir::S => Dir::E,
        _ => Dir::N
    }, val - 90)
}

fn turn_ship(ship: &Ship, turn: &Action, val: i32) -> Dir {
    match turn {
        Action::L => decrese_orientation(&ship.orientation, val),
        _ => increase_orientation(&ship.orientation, val)
    }
}

fn move_ship(ship: &Ship, dir: &Dir, val: i32) -> (i32, i32) {
    match dir {
        Dir::N => (ship.position.0, ship.position.1 - val),
        Dir::S => (ship.position.0, ship.position.1 + val),
        Dir::W => (ship.position.0 - val, ship.position.1),
        _ => (ship.position.0 + val, ship.position.1)
    }
}

fn parse_input() -> Vec<Navigation> {
    input().split('\n').map(|line| Navigation {
        action: match line.chars().next().unwrap() {
            'N' => Action::Dir(Dir::N),
            'S' => Action::Dir(Dir::S),
            'W' => Action::Dir(Dir::W),
            'E' => Action::Dir(Dir::E),
            'L' => Action::L,
            'R' => Action::R,
            _ => Action::F
        },
        val: line.chars().skip(1).collect::<String>().parse().unwrap(),
    }).collect()
}

fn input() -> String {
//     "F10
// N3
// F7
// R90
// F11".to_string()
    "F47
W2
S5
R180
R90
N3
F44
W1
N3
F77
S5
L270
F39
N3
L90
F83
W4
R270
E2
F98
N3
R180
N3
F54
N1
W4
R90
N1
L90
S2
E4
N4
W2
R90
F42
W3
S4
L90
E5
F6
R180
N4
E5
R180
E3
N3
F27
L90
S4
L180
E4
F52
E2
N1
R90
E3
S4
F76
R270
W2
R90
S2
R90
F79
S1
L180
F81
E3
F79
L90
S3
L180
F52
S5
L90
N4
W2
F65
N2
W1
R90
F25
W4
L90
S2
R90
N1
F13
W1
N1
F71
N3
L90
W4
R90
F91
W5
N3
W3
S3
F58
W4
N5
W3
F42
S4
E5
N3
F14
L180
E5
L270
F55
N3
R90
R90
S4
F55
W2
N1
W5
R180
F8
E3
L270
N2
F12
N2
R90
W1
R90
W2
L90
S2
F75
L90
S4
E3
F82
L90
L90
F42
N4
E5
F67
R90
E3
F64
E4
R90
F42
S4
F85
W5
S5
R90
F35
R270
W5
F67
R90
S5
R180
S1
F13
N4
W5
S2
F31
L90
E2
F39
R90
F3
W4
N2
F14
E2
F80
L180
F52
N3
E2
F98
W2
F29
R180
E2
L90
W4
N3
W1
S2
W1
N5
F6
E5
E1
W2
R90
S3
F92
L90
E5
F55
L90
S3
R90
S2
L90
N1
E5
F50
L90
N4
F9
L90
N4
L90
R90
R180
E3
F57
L90
S5
R180
S3
E4
F41
W5
N4
W2
N2
R90
S2
W1
F83
R180
W1
R90
W1
F17
F20
S1
E5
F13
N5
F8
F81
E2
S4
F7
W2
F86
N2
L90
N5
L180
E2
R90
E3
S3
N4
W2
F64
L90
F81
L90
E4
F1
E3
L90
W5
L90
N3
F28
F3
F100
E5
N5
F32
R90
W1
R90
S3
W1
W5
N3
F27
R90
W2
R180
W3
W2
N2
E2
S1
R90
W3
F51
E5
N4
W3
S5
R90
F91
S3
W1
S4
R270
N5
W4
F94
R90
N4
L90
N4
R90
F35
E3
F6
S4
F98
E2
L180
W4
N5
F42
S3
W3
N1
R90
S5
E3
S3
F47
S1
F19
W5
R90
F17
R90
N4
R90
F57
E2
F73
W3
F52
F98
R90
N1
F88
N2
E4
S4
R90
E2
R90
N5
F75
L180
F61
E2
S4
N4
W1
N3
E2
N3
F44
E3
L180
N4
F16
E2
S1
L180
R90
W5
F65
S5
F31
E3
L90
N5
E4
S5
E4
S4
R90
F70
R90
W4
L90
N3
W1
L90
S3
L90
F91
L180
S3
R90
N5
L90
S5
W2
F18
E3
F19
N1
F70
R90
E3
S4
F46
N2
S3
W2
S4
F7
L90
E4
R90
F78
S1
F4
L90
W3
F78
E5
L270
F86
E3
F82
L90
F32
R90
E4
L90
E4
L90
E3
F63
N4
E4
L90
F70
R180
F30
R180
F40
N5
R90
W4
F16
L180
S1
W1
R180
F12
W3
L90
F93
S2
L270
F36
L90
W2
N2
F3
W2
L180
L90
F24
S1
W5
R90
E4
L180
E1
S1
L90
F94
L90
F55
N2
E5
F33
E3
L90
N2
L90
S2
R90
F67
W4
F79
E1
E5
F5
S5
R180
F5
E2
E5
N4
W5
N4
W5
E2
L90
F2
L90
N4
E3
N3
R90
F92
N5
F83
L90
F85
R90
W5
S2
L90
E1
F34
E3
L180
W3
R90
F29
W4
L90
F34
W1
S4
E2
S1
W2
W5
L90
E5
N4
R180
N2
W5
R90
F42
W3
N2
L90
F79
W2
F16
N5
E3
F52
F55
L90
F42
L90
W4
S2
E5
L90
S4
F34
N5
N1
L180
L90
E2
L90
W3
L90
F16
E2
F96
N3
E1
F34
R180
S2
F17
W1
L270
F7
W2
N1
F33
N4
F2
N5
R180
F10
W3
L90
S3
E2
S1
F85
N2
F1
R180
F10
N4
W3
S2
R180
N4
W3
S2
S4
L90
E5
N1
F34
S4
W2
W5
F62
S5
E5
S4
F100
L90
W2
F20
S2
E1
R180
F88
N5
F85
N2
R90
N1
E5
F83
R90
W1
R90
E1
F11
E3
F54
N5
L180
F54
R90
S2
E3
L90
E3
N5
R90
W1
S5
R270
F91
E3
F52
W1
F36
W1
N5
F53
E1
R180
N3
F12
L90
S5
F99
S1
R90
S4
R90
S1
W1
N2
L270
W5
F78
S2
R90
F37
W5
R90
E3
S2
E4
L90
S3
W4
F83
L180
S3
R90
F57
W1
S1
L180
W2
N1
R180
N1
L180
W3
S3
R180
E4
F77
N5
S3
W1
N4
F4
N5
F64
W1
R90
N2
W5
L90
N3
L90
F8
L90
F3
S5
F95
R90
W2
F15
L270
F49
R180
S3
F15
N5
L180
S2
F71
S5
F56
W1
F22
F90
E5
F68
N4
R180
N5
E4
F52
E5
L90
E3
F69
W4
S3
L90
N4
R90
F19".to_string()
}
