use std::str::FromStr;

fn main() {
    let line =
        "x (A) 2016-05-20 2016-04-30 measure space for +chapelShelving @chapel due:2016-05-30";
    let task = todo_txt::task::Simple::from_str(line);

    println!("{task:#?}");
}
