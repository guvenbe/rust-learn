use std::collections::HashMap;

// Lifetime parameter in trait definition binds input and output lifetimes
pub trait TextAnalyzer<'a> {
    fn analyze(&self, data: &'a str) -> (&'a str, usize);
}

pub struct WordCounter;

impl<'a> TextAnalyzer<'a> for WordCounter {
    // The output references must live as long as the input 'a
    fn analyze(&self, data: &'a str) -> (&'a str, usize) {
        let word_count = data.split_whitespace().count();
        let first_word = data.split_whitespace().next().unwrap_or("");
        (first_word, word_count)
    }
}

pub struct FrequencyAnalyzer;

impl FrequencyAnalyzer {
    // The returned vector contains references tied to the input text's lifetime
    pub fn get_top_words<'a>(&self, text: &'a str, count: usize) -> Vec<(&'a str, usize)> {
        let mut freq_map: HashMap<&str, usize> = HashMap::new();
        
        for word in text.split_whitespace() {
            let clean_word = word.trim_matches(|c: char| !c.is_alphabetic());
            if !clean_word.is_empty() {
                *freq_map.entry(clean_word).or_insert(0) += 1;
            }
        }
        
        let mut freq_vec: Vec<(&str, usize)> = freq_map.into_iter().collect();
        // Sort by frequency descending, then alphabetically for stability (if needed)
        // The original code only sorts by frequency, so order for ties is non-deterministic
        freq_vec.sort_by(|a, b| b.1.cmp(&a.1)); 
        freq_vec.truncate(count);
        freq_vec
    }
}

// Function with distinct lifetime parameters for inputs
pub fn compare_texts<'first, 'second>(
    text1: &'first str, 
    text2: &'second str
) -> (&'first str, &'second str, bool) {
    let are_equal = text1 == text2;
    (text1, text2, are_equal)
}

// Struct demonstrating lifetime parameters in method definitions
pub struct TextComparator;

impl TextComparator {
    // Method with multiple lifetime parameters in its signature
    pub fn find_common_prefix<'a, 'b>(&self, s1: &'a str, s2: &'b str) -> (&'a str, &'b str) {
        let mut i = 0;
        let bytes1 = s1.as_bytes();
        let bytes2 = s2.as_bytes();
        
        while i < bytes1.len() && i < bytes2.len() && bytes1[i] == bytes2[i] {
            i += 1;
        }
        
        (&s1[..i], &s2[..i])
    }
}