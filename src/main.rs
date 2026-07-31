
fn main() {

    let string_question;

    if false {

        println!("Enter question: ");
        let getting_string = solver1::line_from_cmd();
        string_question = getting_string.trim().to_string();

    } else {
        string_question = String::from("x = 5 + b = 5");
    }

    println!("{}" , string_question);

    solver1::parse(string_question);
}