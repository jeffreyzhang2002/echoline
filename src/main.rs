

fn process_flags(argument: &str) -> String {
    match argument {
        "ltriangle" => {
            return "\u{e0b2}".to_string();
        }

        "lcircle" => {
            return "\u{e0b6}".to_string();
        }
        "lflame" => {
            return "\u{e0c2}".to_string();
        }
        "rtriangle" => {
            return "\u{e0b0}".to_string();
        }
        "rcircle" => {
            return "\u{e0b4}".to_string();
        }
        "rflame" => {
            return "\u{e0c0}".to_string();
        }
        _ => {
            return argument.to_string();
        }

    }


}

fn main() {

    let mut text = Vec::new();
    let mut colors = Vec::new();
    let mut seperator = "\u{e0b0}".to_string(); 
    let mut head = "".to_string();
    let mut tail = "".to_string();
    let mut state = 0;
    let mut direction = "left";

    if std::env::args().len() == 1 {
        println!("Command Format: echoline <flags> [labels] -- [colors]");
        println!("Flags:");
        println!("--head=<Value>\tThe head character");
        println!("--tail=<Value>\tThe tail character");
        println!("--delim=<Value>\tThe delimeter");
        println!("--dir=[r|l]\tThe direction");
        println!("Value can be text or one of these [ltriangle, rtriangle, lcircle, rcircle]");
        println!("Colors");
        println!("Color must following this format. Note that each quote group represents a foreground and a background");
        println!("\"fg1_R;fg1_G;fg1_B bg1_R;bg1_G;bg1_B\" \"fg2_R;fg2_G;fg2_B bg2_R;bg2_G;bg2_B\"");

        std::process::exit(1);
    }

    for arg in std::env::args().skip(1) {
        if arg == "--" {
            state = (state + 1) % 3;
        } else if arg.starts_with("--head=") {
            head = process_flags(&arg[7..arg.len()])
        } else if arg.starts_with("--tail=") {
            tail = process_flags(&arg[7..arg.len()])
        } else if arg.starts_with("--delim=") {
            seperator = process_flags(&arg[8..arg.len()]);
        } else if arg.starts_with("--dir=") {
            if arg.chars().nth(6).unwrap() == 'l' {
                direction = "left";
            } else {
                direction = "right";
            }
        } else {
            if state == 0 {
                text.push(arg.clone());
            } else if state == 1 {
                colors.push(arg.clone());
            }
        }
    }

    
    //Print color

    let mut current_rgb = colors[0].split_ascii_whitespace().collect::<Vec<_>>();
    
    if head != "" {
        print!("\x1b[38;2;{}m{}", current_rgb[1], head);
    }

    print!("\x1b[48;2;{}m", current_rgb[1]);
    
    for i in 0..(text.len() - 1) {
        let next_rgb = colors[ (i + 1) % colors.len() ].split_ascii_whitespace().collect::<Vec<_>>();
        if direction == "right" {
            print!("\x1b[38;2;{}m {} \x1b[48;2;{}m\x1b[38;2;{}m{}", 
                   current_rgb[0], 
                   text[i], 
                   next_rgb[1],
                   current_rgb[1],
                   seperator
            );
        } else {
            print!("\x1b[38;2;{}m {} \x1b[38;2;{}m{}\x1b[48;2;{}m",
                    current_rgb[0],
                    text[i],
                    next_rgb[1],
                    seperator,
                    next_rgb[1]
                   )
        }
        current_rgb = next_rgb;
    }
    print!("\x1b[38;2;{}m {} \x1b[0m", current_rgb[0], text[text.len() - 1]);

    if tail != "" {
        print!("\x1b[38;2;{}m{}", current_rgb[1], tail);
    }

    println!();
}
