

#[cfg(test)]
mod tests {
    use super::transform;

    #[test]
    fn simple() {
        let test_cases = [
          ("", 5, ""),
          ("test", 5, "test "),
          ("Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua", 12,
            "Lorem  ipsum\ndolor    sit\namet        \nconsectetur \nadipiscing  \nelit  sed do\neiusmod     \ntempor      \nincididunt  \nut labore et\ndolore magna\naliqua      ")
        ];

        for &(input, line_width, expected) in &test_cases {
            println!("input: '{}'", input);
            assert_eq!(transform(input, line_width), expected);
        }
    }
}

pub fn transform(input: &str, line_width: u32) -> String {
    let mut output = String::new();
    let mut current = String::new();
    let mut chars_count: u32 = 0;
    let mut i: u32 = 0;
    let mut words: Vec<String> = Vec::new();

    for c in input.chars() {
   
         if i == line_width + 1 {

            output.push_str(&process_chunk(&words, line_width, chars_count));
            output.push('\n');
            
            words.clear();
            chars_count = 0;

            i = current.len() as u32;
        }

        if c == ' ' {
            words.push(current.clone());
            chars_count = chars_count + current.len() as u32;
            current.clear();
        } else {
            current.push(c);
        }

        i = i + 1;   
    }

    if current != "" {
        words.push(current.clone());
        output.push_str(&process_chunk(&words, line_width, current.len() as u32));
    }

    output
}

fn process_chunk(words: &Vec<String>, line_width: u32, chars_count: u32) -> String {
    let mut chunk = String::new();
    let words_count = words.len() as u32;
    let mut rest = line_width - chars_count - (words_count - 1);
 
    if words_count == 1 {
        chunk.push_str(words.first().unwrap());
        chunk.push_str(&" ".repeat(rest as usize));
     } else {
         for w in words.iter() {
            chunk.push_str(w);
            if w != words.last().unwrap() {
                chunk.push(' ');
                if rest > 0 {
                    chunk.push_str(&" ".repeat(rest as usize));
                    rest = rest - 1;
                }
            }
        }
    }
    chunk
}