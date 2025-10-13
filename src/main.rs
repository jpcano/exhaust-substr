use exhaust_substr::*;

fn main() {
    let s3 = "abc";
    let n3 = 2;
    let result3 = split_into_n_substrings(s3, n3);
    println!("String: '{}', N: {}", s3, n3);
    println!("Number of ways: {}", result3.len());
    for (i, split) in result3.iter().enumerate() {
        println!("  {}. {:?}", i + 1, split);
    }
}
