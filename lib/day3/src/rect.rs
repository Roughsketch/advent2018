/// Simple Rect struct for handling input
/// Id is added for part 2
#[derive(Debug)]
pub struct Rect {
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