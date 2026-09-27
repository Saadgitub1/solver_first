
fn main() {

    let string_question;

    if true {

        println!("________________________________________________________\n");
        println!("\n  Simple Math Solver Solver\n");
        println!("________________________________________________________\n");
        println!("  Works according to BODMAS");
        println!("  Powers '^' do not solve before * , / , bracket!");
        println!("  Powers '^' solve during solving + , -");
        println!("\n  Prefer Powers '^' to solve inside bracket '(num ^ num)'");
        println!("  If (nothing ^ num) then '^' will be ignored! e.g ^2 * 2 = 4");
        println!("________________________________________________________\n");

        println!("Enter question: \n");
        let getting_string = solver1::line_from_cmd();
        string_question = getting_string.trim().to_string();

    } else {
        string_question = String::from("");
    }

    println!("");
    println!("Question: {}" , string_question);

    let Some(tokenized) = solver1::parse(string_question) else {return ();};

    let ans = solver1::solve(&tokenized);
    println!("________________________________________________________\n");
    println!("Ans: {:?}" , ans);
    println!("________________________________________________________\n");

    //println!("{:#?}" , tokenized);
}