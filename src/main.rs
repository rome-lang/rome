pub mod rome;

use std::io;


fn read_eval(input: String, env: &mut rome::Model) -> Result<rome::Oexp, rome::RomeError> {
    let (parsed_exp, _) = rome::parse(&rome::tokenise(input))?;
    let evaluated_exp = rome::eval(&parsed_exp, env)?;
    
    Ok(evaluated_exp)
}

fn slurp_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    
    input
}

fn main() {
    let env = &mut rome::new_core_model();
    loop {
        println!("O. ");
        let input = slurp_input();
        match read_eval(input, env) {
            Ok(res) => println!("    : {}", res),
            Err(e) => match e {
                rome::RomeError::ReaderError(msg) => println!("    ~ {}", msg),
                rome::RomeError::OperatorError(msg) => println!("    ~ {}", msg),
                rome::RomeError::ModelingError(msg) => println!("    ~ {}", msg),
                rome::RomeError::EffectorError(msg) => println!("    ~ {}", msg),

            }
        }

    }
}
