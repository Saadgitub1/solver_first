
fn main() {

    let string_question;

    if true {

        println!("Enter question: ");
        let getting_string = solver1::line_from_cmd();
        string_question = getting_string.trim().to_string();

    } else {
        string_question = String::from("");
    }

    println!("");
    println!("Question: {}" , string_question);

    let Some(tokenized) = solver1::parse(string_question) else {return ();};

    let ans = solver1::solve(&tokenized);
    println!("");
    println!("Ans: {:?}" , ans);

    //println!("{:#?}" , tokenized);
}