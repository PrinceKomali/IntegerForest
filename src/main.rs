fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let sg: bool = if args.contains(&String::from("-g")) {
        args.remove(args.iter().position(|x| *x == "-g").unwrap());
        true
    } else { false }; 
    if args.len() != 2 {
        println!("Two arguments (not including \x1b[32m-g\x1b[0m) are required");
        std::process::exit(1)
    }
    let code = match std::fs::read_to_string(&args[1]) {
        Ok(s) => s,
        Err(_) => {
            println!("Error reading file; it may not exist or is unreadable");
            std::process::exit(1)
        }
    };
    run(code, sg);

}

fn run(code: String, sg: bool) {
    let code: Vec<&str> = code.split("\n").collect();
    let instructions: Vec<&str> = code[0].split(",").collect();
    let x_max: u32 = match instructions[0].parse::<u32>() {
        Ok(n)=>n,
        Err(_)=>{
            println!("Error: X Max is not a number!");
            std::process::exit(1)
        }
    };
    let y_max: u32 = match instructions[1].parse::<u32>() {
        Ok(n)=>n,
        Err(_)=>{
            println!("Error: Y Max is not a number!");
            std::process::exit(1)
        }
    };
    let mut func = String::from(instructions[2]);
    let mut grid: Vec<Vec<i32>> = vec![Vec::new(); y_max as usize];
    for i in 0..x_max * y_max {
        grid[(i/x_max) as usize].push(fn_parse(&mut func, i));
    }
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    let mut output = String::new();
    for chr in code[1].chars() {
        if chr == '>' { x = (x + 1) % x_max; }
        if chr == '<' { x = (x - 1) % x_max; }
        if chr == '^' { y = (y - 1) % y_max; }
        if chr == 'v' { y = (y + 1) % y_max; }
        if chr == ';' { output.push(std::char::from_u32(
            grid[(y%y_max) as usize][(x%x_max) as usize] as u32
        ).unwrap()); }
    }
    if sg { 
        for i in grid {
            println!("\x1b[33m{:?}\x1b[0m", i)
        }
    };
    println!("{}", output);


}

fn fn_parse(func: &mut String, n: u32) -> i32 {
    let mut m_check = false;
    let mut s_check = false;
    let mut a_check = false;
    let mut e_check = false;
    let mut d_check = false;
    let mut current_str = String::new();
    let mut num = n as i32;

    func.push('?');
    for i in func.chars() {
        match i {
            '+' if !a_check => a_check = true,
            '/' if !d_check => d_check = true,
            '*' if !m_check => m_check = true,
            '^' if !e_check => e_check = true,
            '-' if !s_check => s_check = true,
            _ => {
                if i.is_ascii_digit() {
                    current_str.push(i);
                }
                else {
                    if current_str == "" { continue; }
                    let temp_n = match current_str.parse::<i32>() {
                        Ok(n) => n,
                        Err(_) => {
                            println!("Error parsing expression: {} is not a valid integer!", current_str);
                            std::process::exit(1);
                        }
                    };
                    if m_check {
                        num *= temp_n;
                        m_check = false;
                    }
                    if s_check {
                        num -= temp_n;
                        s_check = false;
                    }
                    if a_check {
                        num += temp_n;
                        // println!("   {} + {}", num, temp_n);
                        a_check = false;
                    }
                    if e_check {
                        num = num.pow(temp_n as u32);
                        e_check = false;
                    }
                    if d_check {
                        num /= temp_n;
                        d_check = false;
                    }
                    current_str = String::new();
                    continue;
                }
            }
            };

        }
        
    return num;
}