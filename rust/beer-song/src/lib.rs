pub fn verse(n: i32) -> String {
    let bottles = |n| match n {
        0 => format!("no more bottles"),
        1 => format!("1 bottle"),
        n => format!("{} bottles", n),
    };

    match n {
        0 => format!(
            "No more bottles of beer on the wall, no more bottles of beer.\n\
             Go to the store and buy some more, 99 bottles of beer on the wall.\n"
        ),
        n => format!(
            "{bottles} of beer on the wall, {bottles} of beer.\n\
             Take {one} down and pass it around, {one_fewer_bottles} of beer on the wall.\n",
            bottles = bottles(n),
            one = if n > 1 { "one" } else { "it" },
            one_fewer_bottles = bottles(n - 1)
        ),
    }
}

pub fn sing(start: i32, end: i32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<String>>()
        .join("\n")
}
