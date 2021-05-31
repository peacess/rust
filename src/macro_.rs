

macro_rules! macro_f {
    ($($field:expr),+) => {
        fn tt(){
            let fields: Vec<String> = vec![$($field),+];
            println!("{:?}",fields);
        }
    };
}
macro_f!("a".to_owned(), "b".to_owned());

// macro_rules! macro_array {
//     ($($field:expr),+) => {
//         fn tt(){
//             let fields: Vec<String> = vec![$($field),+];
//             println!("{:?}",t);
//         }
//     };
//     ($($field:expr),+) => {
//         fn tt(){
//             let fields: Vec<String> = vec![$($field),+];
//             println!("{:?}",t);
//         }
//     };
// }
