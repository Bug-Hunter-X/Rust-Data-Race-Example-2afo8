fn main() {    let mut x = 5;    {        let y = &mut x;        *y = 6;    }    {        let z = &mut x;        *z = 7;    }    println!("x = {}", x);}
This solution uses separate scopes to ensure that only one mutable reference to `x` exists at any given time, thereby preventing the data race.