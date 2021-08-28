#[macro_use] 
extern crate random_number;
use text_io::read;

fn main(){
    let n:i32 = random!(1..=3);
    //user chosse
        println!("1.(ROCK) 2.(PAPER) 3.(SISSORS)");

        let user_choose: i32 = read!();

    if user_choose ==1{
        println!("You choose ROCK")
    }
    else if user_choose==2{
        println!("You choose PAPER")
    }
    else if user_choose==3{
        println!("You choose SISSOR")
    }
    else{
        println!("Invalid choice");
    }

    // computer choose
if n == 1{
    println!("Computer choose ROCK");
}
else if n==2{
    println!("computer choose PAPER");
}
else if n==3{
    println!("computer choose SISSORS");
}
else{
    println!("Invalid choice");
}

if n==user_choose {
    println!("DRAW !");
}
else if n==1 && user_choose ==3 || n==2 && user_choose==1 || n==3 && user_choose==2{
    println!("Computer wins");
}
else if user_choose==1 && n ==3 || n==2 && user_choose==1 || user_choose==3 && n==2{
    println!("You wins");
}
else{
    println!("Invalid Coice");
}

}