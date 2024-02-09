use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone)]
pub enum Direction{
    Backwards, Forwards
}

#[derive(PartialEq)]
pub enum StateType{
    Accepting, Rejecting, None
}

pub struct State<'a> {
    pub name: &'a str,
    pub transitions: RefCell<HashMap<char, Transition<'a>>>,
    pub state_type: StateType
}

impl<'a> State<'a> {
    pub fn start(
        &self,
        chars: &mut Vec<char>,
        empty_symbol: char
    ) -> Result<bool, String> {
        let current_index = 1;
        return self.run(chars, current_index, empty_symbol);
    }

    fn run(
        &self,
        chars: &mut Vec<char>,
        current_index: usize,
        empty_symbol: char,
    ) -> Result<bool, String> {
        // if state is accepting or rejecting, instantly stop and return corresponding result
        if self.state_type == StateType::Accepting {
            return Ok(true);
        }
        else if self.state_type == StateType::Rejecting {
            return Ok(false);
        }

        let current_char = chars.get(current_index)
            .ok_or(
                String::from("Index out of bounds!")
            )?;

        let transition = self.transitions.borrow();
        let transition = transition.get(current_char);

        // no transition for current symbol
        if transition.is_none() {
            // current symbol is empty symbol, return rejected
            return if *current_char == empty_symbol {
                Ok(false)
            }
            // current symbol is a real symbol, return error
            else {
                Err(String::from("Unknown input symbol!"))
            }
        }
        let transition = transition.unwrap(); // should be Some because None is checked.

        chars[current_index] = transition.output_symbol;

        let next_index = match transition.direction {
            Direction::Backwards => current_index - 1,
            Direction::Forwards => current_index + 1
        };

        return transition.state_to.borrow().run(chars, next_index, empty_symbol);
    }

    pub fn new(
        name: &'a str,
        state_type: StateType
    ) -> Rc<RefCell<State<'a>>> {
        return Rc::new(
            RefCell::new(
                State {
                    name: name,
                    state_type: state_type,
                    transitions: RefCell::new(HashMap::new())
                }
            )
        );
    }
}

#[derive(Clone)]
pub struct Transition<'a> {
    // pub state_from: &'a State<'a>, // no need, since transition is a part of a state
    // pub input_symbol: &'a char, // no need for this as already in hashmap
    pub state_to: Rc<RefCell<State<'a>>>,
    pub output_symbol: char,
    pub direction: Direction
}

impl<'a> Transition<'a> {
    pub fn new(
        state_to: Rc<RefCell<State<'a>>>,
        output_symbol: char,
        direction: Direction
    ) -> Transition<'a> {
        return Transition {
            state_to: state_to,
            output_symbol: output_symbol,
            direction: direction
        }
    }

    pub fn arr_to_hashmap (
        arr: &[(char, Transition<'a>)]
    ) -> RefCell<HashMap<char, Transition<'a>>> {
        return RefCell::new(arr.into_iter().cloned().collect());
    }
}

pub fn run(
    //input: &str,
    input: &mut Vec<char>,
    start_state: Rc<RefCell<State>>,
    empty_symbol: char
) -> Result<bool, String> {
    // let mut chars: Vec<char> = input.chars().collect();
    input.insert(0, empty_symbol);
    input.push(empty_symbol);

    return start_state.borrow().start(input, empty_symbol);
}