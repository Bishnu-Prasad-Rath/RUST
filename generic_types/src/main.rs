// There is an concept known as DRY means do not repeat yourself meaning u must don't repeat a code logic in a system and to preventing the rules violation we use generics.

fn main() {
    // let list = vec![1, 2, 5, 6, 10, 4];
    // let l = larges_i32(&list);
    // let list2 = vec![1.0, 2.1, 5.5, 6.8, 10.1, 4.9];
    // let l2 = largest_f64(&list);
    let list2 = vec![1.0, 2.1, 5.5, 6.8, 10.1, 4.9];
    let l2 = largest(&list2);
}

// fn largest_f64(list: &[f64]) -> &f64 {
//     let mut result = &list[0];

//     for item in list {
//         if item > result {
//             result = item;
//         }
//     }
//     result
// } //Here these functions are accepting different datatypes and return with different data types as well but with same concpet so it is technically repeating process
// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut result = &list[0];

//     for item in list {
//         if item > result {
//             result = item;
//         }
//     }
//     result
// }
// And to prevent this we can use generics here.

fn largest<T>(list: &[T]) -> &T {
    //Here generics infere the datatpyes depending what u are giving the input to use this function.
    let mut result = &list[0];

    for item in list {
        if item > result {
            result = item;
        }
    }
    result
}
