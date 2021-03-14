fn main() {
    let str = "first apple hello compact";
    println!("{}", to_pig_latin(&str))
}

fn to_pig_latin(str: &str) -> String {
    let mut chars = str.chars().peekable();
    let mut pigified_string = String::new();
    while let Some(c) = chars.next() {
        println!("char = {}", c);
        let suffix = match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                pigified_string.push(c);
                String::from("-hay")
            }
            'a'..='z' | 'A'..='Z' => {
                format!("-{}ay", c)
            }
            _ => {
                pigified_string.push(c);
                continue;
            }
        };

        println!("suffix = {}", suffix);
        while let Some(&c) = chars.peek() {
            println!("peek char = {}", c);
            match c {
                'a'..='z' | 'A'..='Z' => {
                    chars.next();
                    pigified_string.push(c);
                }
                _ => break,
            }
        }
        println!("final suffix = {}", &suffix);
        pigified_string += &suffix;
    }
    pigified_string
}