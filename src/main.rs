use std::io;
fn main() {
    let mut input = String::from("");
    println!("4-function calculator by Allen MacFarland\nPlease enter an equation without parentheses or exponents to calculate: ");
    io::stdin().read_line(&mut input).expect("Please enter a valid equation");
    println!("{}",input);

    //remove whitespace
    input = input.replace(" ",&"".to_string());
    input = input.replace("\n",&"".to_string());
    input = input.replace("-",&"+-".to_string());

    let operators = "+-*/";
    let mut terms_string:Vec<String>= Vec::new();
    let mut terms_float:Vec<f64>= Vec::new();
    let mut operator_after:Vec<char>= Vec::new();
    let mut key = 0;

    // IE 4+9 would be converted to:
        //  terms_float: [4,9]
        //operator_after [+]

    for i in input.chars() { //every character
        if operators.contains(i){ //if i is an operator
            operator_after.push(*&i);
            key+=1;
        } else if terms_string.len()<=key{ //if i is the first character in the term
            terms_string.push(i.to_string());
        } else{ //if i the second (or later) character in the term
            terms_string[key].push(*&i);
        }
    }

    for i in &terms_string{ //convert strings to float
        let x: f64 = i.parse().unwrap();
        terms_float.push(x);
    }

    let mut x:f64 = 0.0;
    while x <operator_after.len() as f64  { //Multiplication and division
        let i = x as usize;
        if operator_after[i]=='*'{
            terms_float[i] = terms_float[i]*terms_float[i+1];
            terms_float.remove(i+1);
            operator_after.remove(i);
        }else if operator_after[i]=='/' {
            terms_float[i] = terms_float[i] / terms_float[i + 1];
            terms_float.remove(i + 1);
            operator_after.remove(i);
        } else{
            x += 1.0;
        }
    }

    x = 0.0;
    while x <operator_after.len() as f64  { //Addition and Subtraction
        let i = x as usize;
        if operator_after[i]=='+'{
            terms_float[i] = terms_float[i]+terms_float[i+1];
            terms_float.remove(i+1);
            operator_after.remove(i);
        }else if operator_after[i]=='-'{
            terms_float[i] = terms_float[i]-terms_float[i+1];
            terms_float.remove(i+1);
            operator_after.remove(i);
        }else{
            x += 1.0;
        }
    }
    let output = terms_float[0];
    println!("{}={}",input,output.to_string());
}
