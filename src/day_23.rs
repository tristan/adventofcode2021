const INPUT: &str = r#"
#############
#...........#
###D#A#C#A###
  #D#C#B#B#
  #########
"#;

fn main() {
    let a = 1;
    let b = 10;
    let c = 100;
    let d = 1000;

    let part1 =
        a * 4 +
        a * 2 +
        c * 2 +
        b * 5 +
        c * 6 +
        c * 2 +
        b * 3 +
        b * 7 +
        d * 9 +
        d * 9 +
        a * 3 +
        a * 8;

    println!("part1: {}", part1);

}
