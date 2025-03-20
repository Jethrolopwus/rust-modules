
use maths::lib::basic::{arithmetic, logical};


fn main() {
    println!("the sum of 2 and 3 is {:?} \n", arithmetic::add(2,3));

    print!("the difference of 2 and 3 is {:?} \n", arithmetic::sub(2,3));
    print!("{:?}\n", logical::less_than(2, 3));
}
