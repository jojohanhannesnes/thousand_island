pub fn largest_palindrome_3_digit() -> i32 {
    let mut test: Vec<i32> = Vec::new();
    for i in 900..=999 {
        for y in i..=999 {
            let x = (i * y).to_string();
            let val: Vec<char> = x.chars().collect();
            if val[0..=val.len() / 2 - 1]
                == val[val.len() / 2..]
                    .iter()
                    .rev()
                    .cloned()
                    .collect::<Vec<char>>()
            {
                println!("{val:?}");
                test.push(i * y);
            }
        }
    }
    test.into_iter().max().unwrap()
}

#[test]
fn it_works() {
    assert_eq!(largest_palindrome_3_digit(), 906609);
}
