fn main() {
    let s = "1234".to_string();
    let no1 = &s[0..1];
    let no2 = &s[1..2];
    let no3 = &s[2..3];
    let no4 = &s[3..4];
    println!("{} {} {} {}", no1, no2, no3, no4)
}
