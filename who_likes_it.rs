fn likes(names: &[&str]) -> String {
    let size = names.len();
    let message: String = match size {
        0 => "no one likes this".to_string(),
        1 => format!("{} likes this", names[0]),
        2 => format!("{} and {} like this", names[0], names[1]),
        3 => format!("{}, {} and {} like this", names[0], names[1], names[2]),
        _ => format!("{}, {} and {} others like this", names[0], names[1], size - 2)
    };
    message
}

fn main() {
    let arr = ["Cesar"];
    println!("{}", likes(&arr));
}