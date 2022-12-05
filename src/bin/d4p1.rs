use std::io::BufRead;

fn main() {
    let input_file = std::fs::File::open("inputs/d4").unwrap();
    let buf_reader = std::io::BufReader::new(input_file);

    let pairs: Vec<(SectionRange, SectionRange)> = buf_reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let parts = l.split_once(',').expect("bad pair format");
            (
                parts.0.try_into().expect("bad range format"),
                parts.1.try_into().expect("bad range format"),
            )
        })
        .collect();

    let mut count = 0;

    pairs.iter().for_each(|(p1, p2)| {
        if p1.contains(&p2) || p2.contains(p1) {
            count += 1;
        }
    });

    println!("{}", count);
}

struct SectionRange(u64, u64);

impl TryFrom<&str> for SectionRange {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split_once('-').ok_or("bad range format")?;
        let start = parts.0.parse().map_err(|_| "bad start format")?;
        let end = parts.1.parse().map_err(|_| "bad start format")?;
        return Ok(Self(start, end));
    }
}

impl SectionRange {
    pub fn contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }
}
