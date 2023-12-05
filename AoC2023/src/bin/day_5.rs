fn main() {
    let almanac = input();
}

struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil: Vec<(usize, usize, usize)>,
    fertilizer_to_water: Vec<(usize, usize, usize)>,
    water_to_light: Vec<(usize, usize, usize)>,
    light_to_temperature: Vec<(usize, usize, usize)>,
    temperature_to_humidity: Vec<(usize, usize, usize)>,
    humidity_to_location: Vec<(usize, usize, usize)>,
}

fn input() -> Almanac {
    let string = std::fs::read_to_string("inout/day4.in").unwrap();
    let mut lines = string.lines();
    let seeds: Vec<usize> = {
        let line = lines.next().unwrap();
        line[6..].split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect()
    };

    // let seed_to_soil: Vec<(usize, usize, usize)> = {
    //     lines.next().unwrap();
    //     let mut v = vec![];
    //     while let Some(line) = lines.next() {
    //         if line.is_empty() {
    //             break;
    //         }
    //         let [v0, v1, v2, ..] = &line.split_whitespace().map(|str| str.parse::<usize>().unwrap()).collect::<Vec<_>>()[..] else {
    //             unreachable!()
    //         }; 
    //         v.push((*v0, *v1, *v2))
    //     }
    //     v
    // };

    todo!()

}