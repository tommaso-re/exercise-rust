

fn main()
{
    greet("Tommaso");
    println!("{} and 18 is odd = {}", add(10,12), is_odd(18));
}

fn greet(name: &str) -> ()
{
    println!("Hey {}", name);
}

fn add(a:  i32, b: i32) -> i32
{
    return a + b;
}

fn is_odd(n: u16) -> bool
{
    if n % 2 == 1
    {
        return true;
    }
    else
    {
        return false;
    }
}