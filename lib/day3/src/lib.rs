#[derive(Debug)]
struct Rect {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn new(id: i32, x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            id, x, y, w, h,
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

        Some(Rect::new(0, x, y, w, h))
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
            split[0],
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
    let mut area = std::collections::HashSet::new();
    let mut overlap = std::collections::HashSet::new();

    for rect in &rects {
        for x in rect.x..rect.x + rect.w {
            for y in rect.y..rect.y + rect.h {
                if !area.insert((x, y)) {
                    overlap.insert((x, y));
                }
            }
        }
    }

    Ok(overlap.iter().count() as i32)
}

pub fn part_2() -> Result<i32, std::io::Error> {
    let content = std::fs::read_to_string("input/3.txt")?;

    let rects = read_rects(&content);

    for (current, rect) in rects.iter().enumerate() {
        let mut overlapped = false;

        for index in 0..rects.len() {
            if current != index && rect.overlap(&rects[index]).is_some() {
                overlapped = true;
                break;
            }
        }

        if !overlapped {
            return Ok(rect.id);
        }
    }

    Ok(0)
}