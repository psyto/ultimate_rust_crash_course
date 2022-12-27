fn main() {
    let x = 5;
    {
        let x = 10;
        println!("{}", x);
    }
    println!("{}", x);
}
