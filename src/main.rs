fn main() {
    let mut text = String::from("Пик Балмера");
    reverse_words_at_odd_positions(&mut text);
    println!("{text}");
}

fn reverse_words_at_odd_positions(text: &mut String) {
    // 1. find words at odd positions starting with the first word.
    // 2. find the borders of the slice of the text where the word is
    // 3. reverse the chars
    let mut word_counter: u32 = 1;
    let mut start_search: usize = 0;

    while start_search < text.len() {
        let (begin, end) = {
            let s = text.as_str();
            if start_search > s.len() {
                break;
            }
            let tail = &s[start_search..];

            let Some(diff) = tail.find(|c: char| !c.is_whitespace()) else {
                break;
            };

            let word_start = start_search + diff;
            let word_end = match s[word_start..].find(char::is_whitespace) {
                Some(word_len) => word_start + word_len,
                None => text.len(),
            };
            (word_start, word_end)
        };

        if word_counter % 2 == 1 {
            let text_vec = unsafe { text.as_mut_vec() };
            reverse_chars_in_word(&mut text_vec[begin..end]);
        }

        word_counter += 1;
        start_search = end;
    }
}

fn reverse_chars_in_word(word: &mut [u8]) {
    word.reverse();
    let mut start = 0;
    while start < word.len() {
        let mut end = start;
        while end < word.len() && (word[end] & 0xC0) == 0x80 {
            end += 1;
        }
        if end >= word.len() {
            return;
        }
        word[start..=end].reverse();
        start = end + 1;
    }
}
#[cfg(test)]
#[path = "../tests/unit/monologue.rs"]
mod monologue_tests;
