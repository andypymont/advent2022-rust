use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct FileSystem {
    folders: HashSet<String>,
    files: HashMap<String, u32>,
}

const MAX_SMALL_FOLDER_SIZE: u32 = 100_000;
const FILE_SYSTEM_SIZE: u32 = 70_000_000;
const SPACE_NEEDED: u32 = 30_000_000;

impl FileSystem {
    fn new() -> FileSystem {
        let folders = HashSet::new();
        let files = HashMap::new();
        FileSystem { folders, files }
    }

    fn total_size(&self, folder_path: &String) -> u32 {
        self.files
            .iter()
            .map(|(path, size)| {
                if path.starts_with(folder_path) {
                    size
                } else {
                    &0
                }
            })
            .sum()
    }

    fn total_size_of_small_directories(&self) -> u32 {
        self.folders
            .iter()
            .map(|f| {
                let size = self.total_size(f);
                if size <= MAX_SMALL_FOLDER_SIZE {
                    size
                } else {
                    0
                }
            })
            .sum()
    }

    fn deletion_candidates(&self) -> HashMap<String, u32> {
        let occupied: u32 = self.files.values().sum();
        let free_space_needed = SPACE_NEEDED - (FILE_SYSTEM_SIZE - occupied);

        let mut candidates = HashMap::new();
        for folder in &self.folders {
            let size = self.total_size(folder);
            if size >= free_space_needed {
                candidates.insert(folder.to_string(), size);
            };
        }
        candidates
    }

    fn smallest_deletion_candidate_size(&self) -> Option<u32> {
        self.deletion_candidates().values().min().copied()
    }
}

fn read_file_system(input: &str) -> FileSystem {
    let mut fs = FileSystem::new();

    let mut path: Vec<String> = vec![];

    for line in input.lines() {
        if line == "$ cd /" {
            path.clear();
        } else if line == "$ cd .." {
            path.pop();
        } else if line == "$ ls" {
            continue;
        } else if line[..5].to_string() == "$ cd " {
            let subfolder = line[5..].to_string();
            path.push(subfolder);
            fs.folders.insert(path.join("/"));
        } else if line[..4].to_string() == "dir" {
            let subfolder = line[4..].to_string();
            path.push(subfolder);
            fs.folders.insert(path.join("/"));
            path.pop();
        } else {
            let parts: Vec<&str> = line.split(' ').collect();
            let filename = parts[1].to_string();
            let filesize = parts[0].parse::<u32>().unwrap_or(0);
            path.push(filename);
            fs.files.insert(path.join("/"), filesize);
            path.pop();
        }
    }

    fs
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    Some(read_file_system(input).total_size_of_small_directories())
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    read_file_system(input).smallest_deletion_candidate_size()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_system() {
        let input = advent_of_code::read_file("examples", 7);
        let fs = read_file_system(&input);

        assert_eq!(fs.folders.contains("a"), true);
        assert_eq!(fs.folders.contains("d"), true);
        assert_eq!(fs.folders.contains("a/e"), true);

        assert_eq!(fs.files.get("d/d.ext").map(|v| *v as u32), Some(5626152));
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
