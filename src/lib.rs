
use std::collections::HashMap;
use std::io;
use parser::{lifo , it_is_bracket , ToTrack , Tokens , parsing , Operators};
use to_solve::{Sides , solve_bracket};

mod parser;
mod to_solve;

pub fn line_from_cmd() -> String {
    let mut line = String::new();

    io::stdin().read_line(&mut line).expect("Failed to read line");

    line
}

pub fn parse(question: String) -> Option<Vec<Tokens>> {

    if question.len() == 0 {
        println!("________________________________________________________\n");
        eprintln!("ERROR: Question Empty!");
        println!("________________________________________________________\n");
        return None;
    }

    let question_len_minus_one:usize = question.len() - 1;

    let mut stack : Vec<char> = Vec::new();
    let mut tokenized: Vec<Tokens> = Vec::new();
    let mut tracking = ToTrack::new();

    for (index , character) in question.chars().enumerate() {
        tracking.last = if question_len_minus_one == index {true} else {false};
        tracking.bracket_ended = false;

        if it_is_bracket(&character) {
            match lifo(character , index , &mut stack) {
                Ok(boolean) => {
                    if boolean {
                        tracking.bracket_ended = boolean;
                    }
                },
                Err(error_message) => {
                    println!("________________________________________________________\n");
                    println!("ERROR: 'Breacket Error' {}" , error_message);
                    println!("________________________________________________________\n");
                    return None;
                },
            }
        }

        if let Err(message) = parsing(&character , &index , &mut tokenized , &mut tracking) {
            println!("________________________________________________________\n");
            eprintln!("ERROR: {}" , message);
            println!("________________________________________________________\n");
            return None;
        }
    }

    if stack.len() != 0 {
        println!("________________________________________________________\n");
        println!("'ERROR: 'Breacket Error' {}' is missing" , stack.pop().expect("Problem in displaying bracket from stack.pop()"));
        println!("________________________________________________________\n");
        return None;
    }

    //println!("{:#?}" , tokenized);
    //println!("Passed from parse");

    return Some(tokenized);
    //lifo::going_into_string(&question);
}

pub fn solve(tokenized: &Vec<Tokens>) -> Tokens {

    let mut sides = Sides::new();

    sides.make_lhs_rhs(tokenized);
    let ans = solve_bracket(&tokenized);
    return ans;

    //println!("{:#?}" , values);
    //println!("{:#?}" , sides);
}