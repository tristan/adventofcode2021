fn main() {
    let input = vec![4,3,3,5,4,1,2,1,3,1,1,1,1,1,2,4,1,3,3,1,1,1,1,2,3,1,1,1,4,1,1,2,1,2,2,1,1,1,1,1,5,1,1,2,1,1,1,1,1,1,1,1,1,3,1,1,1,1,1,1,1,1,5,1,4,2,1,1,2,1,3,1,1,2,2,1,1,1,1,1,1,1,1,1,1,4,1,3,2,2,3,1,1,1,4,1,1,1,1,5,1,1,1,5,1,1,3,1,1,2,4,1,1,3,2,4,1,1,1,1,1,5,5,1,1,1,1,1,1,4,1,1,1,3,2,1,1,5,1,1,1,1,1,1,1,5,4,1,5,1,3,4,1,1,1,1,2,1,2,1,1,1,2,2,1,2,3,5,1,1,1,1,3,5,1,1,1,2,1,1,4,1,1,5,1,4,1,2,1,3,1,5,1,4,3,1,3,2,1,1,1,2,2,1,1,1,1,4,5,1,1,1,1,1,3,1,3,4,1,1,4,1,1,3,1,3,1,1,4,5,4,3,2,5,1,1,1,1,1,1,2,1,5,2,5,3,1,1,1,1,1,3,1,1,1,1,5,1,2,1,2,1,1,1,1,2,1,1,1,1,1,1,1,3,3,1,1,5,1,3,5,5,1,1,1,2,1,2,1,5,1,1,1,1,2,1,1,1,2,1];

    // The "I'm not good at math" version

    // calculate a cache of how many fish an adult fish
    // will produce over it's lifetime (including itself)
    // using the simple iterative approach.
    // a fish is considered an adult fish after it's
    // spawned one fish. all input fish are considered young
    // fish to start with.
    let mut fish = vec![6];
    let mut new_fish = vec![];
    let mut fish_count_days = vec![1];

    for _ in 1..=256 {
        for f in &mut fish {
            if *f == 0 {
                *f = 6;
                new_fish.push(8);
            } else {
                *f -= 1;
            }
        }
        if !new_fish.is_empty() {
            fish.append(&mut new_fish);
        }
        fish_count_days.push(fish.len());
    }

    // iterate over the fish for the number of days
    // but using the cache to fill in the amount of
    // fish each adult fish will produce for the
    // remaining time left.
    let mut fish = input.clone();
    let mut new_fish_count = 0;
    for days_remaining in (0..80).rev() {
        fish = fish.into_iter()
            .map(|f| {
                if f == 0 {
                    // this fish becomes an adult!
                    // remove it from the fish list and add the number
                    // of fish it will be responsbile for producting
                    // for the remaining days
                    new_fish_count += fish_count_days[days_remaining];
                    // replace the adult fish with a young fish
                    8
                } else {
                    // decrease the young fish's time to become an adult
                    f - 1
                }
            })
            .collect();
    }
    new_fish_count += fish.len();
    println!("part1: {}", new_fish_count);

    let mut fish = input;
    let mut new_fish_count = 0;
    for days_remaining in (0..256).rev() {
        fish = fish.into_iter()
            .map(|f| {
                if f == 0 {
                    new_fish_count += fish_count_days[days_remaining];
                    8
                } else {
                    f - 1
                }
            })
            .collect();
    }
    new_fish_count += fish.len();
    println!("part2: {}", new_fish_count);
}
