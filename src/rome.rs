use std::collections::HashMap;
use std::fmt;
use std::num::ParseFloatError;

#[derive(Clone)]
pub enum Oexp {
    Boolean(bool),
    Symbol(String),
    Number(f64),
    List(Vec<Oexp>),
    Function(fn(&[Oexp]) -> Result<Oexp, RomeError>),
}

impl fmt::Display for Oexp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Oexp::Boolean(b) => b.to_string(),
            Oexp::Symbol(s) => s.clone(),
            Oexp::Number(n) => n.to_string(),
            Oexp::List(list) => {
                let xs: Vec<String> = list.iter()
                    .map(|x| x.to_string()).collect();
                format!("({})", xs.join(" , "))
            },
            Oexp::Function(_) => "Function: {}".to_string(),
        };

        write!(f, "{}", str)
    }
}


#[derive(Debug)]
pub enum RomeError {
    ReaderError(String),
    OperatorError(String),
    ModelingError(String),
    EffectorError(String),
}

/* Traditionally we call this an Environment or Env 
 but I have a hunch that 'Model' is a better name
 Let's go with Model for now.
*/
#[derive(Clone)]
pub struct Model {
    store: HashMap<String, Oexp>,
}

pub fn tokenise(input: String) -> Vec<String> {
    input
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

pub fn parse<'a>(tokens: &'a [String]) -> Result<(Oexp, &'a [String]), RomeError> {
    let (token, rest) = tokens.split_first()
        .ok_or(RomeError::ReaderError("Could not parse token".to_string()))?;
    match &token[..] {
        "(" => read_seq(rest),
        ")" => Err(RomeError::ReaderError(
                "unexpectedly encountered a closing parens".to_string())),
        _ =>  Ok((parse_atom(token), rest)),
    }
}


fn read_seq<'a>(tokens: &'a [String]) -> 
                Result<(Oexp, &'a [String]), RomeError> {
    let mut res: Vec<Oexp> = vec![]; // result
    let mut rem = tokens; // remaining
    loop {
        let (next_token, rest) = rem.split_first()
            .ok_or(RomeError::ReaderError(
                    "could not find closing parens".to_string()))?;
        if next_token == ")" {
            return Ok((Oexp::List(res), rest))
        }
        let (exp, new_rem) = parse(&rem)?;
        res.push(exp);
        rem = new_rem;
    }
}

fn parse_atom(token: &str) -> Oexp {
    match token.as_ref() {
        "true" => Oexp::Boolean(true),
        "false" => Oexp::Boolean(false),
        _ => {
            let maybe_float: Result<f64, ParseFloatError> = token.parse();
            match maybe_float {
                Ok(v) => Oexp::Number(v),
                Err(_) => Oexp::Symbol(token.to_string().clone()) // else parse as symbol
            }
        }
    }
}

pub fn new_core_model() -> Model {
    let mut store: HashMap<String, Oexp> = HashMap::new();
    store.insert(
        "+".to_string(),
        Oexp::Function(
            |args: &[Oexp]| -> Result<Oexp, RomeError> {
                let sum = parse_list_of_floats(args)?.iter()
                    .fold(0.0, |sum, a| sum + a);
                Ok(Oexp::Number(sum))
            }
            )
        );
    Model {store}
}


fn parse_list_of_floats(args: &[Oexp]) -> Result<Vec<f64>, RomeError> {
    args
        .iter()
        .map(|x| parse_single_float(x))
        .collect()
}

fn parse_single_float(exp: &Oexp) -> Result<f64, RomeError> {
    match exp {
        Oexp::Number(num) => Ok(*num),
        _ => Err(RomeError::ReaderError("expected a number".to_string())),
    }
}

// The eval function 
pub fn eval(exp: &Oexp, env: &mut Model) -> Result<Oexp, RomeError> {
    match exp {
        Oexp::Symbol(k) => env.store.get(k)
            .ok_or(RomeError::OperatorError(format!("Unexpected symbok k='{}'", k)))
            .map(|x| x.clone()),
        Oexp::Number(_a) => Ok(exp.clone()),
        Oexp::Boolean(_b) => Ok(exp.clone()),
        Oexp::List(list) => {
            let foo = list.last() // the last form must be a known function
                .ok_or(RomeError::OperatorError("Did not expect an empty list here".to_string()))?;
            let len = list.len();
            let arg_forms = &list[0..(len - 1)];
            let func_eval = eval(foo, env)?;
            match func_eval {
                Oexp::Function(f) => {
                    let args_eval = arg_forms.iter()
                        .map(|x| eval(x, env))
                        .collect::<Result<Vec<Oexp>, RomeError>>();
                    f(&args_eval?)
                },
                _ => Err(RomeError::OperatorError("The last form must be a function".to_string())),

            }
        },
        Oexp::Function(_) => Err(RomeError::OperatorError("I don't know this function".to_string())),
    }
}
