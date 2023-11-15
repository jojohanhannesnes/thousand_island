fn special_pythagorean_triplet() -> i32 {
    for a in 1..1000 {
        for b in a + 1..1000 {
            let c = 1000 - (a + b);
            if a * a + b * b == c * c {
                return a * b * c;
            }
        }
    }
    panic!("cannot find the combination");
}

#[test]
fn test() {
    assert_eq!(special_pythagorean_triplet(), 31875000);
}
