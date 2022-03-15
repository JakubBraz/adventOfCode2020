use std::collections::HashSet;
use std::time::Instant;
use fnv::FnvHashSet;
use std::hash::BuildHasherDefault;

pub fn main() {
    let str_input = input();
    let dirs = parse_input(&str_input);
    let blacks = travel(&dirs);
    println!("{:?}", blacks.iter().count());

    println!("{:?}", steps(&blacks, 100));
}

#[derive(Debug, Clone)]
enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

fn steps(active: &FnvHashSet<(i32, i32)>, n: usize) -> usize {
    let time = Instant::now();
    let mut blacks: FnvHashSet<(i32, i32)> = active.iter().cloned().collect();
    let all_d = [Direction::E, Direction::SE, Direction::SW, Direction::W, Direction::NW, Direction::NE];
    for i in 0..n {
        blacks = step(&blacks, &all_d);
    }
    println!("duration {:?}", time.elapsed());
    blacks.len()
}

fn step(active: &FnvHashSet<(i32, i32)>, all_d: &[Direction]) -> FnvHashSet<(i32, i32)> {
    let mut new_black: FnvHashSet<&(i32, i32)> = FnvHashSet::default();
    let mut inactives: FnvHashSet<(i32, i32)> = FnvHashSet::default();

    for black in active {
        let neighbours = get_neighbours(black, all_d);
        let black_count = neighbours.iter().filter(|&n| active.contains(n)).count();
        if black_count == 1 || black_count == 2 {
            new_black.insert(black);
        }
        // this is super slow :o
        // inactives = inactives.union(&neighbours.difference(active).cloned().collect()).cloned().collect();
        inactives.extend(neighbours.iter().filter(|&n| !active.contains(n)))
    }

    for white in &inactives {
        if get_neighbours(white, all_d).iter().filter(|&n| active.contains(n))
            .count() == 2 {
            new_black.insert(white);
        }
    }

    new_black.iter().map(|x| **x).collect()
}

fn get_neighbours(pos: &(i32, i32), all_d: &[Direction]) -> FnvHashSet<(i32, i32)> {
    all_d.iter()
        .map(|d| tile_position(pos, d))
        .collect()
}

fn travel(tiles: &Vec<Vec<Direction>>) -> FnvHashSet<(i32, i32)> {
    let mut blacks = FnvHashSet::default();
    let points = tiles.iter().map(|t| find_tile(t));
    for p in points {
        match blacks.contains(&p) {
            true => { blacks.remove(&p); }
            false => { blacks.insert(p); }
        }
    }
    blacks
}

fn tile_position(tile: &(i32, i32), dir: &Direction) -> (i32, i32) {
    let change = position_change(dir);
    (tile.0 + change.0, tile.1 + change.1)
}

fn find_tile(tile: &[Direction]) -> (i32, i32) {
    tile.iter().fold((0, 0), |x, y| {
        let d = position_change(y);
        (x.0 + d.0, x.1 + d.1)
    })
}

fn position_change(d: &Direction) -> (i32, i32) {
    match d {
        Direction::E => (1, 0),
        Direction::SE => (0, -1),
        Direction::SW => (-1, -1),
        Direction::W => (-1, 0),
        Direction::NW => (0, 1),
        Direction::NE => (1, 1)
    }
}

fn parse_input(s: &str) -> Vec<Vec<Direction>> {
    s.split('\n').map(|line| {
        let mut chrs = line.chars();
        let mut dir_vec: Vec<Direction> = vec![];
        loop {
            let c = chrs.next();
            match c {
                None => break,
                Some(x) => match x {
                    'e' => dir_vec.push(Direction::E),
                    'w' => dir_vec.push(Direction::W),
                    's' => match chrs.next().unwrap() {
                        'e' => dir_vec.push(Direction::SE),
                        _ => dir_vec.push(Direction::SW)
                    }
                    _ => match chrs.next().unwrap() {
                        'e' => dir_vec.push(Direction::NE),
                        _ => dir_vec.push(Direction::NW)
                    }
                }
            }
        }
        dir_vec
    }).collect()
}

fn input() -> String {
//     "sesenwnenenewseeswwswswwnenewsewsw
// neeenesenwnwwswnenewnwwsewnenwseswesw
// seswneswswsenwwnwse
// nwnwneseeswswnenewneswwnewseswneseene
// swweswneswnenwsewnwneneseenw
// eesenwseswswnenwswnwnwsewwnwsene
// sewnenenenesenwsewnenwwwse
// wenwwweseeeweswwwnwwe
// wsweesenenewnwwnwsenewsenwwsesesenwne
// neeswseenwwswnwswswnw
// nenwswwsewswnenenewsenwsenwnesesenew
// enewnwewneswsewnwswenweswnenwsenwsw
// sweneswneswneneenwnewenewwneswswnese
// swwesenesewenwneswnwwneseswwne
// enesenwswwswneneswsenwnewswseenwsese
// wnwnesenesenenwwnenwsewesewsesesew
// nenewswnwewswnenesenwnesewesw
// eneswnwswnwsenenwnwnwwseeswneewsenese
// neswnwewnwnwseenwseesewsenwsweewe
// wseweeenwnesenwwwswnew".to_string()
    "wenwwsenwwwwnwwnwwwnwsewseewe
nenenwnenenenwnwnenenwneneseseswnenwwne
newewwwwwwswwwww
esenwwwswwnwnwswnwnwnwewnwsenwswwnwe
esesewseeneswnenwneeewnwneswsenwnee
neseswwswwseseseseseseswnwesesesesesese
sewwswsweswwwswswnwenwswswenesww
wenenesenenewnenenenwenweneneenewse
swswsenwneeseswnwswseseswswswneswsesesesw
wswswswsweswseswnwswswnwswswnwseswswswsee
nesweeeneeenwneswneswewneneenw
swseswswswswswswswswswnwswswswsw
eneneenwneeeeeeneeeese
newenwsenewnesenesenwnenenesenwnwnwne
nwswnenwswneneswenenwneswseeeneneneneee
nenenenwsesenwnwwnwnwsesenwseneswnenenw
eeeseseseeeeeewnwsweseneesesene
wesenwswwwwnewwnwenwnwswwswnww
esenweeeeneeswewenwneweseeee
swswnenenenenenenwneneswneswneneneenwe
neswseenwesenwneswnesewneswnewenwne
nwweswseseswewneeenweeseneeesese
wnwnwwnwnwswswnwswwneeewnwnwsenwnwnw
nenesenewnwneeenene
eseeeewneeeseswnwnwwwseswnewese
neswnenwnwneswenwenwswsewnwneenwwnw
sewswnwswsweswswwwweswwswneswwsw
seseseneseseswseswsese
eswswwnwewwnwswnwswneneseswe
nwswnwswsenwsesweseseseseswseseswswswsww
neneenwswnwwesesenese
wswswswwwwnwswswswweseswneswswwsw
eesewswsweseswsesweseswswswwswnwsww
neswnenwswnesenwnwswwenwswnenwenwnwnwne
swswneseneswwneswswswswwwwswnwnesesee
nwnwnenwneneesenwnwwseswwnwnenwnwnwnenw
esewseseseeswswsweesewnwwsesenwsew
nwsewswnwnwnwenwnwnwwnenwswnwnwnwnwnw
senewswnwnwwnwnwnweneseseneneswnwneesw
swseswnwseswnwseswneswseswswswse
nesenwnwnwnesewwseenenwswnesesenenewe
eseseweeseseneseeseeseeewnwse
wsewwneswneneswwswsewswwenwsewsww
ewswwwwwnwwsewnewwwwwwwe
swwweewwwwwewwewwwwswww
nwnenwwnenwnenenwnenwnwnwneswnenwnesenese
eseseseeseweseeeeenweseswseewse
nwswseswswswnesweswnwnwseswsw
seseseseseseseseneseseswesesese
swneswwnesenwwweswsewswsw
nwneneneenwnenenewnwnenene
swnwenwswwswwwswnwsweswswwswwsee
nwnwnwwnwnwnwnwswnwnwnwesenwnwsenwsenwne
nwneneesenweswswswwewswsewwswswse
senwneeneesweneeeewnweeswsenenew
newnwwnewswwwswneswewewsesenww
swnwnwnwnwnewswnwnesenweneweseeswnw
eswwwnwseswwwnewenewwwswwww
nwnwnwwnwneneneswsewneesenwneswnwewenw
neswswswwswswwswswswwseswwswwenwsw
wwwwwwsewnwwnwsenewwnw
nwneswswnweswnweswswswwsewneseswsesesw
wnwsweneswsenwnenwseswwnenewnwneseswse
newewneswswswnwwswwswseswswwswsesw
neneneswnenwnenwneeseseewsweneswnwnw
nwnewneneenwseeswesenewneneneswenwnw
nenwnenwnenenwnenenwwnwsenwsenwnesewnee
nwnenenewenenenwnenenenenesenwswnenwnese
nesewswwwwsenwnenwwnwwsenewwww
enwnwnwnwnwwnwnwnwnw
swseseneseneswenewsewwsewnesesesenwswse
sweneneeneeswneeeeeneenee
sesenenwsesenwseseseseseesenwswswswsene
seswseewswsesweswseswewswnw
swneenwneeeeeenewneeswneneneene
eeeweneneeeneeeneswnesenewwnenw
swnwseenwnenwnwnwnwnenenwnwwnenwnwnwnw
wnwnwneswenwnwenwnwswnwwnwnwnwnwswnw
swswswswnwseseeneseseseseseseswnwseswse
neeweneswesweeeeseenenweeseeenw
wnwnwnwneenwswnwnwnwenenweswnwwnwnwnw
senwwswnwsenewewenenwewsenwwseesw
ewswwswnwwwwnewwwenw
enenwnenwnenwnwsenewnwneeswnenenenenwnw
seseeseeseswsewnwwseseseseeseeeneese
nwneseswnwsenesewnwnewsesweswsw
nwswsweeswswnwswwenwwswnweswswnwswse
neewseeseseeseeseseseenwee
senwnwwwnwewnwwnwnwnwnwnwnwnw
eeewneenwesenwseseweeeeseee
swnesenwsewseseswnwswwnwnesw
wnwwwnwnwnwnwswnwnweenwnwnwwwenwswnw
nwnwsenwnwnesenwnwnwnwnenwnwnenwswwnwnw
nwewswswwwsewwewnwnenwenwnwnwnwwnw
wnesweneseswnwnesesewseswseswwswse
enesewneenenweseswnesenwesewwneene
eneenwnenenenewswsese
wsewnewswnwseneswswwswnwwweswsesw
eeswneeseeeeeeneeeenwnwwenee
eswsenweeeseeseewnwneeseeseeenw
sesewseswewseeseneeseseneesenwsesese
wswnewnwnwswnenwnwswenwnwsenewswenw
newnesenwnwwnwnwwsenenwnwnwsenenwnwnwe
esweweswsenwnenwnweneeeseeeee
nenwseneneneneneeneseenenenwnenene
esesenwnwswesewneseseesenwseseseese
swswwsweswswnesweswneswwsewswswse
enesweeeenewneswswenenenenwnenene
enwswswnwsewnenwnweneeeeeeeese
enwnweneswswnenenwnesweswnwse
seswseseewneweenweesenewseesesw
swswnenwnwwswswseenenwwneneeeneneneswne
sewwnwnwwwwsewwnwnwwnwewewnww
neseseseswneseeswseseseseseswswwswswse
seseswswswseswwseswsesesesesesene
swenweeneeenwneeeeeese
eneseswswswswewwseseswswnwwswnwneswnenw
wswseeeeneswneneenenenwenweeene
wwwwwnewwwnwsewnwww
nwnwswnwnwnwnwwswsenwnwnenwwewnenw
nwswwswnwwnwnwwewwneenwnwnwnwwwwse
nwnenewseswnwnwsenenenenwnenenenenenee
nwswsenwwsewsenenwnenwnwse
nenwnwnenesenwnwnwnwnenww
ewswnwswswswweswwwseswwswwnenwsw
neseswswneswseswswneswswswswwsweswsenwswsw
newnenwneseeneneswnenenenwnenwenwswnesw
swswswseseseseswswswseswne
nwswswswwwwneseeswsewwswswswswseswnew
swnwsewwwsewnwww
swswseseeswswnenwwswswswswneswwseswswswnw
enenwswneeeneneneneeweneneeseenenw
neswneswwnwenwnwnwswswneeseneewenw
neeenewswneeseeeeneeeeneeenesw
swsesewsesweesewnwnenwnwseswse
swswneswsewnewwwneneesww
nenwnesenesesenwnwnenwnenewnenwnewnenwne
wwsenwsenwenwnwnwwwnwswenwsewnew
neseseenwswswwenwneeeneswenwswweswnw
swswneneneseneseswnwnenwnwneenenwneewne
wnwswwwwweewwswwwsw
wneeneswswseswnwswswswsewswswnwswnesesw
nwenwwnwnwsewnwnwnesesenwnwswnwwneww
enenwseeesewweeswnewsesweesee
wwnewwwwwswww
swneseswswwneneeneswsewnenenwswewnese
nwnewnwnwsenwnwnwwwseeseseewswewnw
eeeweeeesenenene
nwnwwnwenwswnwnwenwnwnwnwsenenwswnwnwnw
wnwneewseenwnwnwwnwwweeswwswnw
sesesesenwsesesesesenwnwse
esweenewsweeneneenenewneeeee
swnweeeneeewsweeneeeseeeeseee
newsweswswnwwewnwwswneswsw
senwnwwnenenenenwnene
nenwneneneenenewseeeneneenewnewne
neswnwswsewwnwenenwwwwnwwsewsww
swnwsesesesesenweeeseseswwseneseeesese
nwnwnwnwnwwnwwnwnwwwwnwe
swswwneswswneswswswswswswseseswnenwswe
enenenenwseseseenwnenenwswnenwnenesenene
eneseeneeenwnwneseenwsewnwesenee
sewswswwswwswneewnwswwnwwswnwswsesenw
seseseseseseseseseneswsewe
swwswswswewswswswswswsw
eeeeneenweswseswee
nwsesesewseswseneeseseseseswsesenesesenw
swneneswswswsenenwenwswswseswneeseswswnw
nwwnwnewwswswwwwneseswwswnwswwsese
esenwneenweeneswswweneeeswseee
eeeeeeeeeneswesewseeneewsee
neswwwswwwnweswswwewwnw
nenenwnenenwseneneswnwewnwwneswnw
swseswwswswweneswwswnwwwsewenwne
swseesewswseseseeswsenw
nesewswewnewwnwwsenwnwwswwswsenw
eseneeseesenwswsesesewseeseseseneswe
weseswwnwseseseseesesenwenesesenwsese
ewsesenwneseneswswewswswwnwswnesesw
swswswswseswswswsenesesewsenw
nwnwnenesenenenwnwnwsenwnwnwnenwnwwsesw
enewswnenwnwnenwnenenweewenenwsw
neeswnenenwnenenwneenenwswnenenenenwne
nenenwneswnenwnwnwswenwnwnenenwnwnwswswe
wnwwwnenwnwwnenwwwswwwwnwwsewse
neeeneneneneesenenenenenwnenewwnese
nwenwesenwwnwnwnwnwnwnwnwswnwnwsenenenw
ewwwswwweswewswswswwwwewnww
swwsenwwwsenwwwswnwesenwwswenesww
eweswswswnwnwswwnwswneeseeseswwsww
nwnwnwenenwnwnwnwnwewnwnwnww
wneenwenewesesesee
nwswswswwsweseswnwswswwswswneenwswswe
eseseseseswseseswsenwseswneseseswnw
swseswswseswsweweswswswnwswswseswswswnw
swewswswswseseseseswswswswsesenwseeswnw
nenwnwnenwneneseswnwneswseneneswsw
swnweseswnwneseseenewsese
eweneswsenesewsesenweswseeenwew
wweswsenwnwswwswneneneswwswsewsesene
seeeseeseseseweseseseeswnwe
nwnwnwnesenwenwnwsenwwnwnwnwnwwnwwnw
sesewseeeeseseneeseseweesesesenwe
wnewswswnewswswswswswnwswsweswswww
neweeeeeeneneenweesesesw
enesenewsenwnewwwwnwwnwsenwwwswse
senwseesweseeenweenesweneseeesee
sesweseswseswwneswwswswsweswsenwswswsw
swneneeneswneneneneeenwewneseeew
sweswwnewswsweswswnwwwswswswwwwsew
nwseseseswneesesewseseseseseseesesese
seseswswnwnwwseeseswneseesesesesesewne
neseseswseseseswseneswseswseswwnwswsesesw
nwswwwwwnwwnwweswnwwwsewwnwnewe
eeeneeenewswnene
seseseseswnwesenwseseseesweesesesee
nwswwnewneseswwseneneseswnewwwswwsesw
eeenweeeeeesenenweewsweese
enwswnwswneneswnwnwnwnwnwnenweneneseswnw
senenwwneswseswwswneneneneswsewswnenene
nwnwwsesenewnwswsesewsesewseseeenee
swneseseeeweeesenesewewnwseseesesw
neeenwweeenesewnwwewseesesese
neneswswnenenenenenenenenenenenewneese
swenenenwnwnwseesesenwswsenwsewesweww
wenewswwwnwesewswwwenwnenww
neneneseeneneneneneswneenenwnewwnenene
sewwnwsenenwwwsewwwwne
sewsesenwseseseeeeeseewwesenwnee
seswnwnenwneseeseswnenewwesewnenenw
seeseswseseenenwenwneenewsweenww
wnwnwwwnwswwwwwsewnenw
seswswneswswswwswswneneswswswsw
swnwsewnwsewnwwnwnenwwnwnwnw
nwsewswswnwwswswswsweeswesesesenesw
swswnwseseswswswnwseswseswseseweswsenesese
wnwnwwnwswswneeweswseewwnweswswse
nwwwewnwwnenwnwsewnwwnewnwnwsww
seeswseswseswseswsesesenewswswsenww
swnenwswwneneenenesenwneeeneeenee
weseswsesesewsesesenwseneseseseseese
swneeneesewwseswswenwneeswseenwnw
wnwwnwwwwwnwwwwnwe
newwenwseswnewnwsesweneneesenwsew
seenewneneswnwnenewnenwneneeewsenwsw
nwewnwsenwnwwnwnew
neneneneswnenenwnenenenenenene
newsenewneeewnwwwwswseswwwnee
wswneswewnwswnwwswesewwneswswswww
wseeeneseneesesweeeswnweeseneee
seeswswswwsewswswseeswnesewswsweswse
sesenwnwnwwwswwnwnwswenwewsenwwnew
weseneeeeeeeeee
swwswseeswswsesenwnwseswswseesenwesw
eneneweeneeneeeneene
newwswwnewwwswnewsewwwswwwwse
wsenwnwnwnwenwenwnewsw
wwnwnwwwnwwewwnwnwneenwnweswww
eseeeeeneneeesewswenweewnenene
nenwneneswneneswnenwneneswnenwnenenwene
sesesesesewnewnesenwseseswsesesesesesesese
swneneswneseswneneneeswneeenwnewenwne
nwsenwnwsenwwnwnwsewnwnwsenwnwnwnwnwnenw
wnenwnwwswenwenwwnwwnwseswnwswnwwnw
nwnwnwwsewswnewnwwesenesenwnwnwnwnw
swswenesenwneenwwsenwnwnwnenwnwnwsew
nenenwswnenenwnenenwsenwnesenwnewnwwnenene
swwnewwswwwwwnewwww
eeeswswneneeeneneeeeswnenee
swseseswnesewseseswswseenwswseeswswse
wewwewwswwswwwwwnwswenwnwesww
nwnwwnwnenwsewswwwnewwnwwnwsenwww
eeneneswneesenesweneneenenewneenenw
enenwnwswnwnwseneswswnweeswnwnwsenwnwnwnw
neneneneneswneneneneneneeswnenesenwnenwnw
enweseeeseswseenweeswseeesesee
nwswseseswneswsewseseswneseswseseseew
swsenesewswswswnwwwnwwswswwswwwse
sweswnwswwsewseseseswswseswnwneswsw
neeweneesenenwnwswseeeeeenenewe
wnwnwwwnwweenwswwwww
neseswseeeseeewsenweneseese
neswswswswswswseseswswswswnwsweswswnwswsww
nwwnwewwwwwswwwwswnwnwwnwew
nwnwnenwnwnwnwenwneenwnenwnwnwsewnenwsw
nwnenwnenwnwwnwsenwnwwnwwwwnwsesenwnw
swswswseswsesewsewnenwseswswswseswswsenesw
seweweswswseswwnwseswseseenwenew
eenweeeeeeseesenw
wsewwwwsewnewwwesenwneseswneenew
seweseeeeeeeeeee
swsenwswneswswswsesesesewseseneswseswsew
swswneneneneenenenenenweneneneneneswnwene
seeseeneneswwswsenwenwseseneseswswnw
nenesesewnwnenwnwsenwnenenwnenenwnwnwwse
wswnwwneswnweseseswwe
wswsenwswswswweswnwswswwswswsweswnwe
swsesesenwsesenewsesenwseseswsesesenesese
nwnwwnweswnwsenwnwnwsewswnenwnwneese
swswseswwnwsesewesewneswneneseswnwnw
swsweseswsesenwwsenwseseswswseenwsesw
eeneeswenweeneesweeneeenwene
nwswwwwwneswswswseeneswwswseswswnwsw
nwswneseneswswneswneseseswswnesenw
nwnwwneneswneesweenenenewnwnenenwnw
nwwnwenwnwenwwnwnwnwnwnwnwwww
eeesweeeweeneeeenweeweseee
nwenwnwnwnwnwenwnwwnwewnwnwwnwswnwnw
nwneneenewenewseneswneneneneenwwneswne
wwnwwnesesewwwwnewwsewwnwww
swswswnwswseswswwswsweswneswseswswswe
sewwneseseswneseseseneswseswseseew
nwneneenenesweneneneenenenenweesw
sesesenwneewseseeenwnwneenwwswnwswsw
weseneseeneneneweeswewnesesewsw".to_string()
}
