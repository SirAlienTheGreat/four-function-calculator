use fltk::{app,prelude::*,window::Window};
use fltk::button::Button;
use fltk::frame::Frame;

fn calculate(label:&mut fltk::frame::Frame) {
    let mut input =label.label().parse::<String>().unwrap();

    //remove whitespace
    input = input.replace(" ",&"".to_string());
    input = input.replace("\n",&"".to_string());
    input = input.replace("-",&"+-".to_string());

    let operators = "+-*/^";
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
    while x <operator_after.len() as f64  { //Exponents
        let i = x as usize;
        if operator_after[i]=='^'{
            terms_float[i] = terms_float[i].powf(terms_float[i+1]);
            terms_float.remove(i+1);
            operator_after.remove(i);
        }else{
            x += 1.0;
        }
    }

    x = 0.0;
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
    label.set_label(&output.to_string());
}

fn add_to_text(character:char, label:&mut fltk::frame::Frame){
    let mut text =label.label().parse::<String>().unwrap();
    text.push(character);
    label.set_label(&text);
    if text.len()>=9 &&text.len()<12 {
        label.set_label_size(35);
    }else if text.len() >= 12{
        label.set_label_size(30);
    }
}

fn backspace(label:&mut fltk::frame::Frame){
    let mut text =label.label().parse::<String>().unwrap();
    text.pop();
    label.set_label(&text);
    label.set_label_size(45);
}

fn main() {
    let equation = "".to_string();

    //window variables
    let text_box_size = 70;
    let buffer = 20;
    let button_size = 50;
    let button_font_size = 20;


    let app = app::App::default()
        .with_scheme(app::Scheme::Gtk);

    let mut windowobj = Window::new(400,0,7*button_size+8*buffer,3*button_size+4*buffer+text_box_size,"Calculator");

    let _frame = Frame::default()
        .center_of(&windowobj)
        .size_of(&windowobj);

    let mut text_label = Frame::default()
        .with_size(250,text_box_size)
        .with_pos(buffer,buffer)
        .with_label(&equation);
    text_label.set_label_size(45);


    // Top row buttons (1-5)
    let mut but1 = Button::default()
        .with_size(button_size,button_size)
        .with_pos(buffer,text_box_size+buffer)
        .with_label("1");
    but1.set_callback({let mut text_label = text_label.clone(); move |_|{
        add_to_text('1',&mut text_label)}});
    but1.set_label_size(button_font_size);

    let mut but2 = Button::default()
        .with_size(button_size,button_size)
        .with_pos(buffer*2+button_size,text_box_size+buffer)
        .with_label("2");
    but2.set_callback({let mut text_label = text_label.clone(); move |_|{
        add_to_text('2',&mut text_label)}});
    but2.set_label_size(button_font_size);

    let mut but3 = Button::default()
        .with_size(button_size,button_size)
        .with_pos(buffer*3+button_size*2,text_box_size+buffer)
        .with_label("3");
    but3.set_callback({let mut text_label = text_label.clone(); move |_|{
        add_to_text('3',&mut text_label)}});
    but3.set_label_size(button_font_size);

    let mut but4 = Button::default()
        .with_size(button_size,button_size)
        .with_pos(buffer*4+button_size*3,text_box_size+buffer)
        .with_label("4");
    but4.set_callback({let mut text_label = text_label.clone(); move |_|{
        add_to_text('4',&mut text_label)}});
    but4.set_label_size(button_font_size);

    let mut but5 = Button::default()
        .with_size(button_size,button_size)
        .with_pos(buffer*5+button_size*4,text_box_size+buffer)
        .with_label("5");
    but5.set_callback({let mut text_label = text_label.clone(); move |_|{
        add_to_text('5',&mut text_label)}});
    but5.set_label_size(button_font_size);


    // Second row buttons (6-0)
    let mut but6 = Button::default()
        .with_size(button_size,button_size)
        .with_pos(buffer,text_box_size+buffer*2+button_size)
        .with_label("6");
    but6.set_callback({let mut text_label = text_label.clone(); move |_|{
        add_to_text('6',&mut text_label)}});
    but6.set_label_size(button_font_size);

    let mut but7 = Button::default()
        .with_size(button_size,button_size)
        .with_pos(buffer*2+button_size,text_box_size+buffer*2+button_size)
        .with_label("7");
    but7.set_callback({let mut text_label = text_label.clone(); move |_|{
        add_to_text('7',&mut text_label)}});
    but7.set_label_size(button_font_size);

    let mut but8 = Button::default()
        .with_size(button_size,button_size)
        .with_pos(buffer*3+button_size*2,text_box_size+buffer*2+button_size)
        .with_label("8");
    but8.set_callback({let mut text_label = text_label.clone(); move |_|{
        add_to_text('8',&mut text_label)}});
    but8.set_label_size(button_font_size);

    let mut but9 = Button::default()
        .with_size(button_size,button_size)
        .with_pos(buffer*4+button_size*3,text_box_size+buffer*2+button_size)
        .with_label("9");
    but9.set_callback({let mut text_label = text_label.clone(); move |_|{
        add_to_text('9',&mut text_label)}});
    but9.set_label_size(button_font_size);

    let mut but0 = Button::default()
        .with_size(button_size,button_size)
        .with_pos(buffer*5+button_size*4,text_box_size+buffer*2+button_size)
        .with_label("0");
    but0.set_callback({let mut text_label = text_label.clone(); move |_|{
        add_to_text('0',&mut text_label)}});
    but0.set_label_size(button_font_size);

    // Bottom row buttons (Symbols)
    let mut butplus = Button::default()
        .with_size(button_size,button_size)
        .with_pos(buffer,text_box_size+buffer*3+button_size*2)
        .with_label("+");
    butplus.set_callback({let mut text_label = text_label.clone(); move |_|{
        add_to_text('+',&mut text_label)}});
    butplus.set_label_size(button_font_size);

    let mut butminus = Button::default()
        .with_size(button_size,button_size)
        .with_pos(buffer*2+button_size,text_box_size+buffer*3+button_size*2)
        .with_label("-");
    butminus.set_callback({let mut text_label = text_label.clone(); move |_|{
        add_to_text('-',&mut text_label)}});
    butminus.set_label_size(button_font_size);

    let mut buttimes = Button::default()
        .with_size(button_size,button_size)
        .with_pos(buffer*3+button_size*2,text_box_size+buffer*3+button_size*2)
        .with_label("x");
    buttimes.set_callback({let mut text_label = text_label.clone(); move |_|{
        add_to_text('*',&mut text_label)}});
    buttimes.set_label_size(button_font_size);

    let mut butdivide = Button::default()
        .with_size(button_size,button_size)
        .with_pos(buffer*4+button_size*3,text_box_size+buffer*3+button_size*2)
        .with_label("÷");
    butdivide.set_callback({let mut text_label = text_label.clone(); move |_|{
        add_to_text('/',&mut text_label)}});
    butdivide.set_label_size(button_font_size);

    let mut butbackspace = Button::default()
        .with_size(button_size,button_size)
        .with_pos(buffer*5+button_size*4,text_box_size+buffer*3+button_size*2)
        .with_label("DEL");
    butbackspace.set_callback({let mut text_label = text_label.clone(); move |_|{
        backspace(&mut text_label)}});
    butbackspace.set_label_size(button_font_size);

    let mut butclear = Button::default()
        .with_size(button_size,button_size)
        .with_pos(6*buffer+5*button_size,buffer+text_box_size)
        .with_label("Clear");
    butclear.set_callback({let mut text_label = text_label.clone();
        move |_|{
            text_label.set_label(&"".to_string());
            text_label.set_label_size(45);
        }
    });

    let mut butdecimal = Button::default()
        .with_size(button_size,button_size)
        .with_pos(buffer*6+button_size*5,text_box_size+buffer*2+button_size)
        .with_label(".");
    butdecimal.set_callback({let mut text_label = text_label.clone(); move |_|{
        add_to_text('.',&mut text_label)}});
    butdecimal.set_label_size(button_font_size);

    let mut butpower = Button::default()
        .with_size(button_size,button_size)
        .with_pos(buffer*6+button_size*5,text_box_size+buffer*3+button_size*2)
        .with_label("^");
    butpower.set_callback({let mut text_label = text_label.clone(); move |_|{
        add_to_text('^',&mut text_label)}});
    butpower.set_label_size(button_font_size);

    let mut butequals = Button::default()
        .with_size(button_size,button_size*3+buffer*2)
        .with_pos(buffer*7+button_size*6,text_box_size+buffer)
        .with_label("=");
    butequals.set_callback({let mut text_label = text_label.clone(); move |_| calculate(&mut text_label) });
    butequals.set_label_size(button_font_size);

    windowobj.make_resizable(true);
    windowobj.end();
    windowobj.show();
    app.run().unwrap();
}
