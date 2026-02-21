fn add() -> i32 {
    const REP: i32 = 344;
    let a = 34;
    let b = 242;
    a + b + REP
}

fn main() {
    println!("Hello World!");
    let name = "John";
    println!("{} is a male", name);
    let res = add();
    println!("{}", res);
    let logged_in = true;
    let is_admin = false;

    println!("Is regular user: {}", logged_in && !is_admin);
    println!("Has any access: {}", logged_in || is_admin);
    println!("Not logged in: {}", !logged_in);
    let rt = 34;
    let pp = rt >= 23;
    println!("{}", pp);

    if pp && rt >= 10 {
        println!("The condition satisfies");
    } else {
        println!("The condition not satisfies");
    }
    let day = 4;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."),
    }
}
