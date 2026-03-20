fn split_string<'a>(string: &'a str, delimeter: &str) -> Vec<&'a str> {
    if string.is_empty() {
        return Vec::new();
    }
    // split string on delimeter
    let mut matches: Vec<&str> = Vec::new();
    for word in string.split(delimeter) {
        // filter out empty strings
        if !word.is_empty() {
            matches.push(word);
        }
    }
    matches
}

#[derive(PartialEq, Debug)]
struct Differences<'a> {
    only_in_first: Vec<&'a str>,
    only_in_second: Vec<&'a str>,
}

fn find_differences<'a>(first_string: &'a str, second_string: &'a str) -> Differences<'a> {
    let diff_words_first = split_string(first_string, " ");
    let diff_words_second = split_string(second_string, " ");
    let mut only_in_first: Vec<&str> = Vec::new();
    let mut only_in_second: Vec<&str> = Vec::new();

    for &word_first in &diff_words_first {
        if !diff_words_second
            .iter()
            .any(|&word_second| word_second == word_first || word_second.contains(word_first))
        {
            only_in_first.push(word_first);
        }
    }

    for &word_second in &diff_words_second {
        if !diff_words_first
            .iter()
            .any(|&word_first| word_first == word_second || word_first.contains(word_second))
        {
            only_in_second.push(word_second);
        }
    }

    Differences {
        only_in_first,
        only_in_second,
    }
}

fn merge_names(first_name: &str, second_name: &str) -> String {
    if first_name.is_empty() && second_name.is_empty() {
        return String::new();
    } else if first_name.is_empty() {
        return second_name.to_string();
    } else if second_name.is_empty() {
        return first_name.to_string();
    }
    let mut weaved_names = String::new();
    let mut idx1 = 0;
    let mut idx2 = 0;
    let vowels = "aeiou";
    let mut toggle = true;

    while idx1 < first_name.len() || idx2 < second_name.len() {
        if toggle && idx1 < first_name.len() {
            let mut starting = true;
            while idx1 < first_name.len() {
                let c = first_name.chars().nth(idx1).unwrap();
                if vowels.contains(c) {
                    if starting {
                        // Append the vowel if it's the starting character
                        weaved_names.push(c);
                        idx1 += 1;
                        starting = false;
                        continue;
                    } else {
                        // Do not increment idx1 here; we might need to process this vowel later
                        break;
                    }
                }
                weaved_names.push(c);
                idx1 += 1;
                starting = false;
            }
            toggle = false;
        } else if idx2 < second_name.len() {
            let mut starting = true;
            while idx2 < second_name.len() {
                let c = second_name.chars().nth(idx2).unwrap();
                if vowels.contains(c) {
                    if starting {
                        // Append the vowel if it's the starting character
                        weaved_names.push(c);
                        idx2 += 1;
                        starting = false;
                        continue;
                    } else {
                        // Do not increment idx2 here; we might need to process this vowel later
                        break;
                    }
                }
                weaved_names.push(c);
                idx2 += 1;
                starting = false;
            }
            toggle = true;
        } else {
            break;
        }
    }

    // Append any remaining characters from both names
    weaved_names.extend(first_name.chars().skip(idx1));
    weaved_names.extend(second_name.chars().skip(idx2));

    weaved_names
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_string() {
        // First, make sure the lifetimes were correctly marked
        let matches;
        let string_to_split = String::from("Hello, World!");

        {
            let delimeter = String::from(", ");
            matches = split_string(&string_to_split, &delimeter);
        }
        println!("Matches can be printed! See: {:?}", matches);

        // Now check the split logic
        assert_eq!(split_string(&"", &""), Vec::<&str>::new());
        assert_eq!(
            split_string(&"Hello, World!", &", "),
            vec!["Hello", "World!"]
        );
        assert_eq!(
            split_string(
                &"I this think this that this sentence this is this very this confusing this ",
                &" this "
            ),
            vec!["I", "think", "that", "sentence", "is", "very", "confusing"]
        );
        assert_eq!(
            split_string(&"apple🍎banana🍎orange", &"🍎"),
            vec!["apple", "banana", "orange"]
        );
        assert_eq!(
            split_string(
                &"Ayush;put|a,lot~of`random;delimeters|in|this,sentence",
                &";"
            ),
            vec![
                "Ayush",
                "put|a,lot~of`random",
                "delimeters|in|this,sentence"
            ]
        );
    }

    #[test]
    fn test_find_differences() {
        assert_eq!(
            find_differences(&"", &""),
            Differences {
                only_in_first: Vec::new(),
                only_in_second: Vec::new()
            }
        );
        assert_eq!(
            find_differences(&"pineapple pen", &"apple"),
            Differences {
                only_in_first: vec!["pineapple", "pen"],
                only_in_second: Vec::new()
            }
        );
        assert_eq!(
            find_differences(
                &"Sally sold seashells at the seashore",
                &"Seashells seashells at the seashore"
            ),
            Differences {
                only_in_first: vec!["Sally", "sold"],
                only_in_second: vec!["Seashells"]
            }
        );
        assert_eq!(
            find_differences(
                "How much wood could a wood chuck chuck",
                "If a wood chuck could chuck wood"
            ),
            Differences {
                only_in_first: vec!["How", "much"],
                only_in_second: vec!["If"]
            }
        );
        assert_eq!(
            find_differences(
                &"How much ground would a groundhog hog",
                &"If a groundhog could hog ground"
            ),
            Differences {
                only_in_first: vec!["How", "much", "would"],
                only_in_second: vec!["If", "could"]
            }
        );
    }

    #[test]
    fn test_merge_names() {
        assert_eq!(merge_names(&"alex", &"jake"), "aljexake");
        assert_eq!(merge_names(&"steven", &"stephen"), "ststevephenen");
        assert_eq!(merge_names(&"gym", &"rhythm"), "gymrhythm");
        assert_eq!(merge_names(&"walter", &"gibraltor"), "wgaltibreraltor");
        assert_eq!(merge_names(&"baker", &"quaker"), "bqakueraker");
        assert_eq!(merge_names(&"", &""), "");
        assert_eq!(merge_names(&"samesies", &"samesies"), "ssamamesesiieses");
        assert_eq!(merge_names(&"heather", &"meagan"), "hmeeathageran");
        assert_eq!(merge_names(&"panda", &"turtle"), "ptandurtlae");
        assert_eq!(merge_names(&"hot", &"sauce"), "hsotauce");
        assert_eq!(merge_names(&"", &"second"), "second");
        assert_eq!(merge_names(&"first", &""), "first");
    }
}
