/// Simple Rect struct for handling input
/// Id is added for part 2
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

    /// Determines whether a rectangle overlaps with another, and
    /// if it does, returns the overlapped rectangle.
    pub fn overlap(&self, other: &Rect) -> Option<Rect> {
        if self.x > other.x + other.w || other.x > self.x + self.w {
            //  If they are to the left or right of eachother, no overlap
            return None;
        } else if self.y > other.y + other.h || other.y > self.y + self.h {
            //  If they are above or below eachother, no overlap
            return None;
        }

        let x = std::cmp::max(self.x, other.x);
        let y = std::cmp::max(self.y, other.y);
        let w = std::cmp::min(self.x + self.w, other.x + other.w) - x;
        let h = std::cmp::min(self.y + other.h, other.y + other.h) - y;

        Some(Rect::new(0, x, y, w, h))
    }
}

/// Reads in challenge input and turns it into a vec of Rects
fn read_rects(content: &str) -> Vec<Rect> {
    let mut rects = Vec::new();

    //  For each line
    for line in content.lines() {
        //  Split line by non-numeric chars, then parse the numbers
        let split = line.split(|c: char| !c.is_numeric())
            .filter(|s| !s.is_empty())
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        rects.push(Rect::new(
            split[0],   //  id
            split[1],   //  x
            split[2],   //  y
            split[3],   //  w
            split[4],   //  h
        ));
    }

    rects
}

pub fn part_1() -> Result<i32, std::io::Error> {
    let content = std::fs::read_to_string("input/3.txt")?;

    let rects = read_rects(&content);
    let mut area = std::collections::HashSet::new();
    let mut overlap = std::collections::HashSet::new();

    //  For each rect in the challenge input
    for rect in &rects {
        //  Get every (x, y) coord that exists in the rect
        for x in rect.x..rect.x + rect.w {
            for y in rect.y..rect.y + rect.h {
                //  And insert it into the set
                if !area.insert((x, y)) {
                    //  If it already exists, then it overlaps
                    //  Storing overlap as a set prevents counting it multiple times
                    overlap.insert((x, y));
                }
            }
        }
    }

    //  Answer is how many coords were counted as overlap
    Ok(overlap.iter().count() as i32)
}

pub fn part_2() -> Result<i32, std::io::Error> {
    let content = std::fs::read_to_string("input/3.txt")?;

    let rects = read_rects(&content);

    //  For each rect
    for (current, rect) in rects.iter().enumerate() {
        let mut overlapped = false;

        //  Iterate over all rects
        for index in 0..rects.len() {
            //  If not same index as current rect, check overlap
            if current != index && rect.overlap(&rects[index]).is_some() {
                //  If overlapped, stop checking
                overlapped = true;
                break;
            }
        }

        //  If no overlap found, return the current rectangle id
        if !overlapped {
            return Ok(rect.id);
        }
    }

    //  Return 0 as the id if none found
    Ok(0)
}