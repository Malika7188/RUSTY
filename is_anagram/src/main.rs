use is_anagram::is_anagram;

fn main() {
    let s1 = "listen";
    let s2 = "silent";

    if is_anagram(s1, s2) {
        println!("{} and {} are anagrams!", s1, s2);
    } else {
        println!("{} and {} are not anagrams.", s1, s2);
    }
}