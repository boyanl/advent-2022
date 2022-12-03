use std::{io::{self}};
use std::collections::HashMap;

fn part_one() {
   let scores = HashMap::from([(("A", "X"), 3), (("A", "Y"), 6), (("A", "Z"), 0),
                               (("B", "X"), 0), (("B", "Y"), 3), (("B", "Z"), 6),
                               (("C", "X"), 6), (("C", "Y"), 0), (("C", "Z"), 3)
   ]);
   let points = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
   
   let mut score = 0;
   for line in io::stdin().lines().map(|l| l.unwrap()) {
       let parts: Vec<&str> = line.split_ascii_whitespace().collect();
       let (opponent, me) = (parts[0], parts[1]);
       score += points[me] + scores[&(opponent, me)];
   }
   
   println!("{score}")
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Shape {
    Rock, Paper, Scissors
}

fn part_two() {
   let scores = HashMap::from([((Shape::Rock, Shape::Rock), 3), ((Shape::Rock, Shape::Paper), 6), ((Shape::Rock, Shape::Scissors), 0),
                               ((Shape::Paper, Shape::Rock), 0), ((Shape::Paper, Shape::Paper), 3), ((Shape::Paper, Shape::Scissors), 6),
                               ((Shape::Scissors, Shape::Rock), 6), ((Shape::Scissors, Shape::Paper), 0), ((Shape::Scissors, Shape::Scissors), 3)]);
   
    let points_per_shape = HashMap::from([(Shape::Rock, 1), (Shape::Paper, 2), (Shape::Scissors, 3)]);

   let mut score = 0;
   for line in io::stdin().lines().map(|l| l.unwrap()) {
       let parts: Vec<&str> = line.split_ascii_whitespace().collect();
       let (opponent_str, result_str) = (parts[0], parts[1]);
       let opponent = match opponent_str {
           "A" => Shape::Rock,
           "B" => Shape::Paper,
           "C" => Shape::Scissors,
           &_ => todo!()
       };
       let points = match result_str {
           "X" => 0,
           "Y" => 3,
           "Z" => 6,
           &_ => todo!()
       };
       let shape = scores.iter().filter_map(|((opp, me), &score)| {
           if *opp == opponent && score == points {Some(me)} else {None} 
       }).next().unwrap();
       
       score += points + points_per_shape[shape];
   }
   
   println!("{score}")
}

fn main() {
    part_two()
}