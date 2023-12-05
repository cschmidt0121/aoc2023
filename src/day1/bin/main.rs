use utils::read_lines;

fn main() {
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut running_total: i32 = 0;
    println!("Part 1");
    let lines: Vec<String> =  read_lines("./inputs/input_day1.txt"); 
    for line in lines {
        let digits: Vec<i32> = line.chars()
        .filter_map(|x| x.to_digit(10).map(|x| x as i32))
        .collect();
        running_total += digits[0]*10 + digits[digits.len() - 1];
    }
    
    println!("{}", running_total);

    // Part 2
    let mut running_total_pt2: u32 = 0;
    println!("Part 2");
    let lines: Vec<String> =  read_lines("./inputs/input_day1.txt"); 

    for mut line in lines {
        // Shift start pointer of line until it's a number (word or digit)
        while !nums.iter().any(|x| line.starts_with(x)) && !line.chars().next().unwrap().is_numeric()
        {
            line = line[1..].to_string();
        }

        // Trim string until the end of the string is a number (word or digit)
        while !nums.iter().any(|x| line.ends_with(x)) && !line.chars().last().unwrap().is_numeric()
        {
            line.pop();
        }

        let mut first_num: u32 = 0;
        let mut last_num: u32 = 0;

        // If the first char is a digit, just grab it and conert to a u32
        if line.chars().next().unwrap().is_numeric() {
            first_num = line.chars().next().unwrap().to_digit(10).unwrap();
        }
        // Otherwise find it in nums. Index+1 = the actual value
        else {
            for (i, num) in nums.iter().enumerate() {
                if line.starts_with(num) {
                    first_num = (i as u32)+1;
                }
            }
        }

        // Now do it again for the last number
        if line.chars().last().unwrap().is_numeric() {
            last_num = line.chars().last().unwrap().to_digit(10).unwrap();
        }
        else {
            for (i, num) in nums.iter().enumerate() {
                if line.ends_with(num) {
                    last_num = (i as u32)+1;
                }
            }
        }

        running_total_pt2 += first_num*10 + last_num

    }
    println!("{}", running_total_pt2);
}
