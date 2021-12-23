const _INPUT_PART1: &str = r#"
#############
#...........#
###D#A#C#A###
  #D#C#B#B#
  #########
"#;

const INPUT_PART2: &str = r#"
#############
#...........#
###D#A#C#A###
  #D#C#B#A#
  #D#B#A#C#
  #D#C#B#B#
  #########
"#;

#[derive(Clone, Copy, PartialEq, Eq)]
enum AmphipodColour {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl AmphipodColour {
    fn room_index(&self) -> usize {
        match self {
            AmphipodColour::Amber => 0,
            AmphipodColour::Bronze => 1,
            AmphipodColour::Copper => 2,
            AmphipodColour::Desert => 3,
        }
    }

    fn door_index(&self) -> usize {
        match self {
            AmphipodColour::Amber => 2,
            AmphipodColour::Bronze => 4,
            AmphipodColour::Copper => 6,
            AmphipodColour::Desert => 8,
        }
    }

}

#[derive(Clone, Copy)]
enum MoveState {
    NotMoved,
    Hallway,
    Done
}

impl MoveState {
    fn next(&self) -> MoveState {
        match self {
            MoveState::NotMoved => MoveState::Hallway,
            MoveState::Hallway => MoveState::Done,
            MoveState::Done => panic!("not possible")
        }
    }
}

#[derive(Clone, Copy)]
struct Amphipod {
    colour: AmphipodColour,
    move_state: MoveState
}

impl Amphipod {
    fn from_char(c: char) -> Amphipod {
        Amphipod {
            colour: match c {
                'A' => AmphipodColour::Amber,
                'B' => AmphipodColour::Bronze,
                'C' => AmphipodColour::Copper,
                'D' => AmphipodColour::Desert,
                _ => panic!("unknown amphipod: {}", c)
            },
            move_state: MoveState::NotMoved
        }
    }

    fn score(&self, moves: usize) -> usize {
        moves * match self.colour {
            AmphipodColour::Amber => 1,
            AmphipodColour::Bronze => 10,
            AmphipodColour::Copper => 100,
            AmphipodColour::Desert => 1000,
        }
    }

    fn do_move(&self) -> Amphipod {
        Amphipod {
            colour: self.colour,
            move_state: self.move_state.next()
        }
    }

    fn _print(&self) {
        match self.colour {
            AmphipodColour::Amber => print!("A"),
            AmphipodColour::Bronze => print!("B"),
            AmphipodColour::Copper => print!("C"),
            AmphipodColour::Desert => print!("D"),
        }
    }
}

#[derive(Clone)]
struct State {
    rooms: [[Option<Amphipod>; 4]; 4],
    hallway: [Option<Amphipod>; 11],
    score: usize,
}

impl State {

    fn _assert_sanity(&self) -> bool {
        let (a, b, c, d) = self.hallway.iter()
            .chain(self.rooms[0].iter())
            .chain(self.rooms[1].iter())
            .chain(self.rooms[2].iter())
            .chain(self.rooms[3].iter())
            .fold((0, 0, 0, 0), |(a, b, c, d), n| {
                match n {
                    Some(Amphipod { colour: AmphipodColour::Amber, .. }) =>
                        (a + 1, b, c, d),
                    Some(Amphipod { colour: AmphipodColour::Bronze, .. }) =>
                        (a, b + 1, c, d),
                    Some(Amphipod { colour: AmphipodColour::Copper, .. }) =>
                        (a, b, c + 1, d),
                    Some(Amphipod { colour: AmphipodColour::Desert, .. }) =>
                        (a, b, c, d + 1),
                    None => (a, b, c, d)
                }
            });
        if a != 4 || b != 4 || c != 4 || d != 4 {
            println!("+++++++++ BAD STATE +++++++++");
            self._print();
            false
        } else {
            true
        }
    }

    fn _print(&self) {
        println!("#############");
        print!("#");
        for x in &self.hallway {
            if let Some(amp) = x {
                amp._print();
            } else {
                print!(".");
            }
        }
        println!("#");
        print!("###");
        if let Some(amp) = &self.rooms[0][0] {
            amp._print();
        } else {
            print!(".");
        }
        print!("#");
        if let Some(amp) = &self.rooms[1][0] {
            amp._print();
        } else {
            print!(".");
        }
        print!("#");
        if let Some(amp) = &self.rooms[2][0] {
            amp._print();
        } else {
            print!(".");
        }
        print!("#");
        if let Some(amp) = &self.rooms[3][0] {
            amp._print();
        } else {
            print!(".");
        }
        println!("###");
        print!("  #");
        if let Some(amp) = &self.rooms[0][1] {
            amp._print();
        } else {
            print!(".");
        }
        print!("#");
        if let Some(amp) = &self.rooms[1][1] {
            amp._print();
        } else {
            print!(".");
        }
        print!("#");
        if let Some(amp) = &self.rooms[2][1] {
            amp._print();
        } else {
            print!(".");
        }
        print!("#");
        if let Some(amp) = &self.rooms[3][1] {
            amp._print();
        } else {
            print!(".");
        }
        println!("#");
        print!("  #");
        if let Some(amp) = &self.rooms[0][2] {
            amp._print();
        } else {
            print!(".");
        }
        print!("#");
        if let Some(amp) = &self.rooms[1][2] {
            amp._print();
        } else {
            print!(".");
        }
        print!("#");
        if let Some(amp) = &self.rooms[2][2] {
            amp._print();
        } else {
            print!(".");
        }
        print!("#");
        if let Some(amp) = &self.rooms[3][2] {
            amp._print();
        } else {
            print!(".");
        }
        println!("#");
        print!("  #");
        if let Some(amp) = &self.rooms[0][3] {
            amp._print();
        } else {
            print!(".");
        }
        print!("#");
        if let Some(amp) = &self.rooms[1][3] {
            amp._print();
        } else {
            print!(".");
        }
        print!("#");
        if let Some(amp) = &self.rooms[2][3] {
            amp._print();
        } else {
            print!(".");
        }
        print!("#");
        if let Some(amp) = &self.rooms[3][3] {
            amp._print();
        } else {
            print!(".");
        }
        println!("#");
        println!("  #########");
    }

    fn from_input(input: &str) -> State {
        let input = input.as_bytes();
        let room_a = [input[32], input[46], input[58], input[70]]
            .map(|c| Some(Amphipod::from_char(c as char)));
        let room_b = [input[34], input[48], input[60], input[72]]
            .map(|c| Some(Amphipod::from_char(c as char)));
        let room_c = [input[36], input[50], input[62], input[74]]
            .map(|c| Some(Amphipod::from_char(c as char)));
        let room_d = [input[38], input[52], input[64], input[76]]
            .map(|c| Some(Amphipod::from_char(c as char)));
        State {
            rooms: [room_a, room_b, room_c, room_d],
            hallway: [None; 11],
            score: 0,
        }
    }

    fn solve(self) -> usize {
        let mut lowest_score = usize::MAX;
        self.iterate(&mut lowest_score);
        lowest_score
    }

    fn iterate(self, lowest_score: &mut usize) {
        // try move things in the hallway
        for i in 0..11 {
            if let Some(amp) = &self.hallway[i] {
                let r = amp.colour.room_index();
                if let Some(idx) = can_move_to_room(&self.rooms[r], amp.colour) {
                    // check if it's not blocked and get the distance to the door
                    if let Some(dist) = self.get_dist_to_hallway_idx(i, amp.colour.door_index()) {
                        let moves = dist + (4 - idx);
                        let add_score = amp.score(moves);
                        if self.score + add_score < *lowest_score {
                            let mut next = self.clone();
                            next.rooms[r][idx] = Some(amp.do_move());
                            next.hallway[i] = None;
                            next.score += add_score;
                            if next.check_win() {
                                println!("{}", next.score);
                                *lowest_score = next.score;
                            } else {
                                next.iterate(lowest_score);
                            }
                        }
                    }
                }
            }
        }
        // try move things in the rooms
        for r in 0..4 {
            for i in 0..4 {
                if let Some(amp) = &self.rooms[r][i] {
                    // if this is already in position, there's
                    // nothing else to do for the whole room
                    if !matches!(amp.move_state, MoveState::NotMoved) {
                        break;
                    }
                    // can it move to the room
                    let extra_dist = i + 1;
                    let pos = r * 2 + 2;
                    let target_room_idx = amp.colour.room_index();
                    if let Some(idx) = can_move_to_room(&self.rooms[target_room_idx], amp.colour) {
                        // check if it's not blocked and get the distance to the door
                        if let Some(dist) = self.get_dist_to_hallway_idx(pos, amp.colour.door_index()) {
                            let moves = dist + (4 - idx) + extra_dist;
                            let add_score = amp.score(moves);
                            if self.score + add_score < *lowest_score {
                                let mut next = self.clone();
                                next.rooms[target_room_idx][idx] = Some(amp.do_move());
                                next.rooms[r][i] = None;
                                next.score += add_score;
                                if next.check_win() {
                                    println!("{}", next.score);
                                    *lowest_score = next.score;
                                } else {
                                    next.iterate(lowest_score);
                                }
                            }
                        }
                    }
                    // move it to every possible spot in the hallway
                    for hallway_idx in 0..11 {
                        if hallway_idx > 0 && hallway_idx < 10 && hallway_idx % 2 == 0 {
                            // we don't move to doorways
                            continue;
                        }
                        if self.hallway[hallway_idx].is_some() {
                            // we can't move to spots already taken
                            continue;
                        }
                        if let Some(dist) = self.get_dist_to_hallway_idx(pos, hallway_idx) {
                            let moves = dist + extra_dist;
                            let add_score = amp.score(moves);
                            if self.score + add_score < *lowest_score {
                                let mut next = self.clone();
                                next.rooms[r][i] = None;
                                next.hallway[hallway_idx] = Some(amp.do_move());
                                next.score += add_score;
                                if next.check_win() {
                                    println!("{}", next.score);
                                    *lowest_score = next.score;
                                } else {
                                    next.iterate(lowest_score);
                                }
                            }
                        }
                    }

                    // we can only move the top item, so we're done
                    break;
                }
            }
        }
    }

    fn get_dist_to_hallway_idx(&self, pos: usize, target_idx: usize) -> Option<usize> {
        let (s, e, r) = if pos < target_idx {
            (pos+1, target_idx, target_idx - pos)
        } else {
            (target_idx+1, pos, pos - target_idx)
        };
        for hallway_idx in s..e {
            if hallway_idx > 0 && hallway_idx < 10 && hallway_idx % 2 == 0 {
                continue;
            } else if self.hallway[hallway_idx].is_some() {
                return None;
            }
        }
        Some(r)
    }

    fn check_win(&self) -> bool {
        for (room, colour) in [
            (&self.rooms[0], AmphipodColour::Amber),
            (&self.rooms[1], AmphipodColour::Bronze),
            (&self.rooms[2], AmphipodColour::Copper),
            (&self.rooms[3], AmphipodColour::Desert),
        ] {
            for amp in room.iter() {
                if let Some(amp) = amp {
                    if amp.colour != colour {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        true
    }
}

fn can_move_to_room(room: &[Option<Amphipod>; 4], colour: AmphipodColour) -> Option<usize> {
    if room[3].is_none() {
        Some(3)
    } else if colour_match(&room[3], colour) {
        if room[2].is_none() {
            Some(2)
        } else if colour_match(&room[2], colour) {
            if room[1].is_none() {
                Some(1)
            } else if colour_match(&room[1], colour) {
                if room[0].is_none() {
                    Some(0)
                } else {
                    // if not empty there's another color here
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn colour_match(room: &Option<Amphipod>, colour: AmphipodColour) -> bool {
    if let Some(amp) = room {
        amp.colour == colour
    } else {
        false
    }
}

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

    let state = State::from_input(INPUT_PART2);
    let part2 = state.solve();

    println!("THIS IS NEVER ENDING, BUT WILL GET THE LIKELY SOLUTION.. I'm too lazy to figure out how to make it run FAST");
    println!("part2: {}", part2);
}
