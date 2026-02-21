fn main() {
    // println!("Hello World!")
    let a = [1, 2];
    println!("{:?}", a);
    println!("Second element is : {}", a[0]);
    let mut b = vec![1, 2, 3, 4, 5, 6];
    b.push(34);
    println!("{:?}", b);
    let animal: &[&str] = &["Animals", "Open"];
    println!("Animal slices are: {:?}", animal);
    println!("{}", animal[1]);

    let mut res: String = String::from("Hello, ");
    res.push_str("World");
    println!("{}", res);
    let res_shadow: &str = &res[0..=5];
    println!("{}", res_shadow);
    {
        let res_shadow: &str = &res[0..5];
        println!("{}", res_shadow);
    }
}
