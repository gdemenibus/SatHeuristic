use crate::schedule::Schedule;
use bumpalo::Bump;
use itertools::izip;
use shared::id_generator::IdGenerator;
use shared::project::Project;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub fn read_input<'a>(
    filename: &'a str,
    arena: &'a Bump,
    set_up_time: u32,
) -> Option<Schedule<'a>> {
    let file = File::open(filename).unwrap();
    //unwrap_or_else(|_| panic!("Could not read file {}", filename));
    let mut lines = BufReader::new(file).lines();
    // Corresponds to file info at top. We only care about number of jobs
    skip_lines(&mut lines, 5);
    let n_activities = get_number_of_activities(lines.next()?.ok()?);
    //skip_lines(&mut lines, 2);
    let _horizon = get_horizon(lines.next()?.ok()?);
    skip_lines(&mut lines, 1);
    let n_renewable = get_number_of_resources(lines.next()?.ok()?);
    let n_nonrenewable = get_number_of_resources(lines.next()?.ok()?);
    skip_lines(&mut lines, 8);

    let activities: Vec<usize> = (1..n_activities + 1).collect();
    let mut modes: Vec<usize> = Vec::new();
    let mut modes_per_activity: Vec<Vec<usize>> = vec![Default::default(); n_activities];
    let mut successors: Vec<Vec<usize>> = vec![Default::default(); n_activities];

    for i in 0..n_activities {
        let line = line_to_numbers(lines.next()?.ok()?);
        let n_modes = *line.get(1).unwrap();
        let modes_i = modes_per_activity.get_mut(i).unwrap();
        for _ in 0..n_modes {
            modes_i.push(modes.len());
            modes.push(modes.len());
        }
        if line.len() > 3 {
            let successors_i = successors.get_mut(i).unwrap();
            for j in 3..line.len() {
                successors_i.push(*line.get(j).unwrap() as usize);
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
        (0..n_renewable).for_each(|i| {
            resources_mode[i] = *line
                .get(line.len() + i - n_nonrenewable - n_renewable)
                .unwrap();
        });
        (n_renewable..(n_renewable + n_nonrenewable)).for_each(|i| {
            resources_mode[i] = *line
                .get(line.len() + i - n_nonrenewable - n_renewable)
                .unwrap();
        });
    }

    skip_lines(&mut lines, 3);
    let line = line_to_numbers(lines.next()?.ok()?);
    let capacity_per_renewable_resource = line.get(..n_renewable).unwrap().to_vec();
    let capacity_per_nonrenewable_resource = line.get(n_renewable..).unwrap().to_vec();
    let schedule = Schedule::new(
        create_projects(
            arena,
            resources_per_mode,
            successors,
            activities,
            durations_per_mode,
            set_up_time,
        ),
        capacity_per_renewable_resource,
    );
    Some(schedule)
}
fn create_projects(
    arena: &Bump,
    resources: Vec<Vec<u32>>,
    successors: Vec<Vec<usize>>,
    projs: Vec<usize>,
    durations: Vec<u32>,
    set_up_time: u32,
) -> Vec<&Project<'_>> {
    let mut projects: Vec<&Project> = Vec::new();
    let mut id_gen = IdGenerator::generator_for_segment();
    for (resource, proj, duration) in izip!(resources, projs, durations) {
        let project = arena.alloc(Project::new(
            duration,
            proj as u64,
            resource,
            RefCell::new(Vec::new()),
            &mut id_gen,
            set_up_time,
        ));
        projects.push(project);
    }
    // Ids start at 1 and vec starts at 0, so there is an ofset by one going on here
    // Projects are sorted by ids
    projects.sort();
    connect_precedence(&projects, successors);
    link_with_precedence(&projects);

    projects
}
fn connect_precedence<'a>(projects: &Vec<&'a Project<'a>>, successors: Vec<Vec<usize>>) {
    for (index, successors) in successors.into_iter().enumerate() {
        let precedent = projects.get(index).unwrap();
        for successor in successors {
            //println!("Suc {} of {}", index + 1, successor);
            // successor is based on the id's, not the index
            let suc = projects.get(successor - 1).unwrap();
            suc.add_presedence(precedent);
        }
    }
}
fn link_with_precedence<'a>(projects: &Vec<&'a Project<'a>>) {
    for project in projects {
        project.link_with_precedents();
    }
}
fn skip_lines(lines: &mut Lines<BufReader<File>>, n: u32) {
    for _ in 0..n {
        lines.next();
    }
}

fn get_number_of_activities(line: String) -> usize {
    line.split(':').nth(1).unwrap().trim().parse().unwrap()
}

fn get_number_of_resources(line: String) -> usize {
    line.split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap()
}

fn line_to_numbers(line: String) -> Vec<u32> {
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_horizon(line: String) -> usize {
    line.split(':').nth(1).unwrap().trim().parse().unwrap()
}
#[cfg(test)]
#[test]
fn proj_creation_from_read() {
    //Project of size 1, with no dependencies, and resource 1
    let rsr = vec![vec![1]];
    let act = vec![1];
    let dep: Vec<Vec<usize>> = vec![vec![]];
    let arena = Bump::new();

    let created_project = create_projects(&arena, rsr, dep, act, vec![1], 0)[0];
    let expected_project = Project::new(
        1,
        1,
        vec![1],
        RefCell::new(Vec::new()),
        &mut IdGenerator(0),
        0,
    );
    //simpl test
    assert_eq!(*created_project, expected_project);
}
#[test]
fn proj_creation_three() {
    // 1 -> 2 -> 3 (3 depends on 2, which depends on 1)
    // all have same resource usage, which is vec1
    let resources = vec![vec![1], vec![1], vec![1]];
    let mut id_gen = IdGenerator(0);
    let project1 = Project::new(1, 1, vec![1], RefCell::new(Vec::new()), &mut id_gen, 0);
    let project2 = Project::new(1, 2, vec![1], RefCell::new(vec![&project1]), &mut id_gen, 0);
    let project3 = Project::new(1, 3, vec![1], RefCell::new(vec![&project2]), &mut id_gen, 0);
    let expected_projects = vec![&project1, &project2, &project3];

    let arena = Bump::new();
    let successors = vec![vec![2], vec![3], vec![]];
    let projs = vec![1, 2, 3];
    let durations = vec![1, 1, 1];
    let generetad_projects = create_projects(&arena, resources, successors, projs, durations, 0);
    for generated in generetad_projects {
        assert!(expected_projects.contains(&generated));
    }
}
