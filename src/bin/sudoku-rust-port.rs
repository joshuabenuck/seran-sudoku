use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::mem::replace;

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

    let script = "sudoku-rust-port.html"; //   my $script = $ENV{SCRIPT_NAME}; $script =~ s/.*\///;
    let blank = ".................................................................................";
    let givens_string: String = env::var("QUERY_STRING").unwrap_or_default() + blank;
    let givens: Vec<char> = givens_string.chars().into_iter().take(81).collect();
    let mut choices: Vec<Cow<str>> = (0..81).map(|_| Cow::from("123456789")).collect();

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
            let d = givens[*i];
            if d == '.' {
                continue;
            }
            for j in subset {
                let c = choices[*j].to_mut();
                replace(c, c.replace(d, ""));
            }
        }
    }

    // identify choices mandated by the at-least-one rule
    let digit_counts = "123456789"
        .chars()
        .into_iter()
        .fold(HashMap::new(), |mut acc, d| {
            acc.insert(d, 0);
            acc
        });
    let mut unique = HashMap::new();
    for subset in subsets.iter() {
        let mut counts = digit_counts.clone();
        let mut where_ = digit_counts.clone();
        for i in subset.into_iter().filter(|i| givens[**i] == '.') {
            let digits = &choices[*i];
            for d in digits.chars().into_iter() {
                let count = counts.get_mut(&d).unwrap();
                replace(count, *count + 1);
                where_.insert(d, *i);
            }
        }
        for d in "123456789".chars().into_iter().filter(|d| counts[d] == 1) {
            let w = where_.get(&d);
            unique.insert(w.unwrap().clone(), d);
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

    let mut board = table(table("X".to_string()));

    fn next_givens(givens: &Vec<char>, i: usize, digit: char) -> Vec<char> {
        let mut search = givens.clone();
        search[i] = digit;
        search
    }

    fn choices_fn(
        script: &str,
        givens: &Vec<char>,
        choices: &Vec<Cow<str>>,
        unique: &HashMap<usize, char>,
        i: usize,
    ) -> String {
        let unique_choice = unique.get(&i);
        choices[i]
            .chars()
            .map(|digit| {
                if unique_choice.is_some() && digit != *unique_choice.unwrap() {
                    format!("\n<font color=gray size=-1>{}</font>", digit)
                } else {
                    let search = next_givens(givens, i, digit);
                    let g: String = search.iter().collect();
                    format!("\n<a href=\"{}?{}\">{}</a>", script, g, digit)
                }
            })
            .collect()
    }

    for i in 0..81 {
        let digit = givens[i];
        let content = if digit != '.' {
            format!("<font size=8>{}", digit)
        } else {
            choices_fn(script, &givens, &choices, &unique, i)
        };
        board = board.replacen('X', &content, 1);
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
        board, script, blank
    );
}
