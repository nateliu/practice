pub fn run(){
    let first_str = String::from("hello");
    let second_str = first_str; //value moved here

    //println!("{},world",first_str); //value borrowed here after move

    println!("{},world",second_str); 
}