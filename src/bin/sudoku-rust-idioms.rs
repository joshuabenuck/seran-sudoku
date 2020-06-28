use std::collections::{HashMap, HashSet};
use std::env;

#[derive(Clone)]
enum Value {
    Choices(HashSet<char>),
    Given(char),
}

impl Value {
    fn choices_mut(&mut self) -> Option<&mut HashSet<char>> {
        match self {
            Value::Choices(c) => Some(c),
            _ => None,
        }
    }

    fn choices(&self) -> Option<&HashSet<char>> {
        match self {
            Value::Choices(c) => Some(c),
            _ => None,
        }
    }

    fn given(&self) -> Option<char> {
        match self {
            Value::Given(g) => Some(*g),
            _ => None,
        }
    }
}

impl Default for Value {
    fn default() -> Value {
        Value::Choices("123456789".chars().into_iter().collect())
    }
}

trait Rule {}

struct Board {
    values: Vec<Value>,
}

impl Board {
    fn new() -> Board {
        let mut values = Vec::new();
        for _ in 0..82 {
            values.push(Value::default());
        }
        Board { values }
    }

    fn choose(&mut self, i: usize, choice: char) {
        self.values[i] = Value::Given(choice)
    }

    fn givens(&self) -> Vec<char> {
        self.values
            .iter()
            .map(|v| match v {
                Value::Given(g) => *g,
                Value::Choices(_) => '.',
            })
            .collect()
    }
}

fn main() {
    println!(
        r#"
<html>
<head>
<meta name="robots" content="noindex,nofollow">
</head>
<body>
"#
    );

    let script = "sudoku-rust-idioms.html"; //   my $script = $ENV{SCRIPT_NAME}; $script =~ s/.*\///;
    let mut board = Board::new();
    for (i, given) in env::var("QUERY_STRING")
        .unwrap_or_default()
        .chars()
        .enumerate()
    {
        if given == '.' {
            board.choose(i, given);
        }
    }

    let subsets: [[usize; 9]; 27] = [
        // rows
        [0, 1, 2, 9, 10, 11, 18, 19, 20],
        [3, 4, 5, 12, 13, 14, 21, 22, 23],
        [6, 7, 8, 15, 16, 17, 24, 25, 26],
        [27, 28, 29, 36, 37, 38, 45, 46, 47],
        [30, 31, 32, 39, 40, 41, 48, 49, 50],
        [33, 34, 35, 42, 43, 44, 51, 52, 53],
        [54, 55, 56, 63, 64, 65, 72, 73, 74],
        [57, 58, 59, 66, 67, 68, 75, 76, 77],
        [60, 61, 62, 69, 70, 71, 78, 79, 80],
        // columns
        [0, 3, 6, 27, 30, 33, 54, 57, 60],
        [1, 4, 7, 28, 31, 34, 55, 58, 61],
        [2, 5, 8, 29, 32, 35, 56, 59, 62],
        [9, 12, 15, 36, 39, 42, 63, 66, 69],
        [10, 13, 16, 37, 40, 43, 64, 67, 70],
        [11, 14, 17, 38, 41, 44, 65, 68, 71],
        [18, 21, 24, 45, 48, 51, 72, 75, 78],
        [19, 22, 25, 46, 49, 52, 73, 76, 79],
        [20, 23, 26, 47, 50, 53, 74, 77, 80],
        // squares
        [0, 1, 2, 3, 4, 5, 6, 7, 8],
        [9, 10, 11, 12, 13, 14, 15, 16, 17],
        [18, 19, 20, 21, 22, 23, 24, 25, 26],
        [27, 28, 29, 30, 31, 32, 33, 34, 35],
        [36, 37, 38, 39, 40, 41, 42, 43, 44],
        [45, 46, 47, 48, 49, 50, 51, 52, 53],
        [54, 55, 56, 57, 58, 59, 60, 61, 62],
        [63, 64, 65, 66, 67, 68, 69, 70, 71],
        [72, 73, 74, 75, 76, 77, 78, 79, 80],
    ];

    // remove choices eliminated by the at-most-one rule
    for subset in subsets.iter() {
        for i in subset {
            if let Some(d) = board.values[*i].given() {
                for j in subset {
                    if let Some(choices) = board.values[*j].choices_mut() {
                        choices.remove(&d);
                    };
                }
            }
        }
    }

    struct UniqueRule {
        matches: HashMap<char, Matches>,
    }

    impl UniqueRule {
        fn new() -> Self {
            Self {
                matches: HashMap::new(),
            }
        }

        fn add(&mut self, digit: char, index: usize) {
            let entry = self.matches.entry(digit).or_insert(Matches {
                digit,
                indexes: Vec::new(),
            });
            entry.add(digit, index);
        }

        fn uniques(&self) -> Vec<(char, usize)> {
            self.matches
                .iter()
                .filter(|(_, m)| m.indexes.len() == 1)
                .map(|(k, v)| (*k, v.indexes[0]))
                .collect()
        }
    }
    struct Matches {
        digit: char,
        indexes: Vec<usize>,
    }

    impl Matches {
        fn add(&mut self, digit: char, index: usize) {
            self.digit = digit;
            self.indexes.push(index);
        }
    }

    // identify choices mandated by the at-least-one rule
    let mut unique = HashMap::new();
    for subset in subsets.iter() {
        let mut unique_rule = UniqueRule::new();
        for i in subset.into_iter() {
            if let Some(digits) = &board.values[*i].choices() {
                for d in *digits {
                    unique_rule.add(*d, *i);
                }
            }
        }

        // let indexes = subset
        //     .into_iter()
        //     .map(|i| (i, board.values[*i]))
        //     .filter_map(|(i, cs)| match cs.choices() {
        //         Some(cs) => Some((i, cs)),
        //         None => None,
        //     })
        //     .flat_map(|(i, cs)| cs.into_iter().map(|c| (c, i)))
        //     .group_by(|(c, i)| c);
        // let mut counts = [0; 10];
        // let mut where_ = Vec::new();
        // for i in subset.into_iter() {
        //     if let Some(digits) = &board.values[*i].choices() {
        //         for d in *digits {
        //             let d = d.to_digit(10).unwrap() as usize;
        //             counts[d] = counts[d] + 1;
        //             where_[d] = Some(*i);
        //         }
        //     }
        // }
        for (c, i) in unique_rule.uniques() {
            unique.insert(i, c);
        }
    }

    // display board as table of tables of hyperlinked choices
    fn table(first: String) -> String {
        let row = format!(
            "\n<td bgcolor=#77bbff width=170 height=50 align=center>{}",
            first
        );
        let rows: String = (0..3)
            .map(|_: i32| format!("<tr>{}{}{}", row, row, row))
            .collect();
        format!("\n<table>{}</table>", rows)
    }

    let mut b = table(table("X".to_string()));
    for (i, value) in board.values.iter().enumerate() {
        let unique_choice = unique.get(&i);
        let content = match value {
            Value::Given(given) => format!("<font size=8>{}", given),
            Value::Choices(choices) => choices
                .iter()
                // TODO: sort digits...
                .map(|digit| {
                    if unique_choice.is_some() && *digit != *unique_choice.unwrap() {
                        format!("\n<font color=gray size=-1>{}</font>", digit)
                    } else {
                        let mut search = board.givens();
                        search[i] = *digit;
                        let g: String = search.iter().collect();
                        format!("\n<a href=\"{}?{}\">{}</a>", script, g, digit)
                    }
                })
                .collect(),
        };
        b = b.replacen('X', &content, 1);
    }

    println!(
        r#"
  <style>
   a         {{text-decoration: none;}}
  </style>
  <center><br>
    {}
    <font color=gray><br><br>
      About Sudokant, the Sudoku Servant
      <br><table width=400><tr><td><font color=gray>
        This server helps you solve <a href=http://www.sudoku.com/>sudoku puzzles</a>
        by showing choices satisfying two simple rules: <i>at-most-one</i> and <i>at-least-one</i>
        of every digit must appear in every row, column and square.
        Choices that violate the <i>at-most-one</i> rule are omitted.
        Alternatives to digits that must be chosen by the <i>at-least-one</i> rule are grayed.
        <br><br>
        Empty squares are dead ends. Back up the browser
        to make a new choice. The above rules are insufficent to solve the hardest
        puzzles. You can guess or look for a third rule. This program will help you
        find these most interesting places in the hardest puzzles without becomming a
        mechanical master of the simpler logic.
        <br><br>Each step has a unique url that you can bookmark or send to
        friends. Use this <a href={}?{}>blank page</a> to enter new puzzles.
        Contribute your insights and most diabolical puzzles to the
        <a href=http://c2.com/cgi/wiki?SuDoku>SuDoku</a> page on my wiki.
      </table><br>
      <font color=gray size=-1>
        &copy;2005 Ward Cunningham
        &copy;2020 Ward Cunningham, Joshua Benuck, Eric Dobbs, Beth Long
"#,
        b,
        script,
        "................................................................................."
    );
}
