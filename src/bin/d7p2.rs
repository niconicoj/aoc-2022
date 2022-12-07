use std::{
    fmt::Display,
    io::{BufRead, Lines},
};

fn main() {
    let input_file = std::fs::File::open("inputs/d7").unwrap();
    let buf_reader = std::io::BufReader::new(input_file);
    let mut lines = buf_reader.lines();

    let directory = Directory::parse(&mut lines).expect("no directories");

    let max_size = 40_000_000;
    let current_size = directory.size();
    println!("current size : {}", current_size);
    println!("target size : {}", current_size - max_size);

    let dir_to_delete = directory.closest_dir_above(current_size - max_size);

    println!("dir size to delete : {:?}", dir_to_delete);
}

#[derive(Default, Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
    directories: Vec<Directory>,
}

impl Directory {
    pub fn parse(lines: &mut Lines<impl BufRead>) -> Option<Self> {
        let mut base_dir = None;
        Self::parse_dir(lines, &mut base_dir);
        base_dir
    }

    fn parse_dir(lines: &mut Lines<impl BufRead>, dir: &mut Option<Directory>) {
        while let Some(line) = lines.next() {
            let line = line.expect("failed to read line");
            if line.starts_with("$ cd") {
                let mut new_dir = Self::parse_command(&line);
                match new_dir {
                    None => return,
                    Some(_) => match dir {
                        None => *dir = new_dir,
                        Some(parent_dir) => {
                            Self::parse_dir(lines, &mut new_dir);
                            parent_dir.directories.push(new_dir.unwrap());
                        }
                    },
                }
            } else if !line.starts_with("dir") && !line.starts_with("$ ls") {
                Self::parse_file(&line, dir);
            }
        }
    }

    fn parse_command(command: &String) -> Option<Directory> {
        let mut parts = command.split_whitespace().skip(2);
        match parts.next().expect("bad command") {
            ".." => None,
            str => Some(Self::new_with_name(str.to_string())),
        }
    }

    fn parse_file(file: &String, dir: &mut Option<Directory>) {
        let mut parts = file.split_whitespace();
        let size = parts
            .next()
            .expect("no size for file")
            .parse()
            .expect("size was not a number");
        let name = parts.next().expect("no name for file");
        match dir {
            Some(dir) => dir.files.push(File::new(name.to_string(), size)),
            None => {}
        }
    }

    fn new_with_name(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    fn size(&self) -> u64 {
        let mut total_size: u64 = self.files.iter().map(|f| f.size).sum::<u64>();
        total_size += self.directories.iter().map(|d| d.size()).sum::<u64>();
        total_size
    }

    fn closest_dir_above(&self, target: u64) -> Option<u64> {
        let mut local_best = None;
        let current_dir_size = self.size();
        if current_dir_size > target {
            local_best = Some(current_dir_size);
        }

        let child_best = self
            .directories
            .iter()
            .filter_map(|d| d.closest_dir_above(target))
            .reduce(|acc, v| if v < acc { v } else { acc });

        match (local_best, child_best) {
            (Some(local), Some(child)) => Some(local.min(child)),
            (None, Some(child)) => Some(child),
            (Some(local), None) => Some(local),
            (None, None) => None,
        }
    }

    fn display(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        writeln!(
            f,
            "{:depth$} - {name} (dir)",
            " ",
            name = self.name,
            depth = depth
        )?;
        for file in self.files.iter() {
            writeln!(
                f,
                "{:depth$} - {name} (file, size={size})",
                " ",
                name = file.name,
                depth = depth,
                size = file.size
            )?;
        }

        for dir in self.directories.iter() {
            dir.display(f, depth + 4)?;
        }
        Ok(())
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(f, 0)?;
        Ok(())
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
}

impl File {
    fn new(name: String, size: u64) -> Self {
        Self { name, size }
    }
}
