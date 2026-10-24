


fn main()
{
    println!("Hello, world!");
    insult("Mojang");
    insult("RockstarGames");
    compliment("Technoblade");
    compliment("Steam")

}

fn insult(name : &str)
{
    println!("Hi, {} you are DUMB and you are angry and not happy..", name);
}

fn compliment(name: &str)
{
    println!("Hi, {} i like you because your good and you are happy!", name);
}

