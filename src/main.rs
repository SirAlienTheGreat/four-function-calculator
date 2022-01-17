use std::fs;
use std::fs::File;
use std::io::Read;
use std::io;
use std::error::Error;
use csv::Writer;

fn output_as_csv(vector1:Vec<i32>, vector2:Vec<i64>) -> Result<(), Box<dyn Error>> {
    assert_eq!(vector1.len(),vector2.len());

    let mut wtr = Writer::from_path("output.csv")?;

    for i in 0..vector1.len(){
        println!("{}",vector1[i]);
        wtr.write_record([
            [&vector1[i].to_string()[..4],"-",&vector1[i].to_string()[4..] ].join(""),
            vector2[i].to_string()]);
    }


    wtr.flush()?;
    Ok(())
}

fn main() {
    println!("\n\n      Discord GDPR data grapher by Allen MacFarland\
    \nRequest your discord GDPR data in Discord > Settings > Privacy and security > Request data\
    \nYou will receive an email containing a .zip file with all your data\
    \nExtract the messages folder from this .zip file\
    \nEnter a path to the .zip file\
    \nThis can be an:\
    \n* Absolute path like /run/media/sir-alien-the-great/Barracuda 4TB/Discord GDPR data/messages\
    \n* Relative path like Discord GDPR data/messages\
    \n* Relative path that goes backwards like ../Discord GDPR data/messages");

    let mut input = "".to_string();
    io::stdin().read_line(&mut input).expect("A problem occurred");

    let paths = fs::read_dir(input.trim()).unwrap();
    let mut str_path :Vec<String> = vec![];
    let n = 7;


    for path in paths { // Convert path (:ReadDir) to :String
        if !path.as_ref().unwrap().path().display().to_string().contains("index"){
            str_path.push([path.unwrap().path().display().to_string(),"/messages.csv".to_string()].join(""))
        }

    }


    let mut dates:Vec<String> = vec![];
    let mut messages:Vec<String> = vec![];


    for path in str_path { // For every messages file:

        // Read CSV file as string
        let mut file = File::open(path)
            .expect("Why are you trying to break my program?\nWhy do you hate me?");
        let mut randomstring = String::new();
        file.read_to_string(&mut randomstring)
            .expect("You picked the wrong path because you wanted to see my program fail, didn't you?");

        // We only care about the 2nd and 3rd row of each csv, so write new vectors with just that
        let mut comma_counter = 5;
        let mut in_quotes = false;
        let mut current_date = "".to_string();
        let mut current_string = "".to_string();

        for c in randomstring.chars(){ // For every character:
            if comma_counter == 0 {
                if c != ',' || in_quotes {
                    if c == '"'{
                        if !in_quotes{
                            in_quotes = true;
                        } else {
                            in_quotes = false;
                        }
                    } else {
                        current_string.push(c);
                    }
                }else{
                    comma_counter = 2;
                    dates.push(current_date);
                    current_date = "".to_string();

                    messages.push(current_string);
                    current_string = "".to_string();
                }
            } else if c != ',' && comma_counter ==1 {
                current_date.push(c);
            } else if c == ','{
                comma_counter+=-1;
            }
        }
    }

    let mut months :Vec<i32> = vec![];


    // create list of months
    for date in &dates { //For every date
        let x:i32 = date.chars().take(n).collect::<String>()
            .replace("-","").parse().unwrap(); //example 202003 :i32
        if !months.contains(&x) {
            months.push(x);
        }
    }

    // Bubble sort months
    let mut sorted:bool=false;
    let mut last_unsorted = months.len() - 1;
    while !sorted {
        sorted = true;
        for i in 0..last_unsorted{
            if months[i] > months[i+1]{
                let erste = months[i];
                let zweite = months[i+1];
                months[i] = zweite;
                months[i+1] =erste;
                sorted = false;
            }
        }
        last_unsorted = last_unsorted -1;
    }

    //Calculate the number of messages/characters sent in each month
    let mut months_messages :Vec<i64> = vec![0;months.len().clone().try_into().unwrap()];

    for key in 0..messages.len()-1 { //For every message
        let date:&i32 = &dates[key].chars().take(n).collect::<String>()
            .replace("-","").parse().unwrap();//example 202003 :i32

        for i in 0..months.len(){ //For every month
            if date == &months[i]{
                months_messages[i]+=messages[key].len() as i64;
            }
        }
    }

    //Write to CSV
    if let Err(err) = output_as_csv(months,months_messages){
        println!("{}",err);
    }
}