use std::io;

fn main() {

    println!("Extended Euclid: calculate ax + bx = gcd(a,b)");
    println!("Insert a:");

    let mut a = String::new();

    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");

    let a: i64 = a
        .trim()
        .parse()
        .expect("Input entered was not a number");

    println!("Insert b:");

    let mut b = String::new();

        io::stdin()
            .read_line(&mut b)
            .expect("Failed to read line");
    
        let b: i64 = b
            .trim()
            .parse()
            .expect("Input entered was not a number");
    
    let (d,x,y) = extended_euclid(a,b);
    println!("EE({},{}) = <{},{},{}>",a,b,d,x,y);


}


fn extended_euclid(a: i64, b: i64) -> (i64, i64, i64) {

    if b == 0 {  
                println!("<{},{},{}>",a,1,0);
                (a,1,0)
    } else {

        let (df, xf, yf) = extended_euclid(b,a%b);
        let (d,x,y) = (df, yf, xf-(a/b)*yf);
        println!("EE({},{}) = <{},{},{}>",b,a%b,d,x,y);
        (d,x,y)


    }
}