
use std::collections::HashMap;
use std::io;
use parser::{lifo , it_is_bracket , ToTrack , Tokens , parsing};

mod parser;

pub fn line_from_cmd() -> String {
    let mut line = String::new();

    io::stdin().read_line(&mut line).expect("Failed to read line");

    line
}

pub fn parse(question: String) {

    if question.len() == 0 {
        eprintln!("Empty question");
        return;
    }

    let question_len_minus_one:usize = question.len() - 1;

    let mut stack : Vec<char> = Vec::new();
    let mut tokenized: Vec<Tokens> = Vec::new();
    let mut tracking = ToTrack::new();

    for (index , character) in question.chars().enumerate() {
        tracking.last = if question_len_minus_one == index {true} else {false};
        tracking.bracket_ended = false;

        match lifo(character , index , &mut stack) {
            Ok(boolean) => {
                if boolean {
                    tracking.bracket_ended = boolean;
                }
            },
            Err(error_message) => {
                println!("{}" , error_message);
                return();
            },
        }

        parsing(&character , &index , &mut tokenized , &mut tracking);
    }

    if stack.len() != 0 {
        println!("'{}' is missing" , stack.pop().expect("Problem in displaying bracket from stack.pop()"));
        return;
    }

    println!("Passed");
    //lifo::going_into_string(&question);
}