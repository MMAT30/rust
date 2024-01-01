
// fn main() {

//     // running functions
//     try_division(4, 2);
//     try_division(1, 0);



//     // creating a none option in two ways
//     let none: Option<i32> = None;
//     println!("{:?}", none);

//     let none = None::<i32>;
//     println!("{:?}", none);


//     // creating a some option to show case the unwrap method
//     let opt_float = Some(0f32);
//     println!("{:?}", opt_float);

//     // unwrapping a some will return the value
//     println!("{:?} unwraps to {:?}", opt_float, opt_float.unwrap());

//     // unwrapping a none will panic
//     // println!("{:?} unwraps to {:?}", none, none.unwrap());
// }


// fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
//     if divisor == 0 {
//         None
//     } else {
//         Some(dividend / divisor)
//     }
// }

// fn try_division(dividend: i32, divisor: i32) {
//     match checked_division(dividend, divisor) {
//         None => println!("{} / {} failed!", dividend, divisor),
//         Some(quotient) => {
//             println!("{} / {} = {}", dividend, divisor, quotient)
//         },
//     }
// }