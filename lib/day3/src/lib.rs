#[derive(Debug)]
struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            x, y, w, h,
        }
    }
    
    pub fn area(&self) -> i32 {
        self.w * self.h
    }

    pub fn overlap(&self, other: &Rect) -> Option<Rect> {
        if self.x > other.x + other.w || other.x > self.x + self.w {
            return None;
        } else if self.y > other.y + other.h || other.y > self.y + self.h {
            return None;
        }

        let x = std::cmp::max(self.x, other.x);
        let y = std::cmp::max(self.y, other.y);
        let w = std::cmp::min(self.x + self.w, other.x + other.w) - x;
        let h = std::cmp::min(self.y + other.h, other.y + other.h) - y;

        Some(Rect::new(x, y, w, h))
    }
}

fn read_rects(content: &str) -> Vec<Rect> {
    let mut rects = Vec::new();

    for line in content.lines() {
        let split = line.split(|c: char| !c.is_numeric())
            .filter(|s| !s.is_empty())
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        rects.push(Rect::new(
            split[1],
            split[2],
            split[3],
            split[4],
        ));
    }

    rects
}

pub fn part_1() -> Result<i32, std::io::Error> {
    let content = std::fs::read_to_string("input/3.txt")?;

    let rects = read_rects(&content);
    let mut overlaps = Vec::new();

    for (current, rect) in rects.iter().enumerate() {
        for index in current + 1..rects.len() {
            if let Some(overlap) = rect.overlap(&rects[index]) {
                println!("Overlap: {:?} {:?} => {:?}", rect, rects[index], overlap);
                overlaps.push(overlap);
            }
        }
    }

    let total_area: i32 = rects.iter().map(|r| r.area()).sum();
    let overlap_area: i32 = overlaps.iter().map(|r| r.area()).sum();

    Ok(total_area - overlap_area)
}

pub fn part_2() -> Result<i32, std::io::Error> {
    Ok(0)
}