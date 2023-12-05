use utils::read_lines;

fn parse_handful(handful: &str) -> (u32, u32, u32)
{
    let groups = handful.split(", ");
    let mut num_red = 0;
    let mut num_green = 0;
    let mut num_blue = 0; 
    for group in groups {
        let num_color: Vec<&str> = group.split(" ").collect();
        let number: u32 = num_color[0].parse().expect("Unable to parse num marbles");
        let color: &str = num_color[1];
        match color {
            "red" => num_red = number,
            "green" => num_green = number,
            "blue" => num_blue = number,
            _ => unreachable!()
        }
    }
    return (num_red, num_green, num_blue);
}
fn main() {
    println!("Part 1");
    let lines: Vec<String> =  read_lines("./inputs/input_day2.txt"); 
    let inventory_red: u32 = 12;
    let inventory_green: u32 = 13;
    let inventory_blue: u32 = 14;
    let mut game_id_sum: u32 = 0;

    for line in lines.iter() {
        let mut split = line.split("Game ");
        split.next();
        let mut rest_of_line: &str = split.next().expect("Uh-oh");
        let game_id: u32 = rest_of_line.to_string().split(":").next().unwrap().parse().unwrap();
        let mut game_is_valid = true;
        // Trim off the game ID and other superfluous chars
        let colon_index = rest_of_line.to_string().find(":").unwrap();
        rest_of_line = &rest_of_line[colon_index+2..];
        let handfuls = rest_of_line.split(";");
        for handful in handfuls {
            let (num_red, num_green, num_blue);
            (num_red, num_green, num_blue) = parse_handful(handful.trim());
            if num_red > inventory_red || num_green > inventory_green || num_blue > inventory_blue {
                game_is_valid = false; 
                break;
            }
        }
        if game_is_valid {
            game_id_sum += game_id;
        }    
    }
    println!("{}", game_id_sum);

    println!("Part 2");
    let mut total_power: u32 = 0;
    for line in lines.iter() {
        let colon_index = line.find(":").unwrap();
        let rest_of_line = &line[colon_index+2..];
        let handfuls = rest_of_line.split(";");

        let mut red_nums: Vec<u32> = Vec::new();
        let mut green_nums: Vec<u32> = Vec::new();
        let mut blue_nums: Vec<u32> = Vec::new();

        for handful in handfuls {
            let (num_red, num_green, num_blue);
            (num_red, num_green, num_blue) = parse_handful(handful.trim());
            red_nums.push(num_red);
            green_nums.push(num_green);
            blue_nums.push(num_blue);
        }
        let max_r = red_nums.iter().max().unwrap();
        let max_g = green_nums.iter().max().unwrap();
        let max_b = blue_nums.iter().max().unwrap();
        let power = max_r * max_g * max_b;

        total_power+=power;
    

    }
    println!("{}", total_power);


    
    
}
