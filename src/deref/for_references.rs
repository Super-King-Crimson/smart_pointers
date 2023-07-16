pub fn explain() {
    println!("Think of a pointer as a person stepping back and literally pointing to  smthn else");

    let x = 5;
    let y = &x; //y goes to the value of x, takes a step back, and points to x

    println!("x is just y dereferenced, right? {}", x == *y);
    //If we want to get the value back, you'd have to step forward, back to its value
    //So think of & as stepping back and pointing to where you were,
    //And * as going to the place you're pointing to

    println!("You probably knew this already.");
}