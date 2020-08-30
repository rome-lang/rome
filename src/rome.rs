use std::collections::HashMap;
use std::fmt;
use std::num::ParseFloatError;
use std::rc::Rc;

#[derive(Clone)]
pub enum Oexp {
    Boolean(bool),
    Symbol(String),
    Number(f64),
    List(Vec<Oexp>),
    Function(fn(&[Oexp]) -> Result<Oexp, RomeError>),
    FunctionDef(Lambda)
}

#[derive(Clone)]
pub struct Lambda {
    params_exp : Rc<Oexp>,
    body_exp: Rc<Oexp>,
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
            Oexp::FunctionDef(_) => "Function Definition: {}".to_string(),
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
    match exp { // 1
        Oexp::Symbol(k) => env.store.get(k)
            .ok_or(RomeError::OperatorError(format!("Unexpected symbok k='{}'", k)))
            .map(|x| x.clone()),
        Oexp::Number(_a) => Ok(exp.clone()),
        Oexp::Boolean(_b) => Ok(exp.clone()),
        Oexp::List(list) => {
            let last_form = list.last() // the last form must be a known function
                .ok_or(RomeError::OperatorError("Did not expect an empty list here".to_string()))?;
            let len = list.len();
            // Let's first check for built-in operators: `def`, `def.` `.` all are forms of define
            // and `?` as a way to check conditions in the form of 
            // `something if (condition is true) else somethingelse ?`
            let arg_forms = list.get(0..len-1).ok_or(RomeError::OperatorError("unable to get forms".to_string()))?;
            match eval_built_in_form(last_form, arg_forms, env) { // 2
                Some(res) => res,
                None => {
                    let arg_forms = &list[0..(len - 1)];
                    let func_eval = eval(last_form, env)?;
                    match func_eval { // 3
                        Oexp::Function(f) => {
                            let args_eval = arg_forms.iter()
                                .map(|x| eval(x, env))
                                .collect::<Result<Vec<Oexp>, RomeError>>();
                            f(&args_eval?)
                        },
                        _ => Err(RomeError::OperatorError("The last form must be a function".to_string())),

                    } // match 3
                }
            } // match 2
        },
        Oexp::Function(_) => Err(RomeError::OperatorError("I don't know this function".to_string())),
        Oexp::FunctionDef(_) => Err(RomeError::OperatorError("I didn't expect this function definition here".to_string())),
    } // match 1
} // end eval

fn eval_built_in_form(exp: &Oexp, arg_forms: &[Oexp], env: &mut Model) -> Option<Result<Oexp, RomeError>> {
    match exp {
        Oexp::Symbol(s) =>
            match s.as_ref() {
                "?" => Some(eval_query(arg_forms, env)),
                "." => Some(eval_define(arg_forms, env)),
                "fn" => Some(eval_function_def(arg_forms, env)),
                _ => None,
            },
            _ => None,
    }
}

fn eval_query(arg_forms: &[Oexp], env: &mut Model) -> Result<Oexp, RomeError> {
    let len = arg_forms.len();
    let subject = arg_forms.get(0).ok_or(
        RomeError::OperatorError("expected a subject as first form in conditional".to_string()))?;
    let verb = arg_forms.get(1).ok_or(
        RomeError::OperatorError("expected a verb as second form in conditional".to_string()))?;
    let object = arg_forms.get(2).ok_or(
        RomeError::OperatorError("expected an object as third form in conditional".to_string()))?;
    match verb {
        Oexp::Symbol(v) => 
            match v.as_ref() {
                ">" => eval_gt_query(subject, object, env), 
                "<" => unimplemented!(),
                "=" => unimplemented!(),
                ">=" => unimplemented!(),
                "=<" => unimplemented!(),
                "~=" => unimplemented!(),
                "if" => {
                    let rem_args = arg_forms.get(3..len).ok_or(
                        RomeError::OperatorError("expected an else/or branch to if".to_string()))?;
                    eval_if_query(subject, object, rem_args, env)
                },
                _ => unimplemented!(),
            },
        _ => unimplemented!(),
    }
}

fn eval_gt_query(subject: &Oexp, object: &Oexp, _env: &mut Model) -> Result<Oexp, RomeError> {
        match (subject, object) { 
        (Oexp::Number(a), Oexp::Number(b)) => Ok(Oexp::Boolean(a > b)),
        _ => Err(RomeError::OperatorError("Can compare only two numbers (as of now)".to_string())),
        }

}

fn eval_if_query(subject: &Oexp, object: &Oexp, rem_args: &[Oexp], env: &mut Model) -> Result<Oexp, RomeError> {
    let predicate = eval(object, env)?;
    match predicate {
        Oexp::Boolean(b) => {
            match b { // 1
                true => eval(subject, env),
                false => {
                    // handle else/or branch
                    let keyword = rem_args.get(0).ok_or(
                        RomeError::OperatorError("expected else/or branch for if condition".to_string()))?;
                    match keyword { // 2
                        Oexp::Symbol(k) => {
                        match k.as_ref() { // 3
                            "else" => {
                                let something_else = rem_args.get(1).ok_or(
                                    RomeError::OperatorError("...else what?...".to_string()))?;
                                eval(something_else, env)
                            },
                            "or" => {
                                unimplemented!();
                            },
                            _ => Err(RomeError::OperatorError("Expected else or or after if condition".to_string())),
                        } // match 3
                        },
                        _ => Err(RomeError::ReaderError("could not read this keyword".to_string())),
                    } // match 2 
                },
            } // match 1
        },
        _ => Err(RomeError::OperatorError("Unexpected test form".to_string()))
    }
}

fn eval_define(arg_forms: &[Oexp], env: &mut Model) -> Result<Oexp, RomeError> {
     let subject = arg_forms.get(0).ok_or(
        RomeError::OperatorError("expected a subject as first form in definition".to_string()))?;
     let name_str = match subject {
         Oexp::Symbol(s) => Ok(s.clone()),
         _ => Err(RomeError::OperatorError("Expected subject to be an oexp of type symbol".to_string())),
     }?;
    let verb = arg_forms.get(1).ok_or(
        RomeError::OperatorError("expected a verb as second form in definition".to_string()))?;
    let object = arg_forms.get(2).ok_or(
        RomeError::OperatorError("expected an object as third form in definition".to_string()))?;
    if arg_forms.len() > 3 {
        return Err(RomeError::OperatorError("A definition can have only subject, verb and object. I can't handle more...".to_string()))
    }
    match verb {
        Oexp::Symbol(v) => 
            match v.as_ref() {
                "=" => {
                    let object_eval = eval(object, env)?;
                    env.store.insert(name_str, object_eval);

                    Ok(subject.clone())
                },
                ">" => unimplemented!(), // assert that subject is greater than object
                "<" => unimplemented!(), // let it be known as fact that subject is less than object from now on.
                ">=" => unimplemented!(),
                "=<" => unimplemented!(),
                "~=" => unimplemented!(),
                _ => unimplemented!(),
            },
        _ => unimplemented!(),
    }
}


fn eval_function_def(arg_forms: &[Oexp], _env: &mut Model) -> Result<Oexp, RomeError> {
    let _len = arg_forms.len();
    let verb = arg_forms.get(1).ok_or(
        RomeError::OperatorError("expected a verb as second form in conditional".to_string()))?;
    match verb {
        Oexp::Symbol(v) => 
            match v.as_ref() {
                "=>" => {// anonymous-function definition
                    let params = arg_forms.get(0).ok_or(
                        RomeError::OperatorError(
                            "expected a list of parameters as first form in anonymous function def"
                            .to_string()))?;
                    let body = arg_forms.get(2).ok_or(
                        RomeError::OperatorError("expected function body as third form in function def".to_string()))?;

                    Ok(Oexp::FunctionDef(Lambda {
                        params_exp: Rc::new(params.clone()),
                        body_exp: Rc::new(body.clone()),
                    }))
                }, 
                "=" => unimplemented!(), // named-function definition
                _ => unimplemented!(),
            },
        _ => unimplemented!(),
    }
}
