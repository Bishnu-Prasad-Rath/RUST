use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("Hello, world!");
    //The difference between ASCII and UTF characters are ASCII character can \only print one kind of lanuage that is only english
    //but UTF characters can print different language with valid string

    //When we add 2 strings using + operator than, the format will be &str to str.
    //The keyword format helps to print a string but it does not take the ownership if we use it.

    let hello = String::from("नमस्ते");

    // नमस् in byte
    //[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
    //224, 165, 135]

    //Scaler
    //['न', 'म', 'स', '्', 'त', 'े']

    //grapheme cluster
    //["न", "म", "स्", "ते"]

    for e in hello.graphemes(true).collect::<Vec<&str>>() {
        println!("E = {}", e);
    }
}
