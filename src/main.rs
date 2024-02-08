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

    match run("abba", q_st.clone(), empty_symbol) {
        Ok(val) => println!("Passes: {val}"),
        Err(err) => println!("{err}")
    };
    match run("abbaba", q_st.clone(), empty_symbol) {
        Ok(val) => println!("Passes: {val}"),
        Err(err) => println!("{err}")
    };
    match run("abbaab", q_st.clone(), empty_symbol) {
        Ok(val) => println!("Passes: {val}"),
        Err(err) => println!("{err}")
    };
    match run("abbaa", q_st.clone(), empty_symbol) {
        Ok(val) => println!("Passes: {val}"),
        Err(err) => println!("{err}")
    };
    match run("a", q_st.clone(), empty_symbol) {
        Ok(val) => println!("Passes: {val}"),
        Err(err) => println!("{err}")
    };
    match run("", q_st.clone(), empty_symbol) {
        Ok(val) => println!("Passes: {val}"),
        Err(err) => println!("{err}")
    };
}
