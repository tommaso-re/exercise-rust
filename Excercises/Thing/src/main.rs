

fn main()
{
    greet("Tommaso")
    println!();
}

fn greet(name: &str)
{
    println!("Hey {}", name);
}

fn add(a:  i32, b: i32) -> i32
{
    return a + b;
}