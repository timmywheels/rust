use std::io::stdin;

fn if_statement() {

    let temp = 100;
    if temp == 75 {
        println!("it's the perfect temp")
    } else if temp < 75 {
        println!("it's not bad outside")
    } else if temp > 75 {
        println!("it's warm outside")
    } else if temp > 90 {
        println!("it's hot outside")
    } else {
        println!("it's nasty outside")
    }

    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}", day);

}

fn while_loop() {
    let mut num = 0;
    while num < 100 {
        println!("num is {}", num);
        // if num == 5 { continue; }
        num += 1;
    }

    let mut y = 1;
    loop { // while true
        y *= 2;
        println!("y = {}", y);
        if y == 1 << 10 { break; }
    }

}

fn for_loop() {
    let stopping_point = 11;
    for x in 1..stopping_point {
        if x == 3 { continue; }
        if x == 5 { break; }
        println!("x = {}", x);
    }

    for (pos, val) in (30..45).enumerate() {
        println!("pos: {}, val: {}", pos, val);
    }
}

fn match_statement() {

    let country_code = 45;

    let country = match country_code {
        45 => "USA",
        46 => "China",
        47..=100 => "Unknown",
        _ => "invalid"
    };

    println!("Country Code {} represents {}", country_code, country);

    let x = false;
    let s = match x {
        true => "yes",
        false => "no"
    };
    println!("s = {}", s);
}

fn combination_lock() {
    enum State {
        Locked,
        Failed,
        Unlocked
    }
    let code = String::from("12345");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end())
                    }
                    Err(_) => continue
                }
                if entry == code {
                    state = State::Unlocked;
                    continue;
                }
                if !code.starts_with(&entry) {
                    state = State::Failed
                }
            }
            State::Failed => {
                println!("FAILED");
                entry.clear(); // sets to empty string ""
                state = State::Locked;
            }
            State::Unlocked => {
                println!("UNLOCKED");
                return;
            }
        }
    }

}

pub fn control_flow() {
    if_statement();
    while_loop();
    for_loop();
    match_statement();
    // combination_lock();
}