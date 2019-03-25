use std::fmt;

#[allow(dead_code)]
struct Structure(i32);

impl fmt::Display for Structure{
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.0)
    }
}

fn main(){
    println!("{} days",31);
    println!("{0} -> {1},{1} -> {0}",0,1);
    println!("{who} {verb} {subject}",who="I",verb="learn",subject="rust");
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    
    println!("{number:>0width$}", number=1, width=6);
    println!("{number:<width$}", number=1, width=6);
    println!("{number:>width$}", number=1, width=6);

    println!("My name is {0}, {1} {0}", "Bond","Rex");

    println!("This struct `{}` would print...", Structure(3));
}