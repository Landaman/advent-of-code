use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fs,
    rc::Rc,
};

const DIMENSIONS: usize = 3;

const LARGEST_N_GROUPS_N: usize = 3;
const CLOSEST_N_EDGES_N: usize = 1000;

type Point = [i64; DIMENSIONS];

struct Edge<'a> {
    distance: f64,
    one: &'a Point,
    two: &'a Point,
}

struct Group<'a> {
    size: usize,
    points: Vec<&'a Point>,
}

enum GroupingResult {
    NewGroup,
    MergedGroups,
    AddedToExistingGroup,
    NoAction,
}

fn process_edge<'a>(
    edge: &Edge<'a>,
    points_to_groups: &mut HashMap<&'a Point, Rc<RefCell<Group<'a>>>>,
) -> GroupingResult {
    match (
        points_to_groups.get(edge.one).cloned(),
        points_to_groups.get(edge.two).cloned(),
    ) {
        (Some(group_one), Some(group_two)) => {
            if Rc::ptr_eq(&group_one, &group_two) {
                return GroupingResult::NoAction;
            }

            let bigger_group_rc: Rc<RefCell<Group>>;
            let smaller_group_rc: Rc<RefCell<Group>>;
            if group_one.borrow().size >= group_two.borrow().size {
                bigger_group_rc = group_one.clone();
                smaller_group_rc = group_two.clone();
            } else {
                bigger_group_rc = group_two.clone();
                smaller_group_rc = group_one.clone();
            }

            let mut bigger_group = bigger_group_rc.borrow_mut();
            let smaller_group = smaller_group_rc.borrow();
            bigger_group.size += smaller_group.size;
            for point in &smaller_group.points {
                bigger_group.points.push(point);
                points_to_groups.insert(point, bigger_group_rc.clone());
            }

            // Now, all references to smaller_group should be dropped
            GroupingResult::MergedGroups
        }

        (Some(group_one), None) => {
            points_to_groups.insert(edge.two, group_one.clone());
            let mut group_one = group_one.borrow_mut();
            group_one.size += 1;
            group_one.points.push(edge.two);

            GroupingResult::AddedToExistingGroup
        }

        (None, Some(group_two)) => {
            points_to_groups.insert(edge.one, group_two.clone());
            let mut group_two = group_two.borrow_mut();
            group_two.size += 1;
            group_two.points.push(edge.one);

            GroupingResult::AddedToExistingGroup
        }

        (None, None) => {
            let group = Rc::new(RefCell::new(Group {
                size: 2,
                points: vec![edge.one, edge.two],
            }));
            points_to_groups.insert(edge.one, group.clone());
            points_to_groups.insert(edge.two, group.clone());

            GroupingResult::NewGroup
        }
    }
}

fn part_one(edges: &Vec<Edge>) {
    let mut points_to_groups: HashMap<&Point, Rc<RefCell<Group>>> = HashMap::new();
    for edge in edges.iter().take(CLOSEST_N_EDGES_N) {
        process_edge(edge, &mut points_to_groups);
    }

    let mut groups: Vec<Rc<RefCell<Group>>> = points_to_groups.values().cloned().collect();
    groups.sort_unstable_by(|a, b| a.borrow().size.cmp(&b.borrow().size).reverse());
    groups.dedup_by(|a, b| Rc::ptr_eq(a, b));
    let product = groups
        .iter()
        .take(LARGEST_N_GROUPS_N)
        .fold(1, |acc, group| acc * group.borrow().size);
    println!("Result: {}", product);
}

fn part_two(edges: &Vec<Edge>) {
    let mut unique_points: HashSet<&Point> = edges
        .iter()
        .flat_map(|edge| vec![edge.one, edge.two])
        .collect();
    let mut points_to_groups: HashMap<&Point, Rc<RefCell<Group>>> = HashMap::new();
    let mut last_edge: Option<&Edge> = None;
    let mut num_groups = 0;

    let mut edges_iter = edges.iter();
    while !unique_points.is_empty() || num_groups > 1 {
        let edge = edges_iter.next().unwrap();
        match process_edge(edge, &mut points_to_groups) {
            GroupingResult::NewGroup => num_groups += 1,
            GroupingResult::MergedGroups => num_groups -= 1,
            GroupingResult::AddedToExistingGroup | GroupingResult::NoAction => {}
        };

        unique_points.remove(edge.one);
        unique_points.remove(edge.two);
        last_edge = Some(edge);
    }

    let checked_last_edge = last_edge.unwrap();
    println!(
        "Result: {}",
        checked_last_edge.one[0] * checked_last_edge.two[0]
    );
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // parse points
    let mut points: Vec<Point> = Vec::new();
    points.extend(contents.lines().map(|line| -> Point {
        line.split(',')
            .map(|string| string.parse::<i64>().expect("Found an invalid point"))
            .collect::<Vec<i64>>()
            .try_into()
            .expect("Point does not have correct dimensions")
    }));

    let mut edges: Vec<Edge> = Vec::new();
    for outer_index in 0..points.len() {
        let outer = &points[outer_index];
        // Start at outer + 1
        for inner in points.iter().skip(outer_index + 1) {
            // Do euclidean distance
            let distance = (outer
                .iter()
                .zip(inner)
                .fold(0, |acc, (outer_value, inner_value)| {
                    acc + (outer_value - inner_value).pow(2)
                }) as f64)
                .sqrt();

            // Keep edges in ascending order by distance
            let edge_index = edges.partition_point(|edge| edge.distance <= distance); // <= creates a more efficient partition
            edges.insert(
                edge_index,
                Edge {
                    distance,
                    one: outer,
                    two: inner,
                },
            );
        }
    }

    part_one(&edges);
    part_two(&edges);
}
