use crate::utils::ivec3::IVec3;

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
struct Box {
    points: Vec<IVec3>,
    supports: Vec<usize>,
    supported_by: Vec<usize>,
}

impl From<&str> for Box {
    fn from(value: &str) -> Self {
        let points = value
            .split('~')
            .map(|s| IVec3::from_iter(s.split(',').map(|n| n.parse().unwrap())))
            .collect();
        Self {
            points,
            supports: vec![],
            supported_by: vec![],
        }
    }
}

impl Box {
    pub fn collinear(&self, other: &Box) -> bool {
        self.max_x() >= other.min_x()
            && self.min_x() <= other.max_x()
            && self.max_y() >= other.min_y()
            && self.min_y() <= other.max_y()
    }
    pub fn shift_down(&mut self, n: u32) {
        self.points[0].z -= n;
        self.points[1].z -= n;
    }
    pub fn min_x(&self) -> u32 {
        self.points[0].x.min(self.points[1].x)
    }
    pub fn max_x(&self) -> u32 {
        self.points[0].x.max(self.points[1].x)
    }
    pub fn min_y(&self) -> u32 {
        self.points[0].y.min(self.points[1].y)
    }
    pub fn max_y(&self) -> u32 {
        self.points[0].y.max(self.points[1].y)
    }
    pub fn min_z(&self) -> u32 {
        self.points[0].z.min(self.points[1].z)
    }
    pub fn max_z(&self) -> u32 {
        self.points[0].z.max(self.points[1].z)
    }
}

pub fn p1(s: &str) -> String {
    let mut boxes = s.lines().map(Box::from).collect::<Vec<_>>();
    boxes.sort_unstable_by_key(|v| v.min_z());
    //Shift boxes down
    for i in 0..boxes.len() {
        let mut shifted = false;
        for j in (0..i).rev() {
            if boxes[j].collinear(&boxes[i])
                && let Some(diff) = boxes[i].min_z().checked_sub(boxes[j].max_z())
            {
                boxes[i].shift_down(diff - 1);
                shifted = true;
                break;
            }
        }
        //Boxes that intersect with nothing fall to the ground
        if !shifted {
            let d = boxes[i].min_z();
            boxes[i].shift_down(d - 1);
        }
    }
    //Update relation tree
    boxes.sort_unstable_by_key(|v| v.min_z());
    for i in 0..boxes.len() {
        for j in (0..i).rev() {
            if boxes[j].collinear(&boxes[i]) && boxes[i].min_z() == boxes[j].max_z() + 1 {
                boxes[j].supports.push(i);
                boxes[i].supported_by.push(j);
            }
        }
    }
    dbg!(&boxes);
    //Boxes are safe to disintegrate if all of their children are supported by at least one other box
    boxes
        .iter()
        .filter(|b| b.supports.iter().all(|&i| boxes[i].supported_by.len() > 1))
        .count()
        .to_string()
}

pub fn p2(_s: &str) -> String {
    unimplemented!()
}
