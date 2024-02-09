use std::time::Instant;

use itertools::Itertools;
use turing_machine::{run, Direction, State, StateType, Transition};

fn main() {
    let empty_symbol = '_';

    let q_st = State::new("Q sākums", StateType::None);
    let q_a = State::new("Q a", StateType::None);
    let q_b = State::new("Q b", StateType::None);
    let q_ab = State::new("Q ab", StateType::None);
    let q_acc = State::new("Q akcept", StateType::Accepting);

    //q st transitions
    let q_st_transitions = Transition::arr_to_hashmap(&[
        ('*', Transition::new(q_st.clone(), '*', Direction::Forwards)),
        ('a', Transition::new(q_a.clone(), '*', Direction::Forwards)),
        ('b', Transition::new(q_b.clone(), '*', Direction::Forwards)),
        (empty_symbol, Transition::new(q_acc, empty_symbol, Direction::Forwards)),
    ]);

    q_st.borrow_mut().transitions = q_st_transitions;

    let q_a_transitions = Transition::arr_to_hashmap(&[
        ('*', Transition::new(q_a.clone(), '*', Direction::Forwards)),
        ('a', Transition::new(q_a.clone(), 'a', Direction::Forwards)), 
        ('b', Transition::new(q_ab.clone(), '*', Direction::Backwards)),
    ]);

    q_a.borrow_mut().transitions = q_a_transitions;

    let q_b_transitions = Transition::arr_to_hashmap(&[
        ('*', Transition::new(q_b.clone(), '*', Direction::Forwards)),
        ('b', Transition::new(q_b.clone(), 'b', Direction::Forwards)), 
        ('a', Transition::new(q_ab.clone(), '*', Direction::Backwards)),
    ]);

    q_b.borrow_mut().transitions = q_b_transitions;

    let q_ab_transitions = Transition::arr_to_hashmap(&[
        ('*', Transition::new(q_ab.clone(), '*', Direction::Backwards)),
        ('a', Transition::new(q_ab.clone(), 'a', Direction::Backwards)), 
        ('b', Transition::new(q_ab.clone(), 'b', Direction::Backwards)),
        (empty_symbol, Transition::new(q_st.clone(), empty_symbol, Direction::Forwards)),
    ]);

    q_ab.borrow_mut().transitions = q_ab_transitions;

    // match run("abba", q_st.clone(), empty_symbol) {
    //     Ok(val) => println!("Passes: {val}"),
    //     Err(err) => println!("{err}")
    // };
    // match run("abbaba", q_st.clone(), empty_symbol) {
    //     Ok(val) => println!("Passes: {val}"),
    //     Err(err) => println!("{err}")
    // };
    // match run("abbaab", q_st.clone(), empty_symbol) {
    //     Ok(val) => println!("Passes: {val}"),
    //     Err(err) => println!("{err}")
    // };
    // match run("abbaa", q_st.clone(), empty_symbol) {
    //     Ok(val) => println!("Passes: {val}"),
    //     Err(err) => println!("{err}")
    // };
    // match run("a", q_st.clone(), empty_symbol) {
    //     Ok(val) => println!("Passes: {val}"),
    //     Err(err) => println!("{err}")
    // };
    // match run("", q_st.clone(), empty_symbol) {
    //     Ok(val) => println!("Passes: {val}"),
    //     Err(err) => println!("{err}")
    // };
    let now = Instant::now();
    let mut variations = variations_up_to_length(&['a', 'b'], 18);

    let gen = Instant::now();

    println!("Generating took {} seconds", now.elapsed().as_secs());

    for variation in variations.iter_mut() {
        let original = variation.clone();
        match run(variation, q_st.clone(), empty_symbol) {
            Ok(val) => {
                if val != equal_a_and_b(&original) {
                    println!("Failed: {:?}; got: {val}", original);
                }
            }
            Err(err) => println!("{err}")  
        }
    }
    println!("Machine took {} seconds", gen.elapsed().as_secs());
    //print!("{:?}", variations);
}

fn equal_a_and_b(chars: &[char]) -> bool {
    let mut a_minus_b_count = 0;
    for char in chars.iter() {
        if *char == 'b' {
            a_minus_b_count -= 1;
        } else if *char == 'a' {
            a_minus_b_count += 1;
        }
    }
    return a_minus_b_count == 0;
}

fn variations_up_to_length(items: &[char], n: usize) -> Vec<Vec<char>> {
    let mut all: Vec<Vec<char>> = vec![];
    all.push(vec![]);
    for i in 0..=n {
        let mut x = std::iter::repeat(items)
            .take(i)
            .multi_cartesian_product()
            .map(|first| {
                first
                    .into_iter()
                    .map(|&second| {
                        second
                    })
                    .collect_vec()
            })
            .collect_vec();
        all.append(&mut x);
    }
    
    return all;
}   