
use crate::HashMap;

pub struct ToTrack {
    pub depth: usize,
    pub prev: Tokens,
    pub bracket_comes: bool,
    pub bracket_ended: bool,
    pub last: bool,
    pub word: String,
    pub num: String,
    pub oper: Operators,
    pub equal: bool,
    pub can_come: CanCome,
}

impl ToTrack {
    pub fn new() -> ToTrack {
        ToTrack {
            depth: 0usize,
            prev: Tokens::None,
            bracket_ended: false,
            bracket_comes: false,
            last: false,
            word: String::new(),
            num: String::new(),
            oper: Operators::None,
            equal: false,
            can_come: CanCome::new_all_true(),
        }
    }
    fn need_to_push(&mut self , tokenized: &mut Vec<Tokens>) -> Result<() , String> {
        if self.word.len() != 0 {
            let new_word = Tokens::Str(self.word.clone());
            match self.can_come.is_feild_true(0) {
                Ok(boolean) => {
                    if boolean {
                        self.prev = Tokens::Str(self.word.clone());
                        self.word.clear();
                        ToTrack::push_it(tokenized , &mut self.depth , 0usize , new_word);
                    } else {
                        return Err(format!("The word: '{:?}' cannot come after {:?}" , new_word , self.prev));
                    }
                },
                Err(err) => return Err(err),
            }
        } else if self.num.len() != 0 {
            let after_parse: f32 = self.num.parse().expect("Problem in parsing num");
            let new_num = Tokens::Num(after_parse);
            match self.can_come.is_feild_true(1) {
                Ok(boolean) => {
                    if boolean {
                        self.prev = Tokens::Num(after_parse);
                        self.num.clear();
                        ToTrack::push_it(tokenized , &mut self.depth , 0usize , new_num);
                    } else {
                        return Err(format!("The num: '{:?}' cannot come after {:?}" , new_num , self.prev));
                    }
                },
                Err(err) => return Err(err),
            }
        } else if !self.bracket_comes {
            match self.can_come.is_feild_true(3) {
                Ok(boolean) => {
                    if boolean {
                        let new_oper = self.oper.clone();
                        match new_oper {
                            Operators::None => return Err(format!("Operator cannot be none ")),
                            oper_type => {
                                self.oper = Operators::None;
                                self.prev = Tokens::Oper(oper_type.clone());
                                ToTrack::push_it(tokenized , &mut self.depth , 0usize , Tokens::Oper(oper_type));
                            }, 
                        }
                    } else {
                        return Err(format!("The oper: '{:?}' cannot come after {:?}" , self.oper , self.prev));
                    }
                },
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }

    fn push_it(tokenized: &mut Vec<Tokens> , depth: &mut usize , inner_depth: usize , to_push: Tokens) {
        *depth += 1;
        reaching_content_to_push(tokenized , depth , inner_depth , to_push);
        *depth -= 1;
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub enum Operators {
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
    fn which_operator(character: &char) -> Operators {
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

        operator
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub enum Tokens {
    None,
    Str(String),
    Num(f32),
    Oper(Operators),
    Bracket {bracket: char , content: Vec<Tokens>},
}

impl Tokens {
    fn new_bracket_type(character: &char) -> Result<Tokens , ()> {
        use Tokens::*;
        if it_is_bracket(character) {
            return Ok(Bracket{bracket: *character , content: Vec::new()}); 
        }
        Err(())
    }
}

#[derive(Debug)]
pub struct CanCome {
    pub string: bool,
    pub num: bool,
    pub bracket: bool,
    pub oper: bool,
}

impl CanCome {
    fn new_all_true() -> CanCome {
        CanCome {
            string: true,
            num: true,
            bracket: true,
            oper: true,
        }
    }

    fn make_these_true_false(&mut self , tuple: &(bool , bool , bool , bool)) {
        //&(string , num , bracket , oper);
        //(0,1,2,3)
        self.string = tuple.0;
        self.num = tuple.1;
        self.bracket = tuple.2;
        self.oper = tuple.3;
    }

    fn is_feild_true(&mut self , which: u8/* , present_oper: &Operators*/) -> Result<bool , String> {
        //&(string , num , bracket , oper);
        //(0,1,2,3)
        if which == 0 {
            if self.string {
                self.make_these_true_false(&(false , false , true , true));
               return Ok(true);
            }
            return Ok(false);
        }
        if which == 1 {
            if self.num {
                self.make_these_true_false(&(false , false , true , true));
                return Ok(true);
            }
            return Ok(false);
        }
        if which == 2 {
            if self.bracket {
                self.make_these_true_false(&(true , true , true , true));
                return Ok(true);
            }
            return Ok(false);
        }
        if which == 3 {
            if self.oper {
                self.make_these_true_false(&(true , true , true , false));
                return Ok(true);
            }
            return Ok(false);

            // if self.oper.1 {
            //     match *&self.oper.0 {
            //         Operators::None => return true,
            //         //(0: Add , 1: Subtract , 2: Multiply , 3 : Divide , 4 : Equal)
            //         Operators::Add => {
            //             return this_oper_can_come(present_oper , &(false , false , false , false , false));
            //         },
            //         Operators::Subtract => {
            //             return this_oper_can_come(present_oper , &(false , false , false , false , false));
            //         },
            //         Operators::Multiply => {
            //             return this_oper_can_come(present_oper , &(false , false , false , false , false));
            //         },
            //         Operators::Divide => {
            //             return this_oper_can_come(present_oper , &(false , false , false , false , false));
            //         },
            //         _ => return false,
            //     }
            // }
        
            // return false;
        }

        // fn this_oper_can_come(the_present_oper: &Operators , this: &(bool , bool , bool , bool ,bool)) -> bool {
        //     // this (0: Add , 1: Subtract , 2: Multiply , 3 : Divide , 4 : Equal)
        //     match *the_present_oper {
        //         Operators::Add => {
        //             return if_true_false(&this.0);
        //         },
        //         Operators::Subtract => {
        //             return if_true_false(&this.1);
        //         }
        //         Operators::Multiply => {
        //             return if_true_false(&this.2);
        //         }
        //         Operators::Divide => {
        //             return if_true_false(&this.3);
        //         }
        //         Operators::Equal => {
        //             return if_true_false(&this.4);
        //         }
        //         _ => return false,
        //     }

        //     fn if_true_false(boolean: &bool) -> bool {
        //         if *boolean {
        //             return *boolean;
        //         }
        //         return false;
        //     }
        //     //end of this_oper_can_come()
        // }

        Err(format!("Nothing in is_feild_true matched which:{}" , which))
        //end of is_feild_true()
    }
}

pub fn parsing(character: &char , index: &usize , tokenized: &mut Vec<Tokens> , tracking: &mut ToTrack) -> Result<() , String> {
    use Tokens::*;

    let mut character_type: Tokens;
    match Tokens::new_bracket_type(&character) {
        Ok(type_came) => {
            character_type = type_came;
            match tracking.can_come.is_feild_true(2) {
                Ok(boolean) => {
                    if boolean {
                        tracking.bracket_comes = true;
                        if let Err(err) = tracking.need_to_push(tokenized) {
                            return Err(err);
                        }
                        tracking.can_come.make_these_true_false(&(true,true,true,true));
                        tracking.bracket_comes = false;
                        if tracking.bracket_ended {
                            tracking.depth -= 1;
                            if let Tokens::Oper(operator) = &tracking.prev {
                                if let Operators::None = operator {
                                    return Ok(());
                                } else {
                                    return Err(format!("The oper: {:?} cannot come at last there must be something after it" , tracking.prev));
                                }
                            }
                            tracking.can_come.make_these_true_false(&(true , true , true , true));
                            return Ok(());
                        }
                        tracking.depth += 1;
                        tracking.prev = Tokens::Bracket{bracket: *character , content: Vec::new()};
                        reaching_content_to_push(tokenized , &mut tracking.depth , 0usize , Tokens::Bracket{bracket: *character , content: Vec::new()});
                    } else {
                        return Err(format!("The bracket: '{}' \ncannot come because can_come = {:#?}" , character , tracking.can_come));
                    }
                },
                Err(err) => return Err(err),
            }
        },
        Err(()) => {
            if character.is_alphabetic() {
                if tracking.num.len() != 0 {
                    return Err(format!("The characarter '{}' basically word cannot come after num: {}" , character , tracking.num));
                }
                tracking.word.push(*character);
            } else if character.is_numeric() {
                if tracking.word.len() != 0 {
                    return Err(format!("The characarter '{}' basically num cannot come after word: {}" , character , tracking.word));
                }
                tracking.num.push(*character);
            } else if character.is_whitespace() {
                if tracking.word.len() == 0 && tracking.num.len() == 0 {
                    if tracking.bracket_ended == true || tracking.last == true {
                        if let Tokens::Oper(operator) = &tracking.prev {
                            if let Operators::None = operator {
                                return Ok(());
                            } else {
                                return Err(format!("The oper: {:?} cannot come at last there must be something after it" , tracking.prev));
                            }
                        }
                    }
                    if let Operators::None = tracking.oper {
                        return Ok(());
                    }
                }
                if let Err(err) = tracking.need_to_push(tokenized) {
                    return Err(err);
                }

                return Ok(());
            } else if Operators::is_it_operator(&character) {
                tracking.oper = Operators::which_operator(&character);
                if let Operators::Equal = tracking.oper {
                    if tracking.equal {
                        return Err(format!("There cannot be more than one equal"));
                    }
                    tracking.equal = true;
                }
                if tracking.word.len() != 0 || tracking.num.len() != 0 {
                    if let Err(err) = tracking.need_to_push(tokenized) {
                        return Err(err);
                    }
                }
                if tracking.bracket_ended == true || tracking.last == true {
                    return Err(format!("The oper: {:?} cannot come at last there must be something after it" , tracking.oper));
                }
                if let Err(err) = tracking.need_to_push(tokenized) {
                    return Err(err);
                }
            }
        },
    }
    if tracking.last {
        if let Err(err) = tracking.need_to_push(tokenized) {
            return Err(err);
        }
        if let Tokens::Oper(operator) = &tracking.prev {
            if let Operators::None = operator {
                return Ok(());
            } else {
                return Err(format!("The oper: {:?} cannot come at last there must be something after it" , tracking.prev));
            }
        }
    }

    Ok(())
}

fn reaching_content_to_push(tokenized_or_content: &mut Vec<Tokens>, depth_to_reach: &mut usize , mut inner_depth: usize , thing_to_push: Tokens) -> () {
    //println!("{}" , depth_to_reach);
    inner_depth += 1;
    //println!("{} , {}" , inner_depth , depth_to_reach);

    if *depth_to_reach == inner_depth {
        //println!("pushed");
        tokenized_or_content.push(thing_to_push);
        return ();
    }

    if tokenized_or_content.len() == 0 {
        tokenized_or_content.push(thing_to_push);
        return ();
    }

    let last_index = tokenized_or_content.len() - 1;
    match &mut tokenized_or_content[last_index] {
        Tokens::Bracket{bracket , content} => {
            reaching_content_to_push(content , depth_to_reach , inner_depth , thing_to_push);
        },
        _ => (),
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