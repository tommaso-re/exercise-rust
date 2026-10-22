/*
data types float int bool char


INTEGERS
UNSIGNED vs SIGNED u/i

i8 i16 i32 i64 i128 signed positive and negatives
u8 u16 u32 u64 u128 unsigned only +

 */

fn main()
{
    let mut x: u16 = 5;
    println!("x is {}" , x);
    x = 4;
    println!("x is {}" , x);

    let is_alive: bool = true;
    let grade: char = 'A';
    let temperature: f64 = 7474.34242;

    //tuple

    let pokemon: (&str, u8) = ("Picachu", 100);

    println!("{} has {} health remaining out of 100", pokemon.0, pokemon.1);

    //arrays

    let mut buffer: [u8; 5] = [1,2,3,4,5];
    buffer[0] = 5;
    println!("{:?}", buffer);

    println!("today i studied the compiler")
}