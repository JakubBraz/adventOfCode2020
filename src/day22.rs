use std::iter::once;
use std::collections::{LinkedList, HashSet};

pub fn part1() {
    let inp = input();
    let decks = parse_input(&inp);
    let memo = &mut HashSet::new();
    let winner = play(&decks.0, &decks.1, memo, false);
    let result: usize = (if winner.0.is_empty() { winner.1 } else { winner.0 })
        .iter().rev().enumerate().map(|(i, v)| (i + 1) * *v as usize).sum();
    println!("{:?}", result);
    let memo = &mut HashSet::new();
    let winner = play(&decks.0, &decks.1, memo, true);
    let result: usize = (if winner.0.is_empty() { winner.1 } else { winner.0 })
        .iter().rev().enumerate().map(|(i, v)| (i + 1) * *v as usize).sum();
    println!("{:?}", result);
}

fn play(deck1: &Vec<u32>, deck2: &Vec<u32>, memo: &mut HashSet<(Vec<u32>, Vec<u32>)>, recurs: bool) -> (Vec<u32>, Vec<u32>) {
    let mut d1 = deck1.clone();
    let mut d2 = deck2.clone();

    while !d1.is_empty() && !d2.is_empty() {
        if memo.contains(&(d1.clone(), d2.clone())) {
            return (d1.iter().chain(d2.iter()).cloned().collect(), vec![]);
        } else {
            memo.insert((d1.clone(), d2.clone()));
        }

        let card1: u32 = d1[0];
        let card2: u32 = d2[0];

        let win1: bool = if recurs && (card1 as usize) <= d1.len() - 1 && (card2 as usize) <= d2.len() - 1 {
            let new_memo = &mut HashSet::new();
            let (d1_rec, _d2_rec) =
                play(&d1.iter().skip(1).take(card1 as usize).cloned().collect(),
                     &d2.iter().skip(1).take(card2 as usize).cloned().collect(), new_memo, recurs);
            !d1_rec.is_empty()
        } else {
            card1 > card2
        };

        if win1 {
            d1 = d1.iter().skip(1).chain(&[card1, card2]).cloned().collect();
            d2 = d2.iter().skip(1).cloned().collect();
        } else {
            d1 = d1.iter().skip(1).cloned().collect();
            d2 = d2.iter().skip(1).chain(&[card2, card1]).cloned().collect();
        }
    }

    (d1.clone(), d2.clone())
}

// fn play(deck1: &Vec<u32>, deck2: &Vec<u32>, memo: &mut HashSet<(Vec<u32>, Vec<u32>)>, recurs: bool) -> (Vec<u32>, Vec<u32>) {
//     println!("decks {:?} {:?} memo {:?}", deck1.len(), deck2.len(), memo.len());
//     // println!("xxx");
//     if memo.contains(&(deck1.clone(), deck2.clone())) {
//         // println!("contains! {:?} {:?}", deck1, deck2);
//         return (deck1.iter().chain(deck2.iter()).cloned().collect(), vec![]);
//     } else {
//         // println!("nie memo!");
//         memo.insert( (deck1.clone(), deck2.clone()) );
//     }
//     if deck1.is_empty() || deck2.is_empty() { return (deck1.clone(), deck2.clone()); }
//     let mut deck1_iter = deck1.iter();
//     let mut deck2_iter = deck2.iter();
//     let card1: u32 = *deck1_iter.next().unwrap();
//     let card2: u32 = *deck2_iter.next().unwrap();
//
//     if recurs && card1 as usize <= deck1_iter.len() && card2 as usize <= deck2_iter.len() {
//         let (d1, d2): (Vec<u32>, Vec<u32>) = (deck1_iter.take(card1 as usize).cloned().collect(),
//                                               deck2_iter.take(card2 as usize).cloned().collect());
//         let empty_set = &mut HashSet::new();
//         let decks: (Vec<u32>, Vec<u32>) = play(&d1, &d2, empty_set, recurs);
//         let (d1, d2): (Vec<u32>, Vec<u32>) = if decks.1.is_empty() {
//             (deck1.iter().skip(1).chain(&[card1, card2]).cloned().collect(),
//                  deck2.iter().skip(1).cloned().collect())
//         } else {
//             (deck1.iter().skip(1).cloned().collect(),
//                  deck2.iter().skip(1).chain(&[card2, card1]).cloned().collect())
//         };
//         play(&d1, &d2, memo, recurs)
//     } else {
//         match card1 > card2 {
//             true => play(&deck1_iter.chain(&[card1, card2]).cloned().collect(),
//                          &deck2_iter.cloned().collect(), memo, recurs),
//             false => play(&deck1_iter.cloned().collect(),
//                           &deck2_iter.chain(&[card2, card1]).cloned().collect(), memo, recurs)
//         }
//     }
// }

fn parse_input(inp: &str) -> (Vec<u32>, Vec<u32>) {
    let mut decks = inp.split("\n\n").map(|deck| {
        deck.split('\n').skip(1).map(|n| n.parse().unwrap()).collect()
    });
    (decks.next().unwrap(), decks.next().unwrap())
}

fn input() -> String {
//     "Player 1:
// 9
// 2
// 6
// 3
// 1
//
// Player 2:
// 5
// 8
// 4
// 7
// 10".to_string()
    "Player 1:
23
32
46
47
27
35
1
16
37
50
15
11
14
31
4
38
21
39
26
22
3
2
8
45
19

Player 2:
13
20
12
28
9
10
30
25
18
36
48
41
29
24
49
33
44
40
6
34
7
43
42
17
5".to_string()
//     "Player 1:
// 43
// 19
//
// Player 2:
// 2
// 29
// 14".to_string()
}
