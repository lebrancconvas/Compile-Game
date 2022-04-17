use compilegame::Character;

fn main()
{
    let n1 = 10;
    let n2 = &n1;
    println!("n1: {}", n1);
    println!("n2: {}", n2);

    let mut n3 = 5;

    while n3 >= 3
    {
        println!("n3: {}", n3);
        n3-=1;
    }

    let player = Character::new("Rachel Dean".to_string(), 20);
    player.greeting();
}