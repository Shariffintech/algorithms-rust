// To report the words of string s that are missing in string t (in the order they are missing), you can use the following algorithm:

// Initialize a variable, "result" as an empty list.
// Split both strings s and t into lists of words using the string method "split()" with a delimiter of " " (space).
// Initialize a variable, "i" as 0, to keep track of the current index in the list of words from string s.
// Iterate through the list of words from string t. For each word in the list:
// a. Check if the current word from string t is equal to the word at the current index "i" in the list of words from string s.
// b. If they are equal, increment the value of "i" by 1.
// c. If they are not equal, add the word from string s at the current index "i" to the "result" list and increment the value of "i" by 1.
// Once the iteration through the list of words from string t is complete, add any remaining words from string s to the "result" list.
// return the "result" list which will have the words of s, missing in t (case sensitive) in the order they are missing.
// It's important to note that this algorithm assumes that the string t is a subsequence of string s. In case if not you need to check it before running the algorithm.


pub fn missing_words(s: &str, t: &str) -> Vec<&str> {
    let s_words = s.split(" ").collect::<Vec<&str>>();
    let t_words = t.split(" ").collect::<Vec<&str>>();

    let mut result = vec![];
    let mut i = 0;

    for t_word in t_words {
        while i < s_words.len() && s_words[i] != t_word {
            result.push(s_words[i]);
            i += 1;
        }
        i += 1;
    }

    while i < s_words.len() {
        result.push(s_words[i]);
        i += 1;
    }

    result
}

