fn get_count(string: &str) -> usize {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    string.chars().filter(|x| vowels.contains(x)).count()
}

#[test]
fn null() {
    assert_eq!(get_count(""), 0)
}

#[test]
fn zero_vowels() {
    assert_eq!(get_count("qwrtyp"), 0)
}

#[test]
fn only_vowels() {
    assert_eq!(get_count("aeiou"), 5)
}

#[test]
fn test1() {
    assert_eq!(get_count("ala ma kota"), 5)
}

#[test]
fn test2() {
    assert_eq!(get_count("zażółć gęślą jaźń"), 2)
}

fn main() {
    println!("The count equals: {}", get_count("aeiou"));
}