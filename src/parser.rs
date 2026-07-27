
use crate::HashMap;

pub struct ToTrack {
    pub depth: usize,
    pub prev: Tokens,
    pub bracket_ended: bool,
    pub last: bool,
    pub word: String,
    pub num: String,
    pub can_come: CanCome,
}

impl ToTrack {
    pub fn new() -> ToTrack {
        ToTrack {
            depth: 0usize,
            prev: Tokens::None,
            bracket_ended: false,
            last: false,
            word: String::new(),
            num: String::new(),
            can_come: CanCome::new_all_true(),
        }
    }
}

#[derive(Clone)]
enum Operators {
    None,
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
}

impl Operators {
    fn is_it_operator(character: &char) -> bool {
        let operators = ['+' , '-' , '*' , '/' , '='];
        operators.contains(character)
    }
    fn which_operator(character: &char) -> Tokens {
        use Operators::*;

        let mut which_operator = HashMap::new();
        which_operator.insert('+' , Add);
        which_operator.insert('-' , Subtract);
        which_operator.insert('*' , Multiply);
        which_operator.insert('/' , Divide);
        which_operator.insert('=' , Equal);

        let operator = which_operator
            .get(character).
            expect("Failed to Operator from  hash map")
            .clone();

        Tokens::Oper(operator)
    }
}

pub enum Tokens {
    None,
    Space,
    Str(String),
    Num(f32),
    Oper(Operators),
    Bracket {bracket: char , content: Vec<Tokens>},
}

impl Tokens {}

#[derive(Debug)]
pub struct CanCome {
    pub string: bool,
    pub num: bool,
    pub bracket: bool,
}

impl CanCome {
    fn new_all_true() -> CanCome {
        CanCome {
            string: true,
            num: true,
            bracket: true,
        }
    }
    fn can_it_come(self) {}
}

pub fn parsing(character: &char , index: &usize , tokenized: &mut Vec<Tokens> , tracking: &mut ToTrack) {
    use Tokens::*;

    let mut can_come = CanCome::new_all_true();

    let character_type: Tokens = None;
    if !tracking.bracket_ended {
        if it_is_bracket(character) {
        } else {
            if character.is_whitespace() {}
            if character.is_alphabetic() {
                tracking.word.push(*character);
            }
            if character.is_numeric() {
                tracking.num.push(*character);
            }
            if Operators::is_it_operator(character) {}
        }
    }
}

pub fn lifo(character: char , index: usize , stack: &mut Vec<char>) -> Result<bool , String> {

    let mut bracket_pairs : HashMap<char , char> = HashMap::new();
    bracket_pairs.insert('[' , ']');
    bracket_pairs.insert('{' , '}');
    bracket_pairs.insert('(' , ')');

    if bracket_pairs.contains_key(&character) {
        stack.push(*bracket_pairs.get(&character).unwrap());
        return Ok(false);
    }

    if stack.len() != 0 {
        let last_on_stack = stack.pop().expect("Error in getting last val from stack");

        if character == last_on_stack {
            return Ok(true);
        } else {
            return Err(format!("at index '{index}' there is '{character}' it should have been '{last_on_stack}' "));
        }
    }

    if stack.len() == 0 {
        return Err(format!("at index '{index}' before '{character}' , there is no start bracket"));
    }

    Ok(false)
}

pub fn it_is_bracket(character: &char) -> bool {
    let brackets = ['[' , ']' , '{' , '}' , '(' , ')'];

    for brackets_char in brackets {
        if *character == brackets_char {
            return true;
        }
    }

    false
}