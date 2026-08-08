
fn main() {

    let string_question;

    if false {

        println!("Enter question: ");
        let getting_string = solver1::line_from_cmd();
        string_question = getting_string.trim().to_string();

    } else {
        string_question = String::from("0 + 1 + 4 * 3");
    }

    println!("Question: {}" , string_question);

    let Some(tokenized) = solver1::parse(string_question) else {return ();};

    solver1::solve(&tokenized);

    // println!("{:#?}" , tokenized);
}