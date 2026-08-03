fn main() {

let config_max = Some(3_u8);
// match config_max{
//     Some(max) => println!("The maximum is configured to be {max}"),
//     _=>(),    // Here we have to write the _ because it is mandatory or we can also write none but it is unnecessary so we need to use if and let.
// }

// We can use if and like this and if u want another case then add the else as well

if let Some(max) = config_max{
    println!("The maximum is configured to be {max}");
}else{
    println!("There is no configuration.")
}

}


