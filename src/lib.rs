

#[cfg(test)]
mod tests {
    use super::transform;

    #[test]
    fn simple() {
        let test_cases = [
            ("", 5, ""),
            ("test", 5, "test "),
            ("one two three", 5, "one  \ntwo  \nthree"),
            ("one two three", 6, "one   \ntwo   \nthree "),
            ("one two three", 7, "one two\nthree  "),
            ("one two three", 10, "one    two\nthree     "),
            ("one two three", 13, "one two three"),
            ("one two three", 14, "one  two three"),
            ("one two three", 15, "one  two  three"),
            ("one two three", 16, "one   two  three"),
            ("one two three", 17, "one   two   three"),
            ("one two three", 18, "one    two   three"),
            ("one two three four", 18, "one two three four"),
            ("раз два три", 5, "раз  \nдва  \nтри  "),
            ("раз два три четыре", 10, "раз    два\nтри четыре"),
            ("раз два три четыре", 3, "too long word"),
            ("раз два три четыре", 2, "too long word"),
            ("只要功夫深 铁杵磨成针", 7, "只要功夫深  \n铁杵磨成针  "),
            ("只要功夫深 铁杵磨成针 爱美之心 人皆有之", 20, "只要功夫深   铁杵磨成针   爱美之心\n人皆有之                "),
            ("aaaaa bbbbb cccc dddd", 20, "aaaaa   bbbbb   cccc\ndddd                "),
             ("one two three four", 6, "one   \ntwo   \nthree \nfour  "),
            ("Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua", 12,
                "Lorem  ipsum\ndolor    sit\namet        \nconsectetur \nadipiscing  \nelit  sed do\neiusmod     \ntempor      \nincididunt  \nut labore et\ndolore magna\naliqua      "),
            ("Mary and Samantha arrived at the bus station early but waited until noon for the bus", 10,
            "Mary   and\nSamantha  \narrived at\nthe    bus\nstation   \nearly  but\nwaited    \nuntil noon\nfor    the\nbus       ")
        ];

        for &(input, line_width, expected) in &test_cases {
            println!("input: '{}'", input);
            assert_eq!(transform(input, line_width), expected);
        }
    }
}

pub fn transform(input: &str, line_width: u32) -> String {
    let mut output = String::new();
    let mut chars_count: u32 = 0;
    let mut i: u32 = 0;
    let mut words: Vec<String> = vec!["".to_string()];
    let mut word_num: usize = 0;
    let mut words_count: u32 = 0;

    // enumerate all chars of input string, split to words, place words to vector and transter these words to output string together with spaces calculated by task conditions
    for c in input.chars() {
   
         if i == line_width + 1 {
  
            if process_words(&mut words, &mut words_count, &mut word_num, &mut output, line_width, &mut chars_count) == false {
                return "too long word".to_string();
            }

            i = words[word_num].chars().count() as u32;
        }

        if c == ' ' {
            // calculates count of complete words
            words_count = words_count + 1;

            // calculates count of non-space characters
            chars_count = chars_count + words[word_num].chars().count() as u32;
       
            word_num = word_num + 1;
            words.push("".to_string());
        } else {
            words[word_num].push(c);
        }

        i = i + 1;   
    }

    if i == line_width + 1 {

        if process_words(&mut words, &mut words_count, &mut word_num, &mut output, line_width, &mut chars_count) == false {
            return "too long word".to_string();
        }
    }
    
    // if there are still some unprocessed words, finally process it
    if words.len() > 0 && words[0] != "" {
        chars_count = chars_count + words[word_num].chars().count() as u32;
        output.push_str(&process_chunk(&words, words.len() as u32, line_width, chars_count));
    }
    output
}

fn process_words(words: &mut Vec<String>, words_count: &mut u32, word_num: &mut usize, output: &mut String, line_width: u32, chars_count: &mut u32) -> bool {
  
    // if words vector is empty then too long word encountered - return false
    if *words_count == 0 {
        return false;
    }
    output.push_str(&process_chunk(words, *words_count, line_width, *chars_count));
    output.push('\n');

    *chars_count = 0;

    // drain all complete words from the vector preserving last incomplete word, if exist
    words.drain(0..*words_count as usize);
    if words.len() == 0 {
        words.push("".to_string());         
    }

    *words_count = 0;
    *word_num = 0;

    true
}

fn process_chunk(words: &Vec<String>, words_count: u32, line_width: u32, chars_count: u32) -> String {
    let mut chunk = String::new();
    let total_spaces = line_width - chars_count;

    if words_count == 1 {

        // if onle one word then pad it with spaces from the right
        let rest = total_spaces  as usize;
        chunk.push_str(&words[0]);
        if rest > 0 {
            chunk = format!("{}{:rest$}", chunk, ' ');
        }
     } else {

        // calculate necessary amount of spaces to append to each word from the right (except the latest word)
         let word_spaces = total_spaces / (words_count - 1);

         // calculate rest amount of spaces
         let mut rest_spaces = total_spaces - word_spaces * (words_count - 1);
     
         // enumerate all words, pad each of them except the latest with word_spaces number of spaces
         // and additionally add one space to each word from left to right redusing rest_spaces by 1 until it become 0
         for (i, w) in words.iter().enumerate() {
            if i >= words_count as usize {
                break;
            }
            chunk.push_str(w);
            if (i as u32) < words_count - 1 {
               let ws = word_spaces as usize;
               chunk = format!("{}{:ws$}", chunk, " ");
                if rest_spaces > 0 {
                    chunk.push(' ');
                    rest_spaces = rest_spaces - 1;
                }
            }
        }

    }

    chunk
}