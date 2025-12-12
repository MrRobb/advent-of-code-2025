struct Present {
    shape: Vec<Vec<char>>,
}

impl Present {
    fn cells(&self) -> usize {
        self.shape
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| cell == '#')
            .count()
    }
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    presents: Vec<usize>,
}

fn parse_input(input: &str) -> (Vec<Present>, Vec<Region>) {
    let mut sections: Vec<&str> = input.trim().split("\n\n").collect();
    let regions_str = sections.pop().unwrap();

    // Parse Presents
    let presents = sections
        .iter()
        .map(|section| {
            let shape = section.lines().skip(1).map(|line| line.chars().collect()).collect();
            Present { shape }
        })
        .collect();

    // Parse Regions
    let regions = regions_str
        .lines()
        .map(|line| {
            let (dimensions, counts) = line.split_once(": ").unwrap();
            let (w, h) = dimensions.split_once('x').unwrap();
            let presents_region = counts.split_whitespace().map(|n| n.parse().unwrap()).collect();

            Region {
                width: w.parse().unwrap(),
                height: h.parse().unwrap(),
                presents: presents_region,
            }
        })
        .collect();

    (presents, regions)
}

fn region_fits(region: &Region, present_list: &[Present]) -> bool {
    let total_present_area = region
        .presents
        .iter()
        .zip(present_list.iter())
        .map(|(count, present)| count * present.cells())
        .sum();

    (region.width * region.height) >= total_present_area
}

pub fn fit_presents(input: &str) -> usize {
    let (presents, regions) = parse_input(input);
    regions.iter().filter(|region| region_fits(region, &presents)).count()
}

pub const fn have_fun(_input: &str) -> &'static str {
    "\u{2605}"
}

pub fn main() {
    let input = std::fs::read_to_string("input/day12.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", fit_presents(&input));
    println!("PART 2 = {}", have_fun(&input));
    println!("Execution time: {:?}", now.elapsed());
}
