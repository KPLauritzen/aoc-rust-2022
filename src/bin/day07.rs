use aoc_rust_2022::file_to_vec;
use scan_fmt::scan_fmt_some;
use std::num::ParseIntError;

#[derive(Debug)]
struct Dir {
    name: String,
    parent: usize,
    size: usize,
}

fn main() {
    let filename = "input/day07.txt";
    let input = file_to_vec(filename).unwrap();
    let part_1_result = part_1(&input).unwrap();
    println!("Part 1: {}", part_1_result);

    let part_2_result = part_2(&input).unwrap();
    println!("Part 2: {}", part_2_result);
}

fn part_1(input: &[String]) -> Result<usize, ParseIntError> {
    let tree = create_dir_tree(input);
    // Calculate size of dirs that have at most size = threshold
    let threshold = 100_000;
    let size = tree
        .into_iter()
        .filter(|dir| dir.size <= threshold)
        .fold(0, |acc, dir| acc + dir.size);
    Ok(size)
}

fn create_dir_tree(input: &[String]) -> Vec<Dir> {
    let mut tree: Vec<Dir> = vec![Dir {
        name: "/".to_string(),
        parent: 0,
        size: 0,
    }];
    let mut current_dir_idx = 0;
    // I'm adding the first line manually to make sure we have an initial element in the list
    for line in input.iter().skip(1) {
        if line.starts_with("$ cd") {
            let name = scan_fmt_some!(line, "$ cd {}", String).unwrap();
            if name == ".." {
                // move up one level.
                current_dir_idx = tree.get(current_dir_idx).unwrap().parent;
            } else {
                // Add a new folder to the tree and move there
                // I'm assuming we only visit each folder once.
                tree.push(Dir {
                    name,
                    parent: current_dir_idx,
                    size: 0,
                });
                current_dir_idx = tree.len() - 1;
            }
        } else if line.starts_with("$ ls") || line.starts_with("dir") {
            // in these cases I dont care
        } else {
            // this is a file
            let (size, _name) = scan_fmt_some!(line, "{d} {}", usize, String);
            tree = update_sizes(current_dir_idx, size.unwrap(), tree)
        }
        // println!("{line}");
        // println!("Current dir: {:?}", &tree.get(current_dir_idx).unwrap());
        // println!("{:?}", tree);
        // println!("--");
    }
    tree
}

fn update_sizes(current_dir_idx: usize, size: usize, mut tree: Vec<Dir>) -> Vec<Dir> {
    let mut update_idx = current_dir_idx;
    loop {
        let current_dir = tree.get_mut(update_idx).unwrap();
        current_dir.size += size;
        update_idx = current_dir.parent;
        // If we have updated the root node, stop
        if current_dir.name == "/" {
            break;
        }
    }
    tree
}

fn part_2(input: &[String]) -> Result<usize, ParseIntError> {
    let tree = create_dir_tree(input);
    let total_diskspace = 70000000;
    let needed_diskspace = 30000000;
    let used_diskspace = tree.first().unwrap().size;
    let available_diskspace = total_diskspace - used_diskspace;
    let space_to_delete = needed_diskspace - available_diskspace;
    // println!("need to delete at least {space_to_delete}");
    // Find the dir I have to delete. It has to be above the threshold, but only as little above as possible.
    let dir_to_delete = tree
        .iter()
        .filter(|dir| dir.size >= space_to_delete)
        .min_by_key(|dir| space_to_delete.abs_diff(dir.size))
        .unwrap();
    // println!("deleting {:?}", dir_to_delete);
    Ok(dir_to_delete.size)
}

#[cfg(test)]
mod tests {
    use super::*;
    // Part 1
    #[test]
    fn part_1_sample_input() {
        let filename = "input/day07_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_1(&input).unwrap();
        assert_eq!(result, 95437);
    }

    // Part 2
    #[test]
    fn part_2_sample_input() {
        let filename = "input/day07_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_2(&input).unwrap();
        assert_eq!(result, 24933642);
    }
}
