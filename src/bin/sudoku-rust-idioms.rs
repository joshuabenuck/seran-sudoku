use clap::{App, Arg};
use std::env;

#[derive(Clone, PartialEq)]
enum Rule {
    AtMostOne,
    AtLeastOne,
}

#[derive(Clone)]
struct Choice {
    digit: char,
    reason: Option<Rule>,
}

#[derive(Clone)]
enum Value {
    Choices(Vec<Choice>),
    Given(char),
}

impl Value {
    fn choices(&self) -> Option<Vec<char>> {
        match self {
            Value::Choices(cs) => Some(
                cs.iter()
                    .filter_map(|c| {
                        if c.reason.is_none() {
                            Some(c.digit)
                        } else {
                            None
                        }
                    })
                    .collect(),
            ),
            _ => None,
        }
    }

    fn count(&self, rule: &Rule) -> usize {
        if let Value::Choices(choices) = self {
            return choices.iter().fold(0, |a, c| {
                if c.reason.is_some() && c.reason.as_ref().unwrap() == rule {
                    return a + 1;
                }
                a
            });
        }
        0
    }

    fn given(&self) -> Option<char> {
        match self {
            Value::Given(g) => Some(*g),
            _ => None,
        }
    }

    fn eliminate(&mut self, digit: char, rule: Rule) {
        if let Value::Choices(ref mut cs) = self {
            let index = cs
                .iter()
                .enumerate()
                .filter(|(_i, c)| c.reason.is_none())
                .fold(None, |a, (i, c)| if c.digit == digit { Some(i) } else { a });
            if index.is_some() {
                cs[index.unwrap()].reason = Some(rule);
            }
        }
    }

    fn keep(&mut self, digit: char, rule: Rule) {
        if let Value::Choices(ref mut cs) = self {
            for choice in cs {
                if choice.reason.is_none() && choice.digit != digit {
                    choice.reason = Some(rule.clone());
                }
            }
        }
    }
}

impl Default for Value {
    fn default() -> Value {
        Value::Choices(
            "123456789"
                .chars()
                .into_iter()
                .map(|d| Choice {
                    digit: d,
                    reason: None,
                })
                .collect(),
        )
    }
}

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

    fn choose_all(&mut self, givens: &str) {
        for (i, given) in givens.chars().enumerate() {
            if given != '.' {
                self.choose(i, given);
            }
        }
    }

    fn choose(&mut self, i: usize, choice: char) {
        self.values[i] = Value::Given(choice)
    }

    fn count(&self, rule: &Rule) -> usize {
        self.values.iter().fold(0, |a, v| a + v.count(rule))
    }

    fn indexes_of<'a>(
        &'a self,
        subset: &'a [usize; 9],
        digit: char,
    ) -> impl Iterator<Item = usize> + 'a {
        subset
            .iter()
            .map(move |i| (*i, &self.values[*i]))
            .filter_map(|(u, v)| match v.choices() {
                Some(v) => Some((u, v)),
                None => None,
            })
            .filter_map(move |(u, c)| match c.binary_search(&digit) {
                Ok(_) => Some(u),
                Err(_) => None,
            })
    }

    fn eliminate(&mut self, indexes: Vec<usize>, digit: char, rule: Rule) {
        for index in indexes {
            self.values[index].eliminate(digit, rule.clone());
        }
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

    fn rows(&self) -> [[usize; 9]; 9] {
        [
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
        ]
    }

    fn cols(&self) -> [[usize; 9]; 9] {
        [
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
        ]
    }

    fn squares(&self) -> [[usize; 9]; 9] {
        [
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
        ]
    }

    fn subsets(&self) -> Vec<[usize; 9]> {
        [self.rows(), self.cols(), self.squares()].concat()
    }

    fn eval(&mut self) {
        // remove choices eliminated by the at-most-one rule
        for subset in self.subsets().iter() {
            for i in subset {
                if let Some(d) = self.values[*i].given() {
                    let indexes = self.indexes_of(&subset, d).collect();
                    self.eliminate(indexes, d, Rule::AtMostOne);
                }
            }
        }

        // identify choices mandated by the at-least-one rule
        for subset in self.subsets().iter() {
            for digit in "0123456789".chars() {
                let mut indexes = self.indexes_of(&subset, digit);
                let possibly_unique = indexes.next();
                if possibly_unique.is_some() && indexes.next().is_none() {
                    drop(indexes);
                    let possibly_unique = possibly_unique.unwrap();
                    self.values[possibly_unique].keep(digit, Rule::AtLeastOne);
                }
            }
        }
    }
    fn solve(&mut self) {
        let mut count_rule1;
        let mut count_rule2;
        for _ in 0..10 {
            let decided: Vec<&mut Value> = self
                .values
                .iter_mut()
                .filter(|v| v.choices().is_some())
                .filter(|v| v.choices().unwrap().len() == 1)
                .collect();
            for value in decided {
                let digit = value.choices().unwrap()[0];
                std::mem::replace(value, Value::Given(digit));
            }
            self.eval();
            count_rule1 = self.count(&Rule::AtMostOne);
            count_rule2 = self.count(&Rule::AtLeastOne);
            println!("{} {}", count_rule1, count_rule2);
            if count_rule1 == 0 && count_rule2 == 0 {
                break;
            }
        }
    }
}

fn html(board: &Board) {
    let script = "sudoku-rust-idioms.html"; //   my $script = $ENV{SCRIPT_NAME}; $script =~ s/.*\///;

    println!(
        r#"
<html>
<head>
<meta name="robots" content="noindex,nofollow">
</head>
<body>
"#
    );

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
        let content = match value {
            Value::Given(given) => format!("<font size=8>{}", given),
            Value::Choices(choices) => choices
                .iter()
                .map(|choice| match choice.reason {
                    Some(Rule::AtLeastOne) => {
                        format!("\n<font color=gray size=-1>{}</font>", choice.digit)
                    }
                    Some(Rule::AtMostOne) => {
                        //format!("\n<font color=#77bbff size=-1>{}</font>", choice.digit)
                        format!("")
                    }
                    _ => {
                        let mut search = board.givens();
                        search[i] = choice.digit;
                        let g: String = search.iter().collect();
                        format!("\n<a href=\"{}?{}\">{}</a>", script, g, choice.digit)
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

fn terminal(board: &Board) {
    let mut rows = "123456789".chars();
    let cols = "123456789".chars();
    print!("  ");
    for (i, col) in cols.enumerate() {
        print!("{}", col);
        if i != 0 && (i + 1) % 3 == 0 {
            print!(" ")
        }
    }
    println!("");
    for (i, row) in board.rows().iter().enumerate() {
        print!("{} ", rows.next().unwrap());
        for (j, col) in row.iter().enumerate() {
            match &board.values[*col] {
                Value::Choices(_choices) => {
                    print!("?");
                }
                Value::Given(choice) => {
                    print!("{}", choice);
                }
            }
            if j != 0 && (j + 1) % 3 == 0 {
                print!(" ");
            }
        }
        println!("");
        if i != 1 && (i + 1) % 3 == 0 {
            println!("");
        }
    }
}

fn main() {
    let matches = App::new("sudoku-rust-idioms")
        .arg(
            Arg::with_name("terminal")
                .long("terminal")
                .short("t")
                .takes_value(true),
        )
        .arg(Arg::with_name("solve").long("solve").short("solve"))
        .get_matches();

    let mut board = Board::new();

    let (givens, present): (String, fn(&Board)) = if matches.is_present("terminal") {
        (matches.value_of("terminal").unwrap().to_owned(), terminal)
    } else {
        (env::var("QUERY_STRING").unwrap_or_default(), html)
    };
    board.choose_all(&givens);
    if matches.is_present("solve") {
        board.solve();
    } else {
        board.eval();
    }
    present(&board);
}
