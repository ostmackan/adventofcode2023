use std::str::FromStr;

pub struct GamePull {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

impl GamePull {
    pub fn is_correct(&self, red_max: i32, green_max: i32, blue_max: i32) -> bool {
        return red_max >= self.red && green_max >= self.green && blue_max >= self.blue;
    }

    pub fn score(&self) -> i32 {
        return self.red * self.green * self.blue;
    }
}

pub struct GameInstance {
    pub id: i32,
    pub pulls: Vec<GamePull>,
}

impl GameInstance {
    pub fn is_correct(&self, red_max: i32, green_max: i32, blue_max: i32) -> bool {
        return self
            .pulls
            .iter()
            .all(|x| x.is_correct(red_max, green_max, blue_max));
    }

    // Return a new GamePull represnting the max of all colors.
    pub fn max_pull(&self) -> GamePull {
        let red = self.pulls.iter().max_by_key(|x| x.red);
        let green = self.pulls.iter().max_by_key(|x| x.green);
        let blue = self.pulls.iter().max_by_key(|x| x.blue);

        if red.is_none() || green.is_none() || blue.is_none() {
            return GamePull {
                red: 0,
                green: 0,
                blue: 0,
            };
        }

        return GamePull {
            red: red.unwrap().red,
            green: green.unwrap().green,
            blue: blue.unwrap().blue,
        };
    }
}

pub fn get_games(file_contents: String, games: &mut Vec<GameInstance>) {
    let rows = file_contents.split("\n");

    for row in rows {
        let semi_split = row.split(':');
        let mut instance: GameInstance = GameInstance {
            id: 0,
            pulls: Vec::new(),
        };

        for semi in semi_split {
            if semi.starts_with("Game") {
                let game_number = semi.replace("Game", "");
                println!("{game_number}");

                let game_id = i32::from_str(game_number.as_str().trim());

                if game_id.is_ok() {
                    instance.id = game_id.unwrap();
                } else {
                    println!("error {}", game_number)
                }
            } else {
                let pulls = semi.split(";");
                for pull in pulls {
                    let mut pullish = GamePull {
                        red: 0,
                        green: 0,
                        blue: 0,
                    };

                    println!("{pull}");
                    let colors = pull.split(',');

                    for color in colors {
                        let color_split = color.split(" ");
                        let mut value = 0;
                        let mut color_name = "";

                        for color_raw in color_split {
                            if color_raw.find(char::is_numeric).is_some() {
                                let color_value = i32::from_str(color_raw);

                                if color_value.is_ok() {
                                    value = color_value.unwrap();
                                }
                            } else {
                                color_name = color_raw;
                            }
                        }

                        match color_name {
                            "red" => pullish.red = value,
                            "green" => pullish.green = value,
                            "blue" => pullish.blue = value,
                            _ => println!("error color miss in {row}"),
                        }
                    }

                    instance.pulls.push(pullish);
                }
            }
        }
        games.push(instance);
    }
}
