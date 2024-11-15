fn main() {
    for diamond_size in 1..=10 {
        print_diamonds(diamond_size, false);
        println!("\n");
        print_diamonds(diamond_size, true);
        println!("\n")
    }
}
fn print_diamonds(size: usize, filled: bool) {
    let top = format_half(size, filled, true);
    let bot = format_half(size, filled, false);
    for i in 0..top.len() {
        print!("{}", top[i]);
    }
    for i in 0..bot.len() {
        print!("{}", bot[i]);
    }
}
fn format_half(size: usize, filled: bool, top: bool) -> Vec<String> {
    let mut rows: Vec<String> = vec![];
    for row in 1..=size {
        let multiplier = if !filled {
            1
        } else if top {
            row
        } else {
            size - row + 1
        };
        let left_width = if filled {
            size
        } else if top {
            size - row + 1
        } else {
            row
        };
        let right_width = if filled && top {
            row
        } else if !filled && top {
            (row * 2) - 1
        } else if filled && !top {
            size - row + 1
        } else {
            (size - row) * 2 + 1
        };
        let left_char = if top {
            "/".to_string()
        } else {
            "\\".to_string()
        };
        let right_char = if top {
            "\\".to_string()
        } else {
            "/".to_string()
        };
        rows.push(format!(
            "{}{}\n",
            format_side(multiplier, left_width, left_char),
            format_side(multiplier, right_width, right_char)
        ))
    }
    rows
}

fn format_side(multiplier: usize, width: usize, character: String) -> String {
    format!("{: >width$}", character.repeat(multiplier), width = width)
}
