use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub fn read_input(filename: &str) -> Option<()> {
    let file = File::open(filename).expect(format!("Could not read file {}", filename).as_str());
    let mut lines = BufReader::new(file).lines();

    skip_lines(&mut lines, 5);
    let n_activities = get_number_of_activities(lines.next()?.ok()?);
    skip_lines(&mut lines, 2);
    let n_renewable = get_number_of_resources(lines.next()?.ok()?);
    let n_nonrenewable = get_number_of_resources(lines.next()?.ok()?);
    skip_lines(&mut lines, 8);

    let activities: Vec<usize> = (0..n_activities).collect();
    let mut modes: Vec<usize> = Vec::new();
    let mut modes_per_activity: Vec<Vec<usize>> = vec![Default::default(); n_activities];
    let mut successors: Vec<Vec<usize>> = vec![Default::default(); n_activities];

    for i in 0..n_activities {
        let line = line_to_numbers(lines.next()?.ok()?);
        let n_modes = *line.get(1).unwrap();
        let modes_i = modes_per_activity.get_mut(i).unwrap();
        for _ in 0..n_modes {
            modes_i.push(modes.len());
            modes.push((modes.len()));
        }
        if line.len() > 3 {
            let successors_i = successors.get_mut(i).unwrap();
            for j in 3..line.len() {
                successors_i.push((line.get(j).unwrap() - 1) as usize);
            }
        }
    }

    let mut durations_per_mode: Vec<u32> = Vec::with_capacity(modes.len());
    let mut resources_per_mode: Vec<Vec<u32>> =
        vec![vec![Default::default(); n_renewable + n_nonrenewable]; modes.len()];

    skip_lines(&mut lines, 4);
    for mode in 0..modes.len() {
        let line = line_to_numbers(lines.next()?.ok()?);
        durations_per_mode.push(
            *line
                .get(line.len() - (n_renewable + n_nonrenewable + 1))
                .unwrap(),
        );
        let resources_mode = resources_per_mode.get_mut(mode).unwrap();
        for i in 0..n_renewable {
            resources_mode[i] = *line
                .get(line.len() + i - n_nonrenewable - n_renewable)
                .unwrap();
        }
        for i in n_renewable..(n_renewable + n_nonrenewable) {
            resources_mode[i] = *line
                .get(line.len() + i - n_nonrenewable - n_renewable)
                .unwrap();
        }
    }

    skip_lines(&mut lines, 3);
    let line = line_to_numbers(lines.next()?.ok()?);
    let capacity_per_renewable_resource = line.get(..n_renewable).unwrap().to_vec();
    let capacity_per_nonrenewable_resource = line.get(n_renewable..).unwrap().to_vec();

    Some(())
}

fn skip_lines(lines: &mut Lines<BufReader<File>>, n: u32) {
    for _ in 0..n {
        lines.next();
    }
}

fn get_number_of_activities(line: String) -> usize {
    line.split(":").nth(1).unwrap().trim().parse().unwrap()
}

fn get_number_of_resources(line: String) -> usize {
    line.split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .nth(0)
        .unwrap()
        .parse()
        .unwrap()
}

fn line_to_numbers(line: String) -> Vec<u32> {
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
