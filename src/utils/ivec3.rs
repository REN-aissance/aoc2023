use std::fmt::{self, Display};

type N = u32;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IVec3 {
    pub x: N,
    pub y: N,
    pub z: N,
}

impl FromIterator<N> for IVec3 {
    fn from_iter<T: IntoIterator<Item = N>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Self {
            x: iter.next().unwrap(),
            y: iter.next().unwrap(),
            z: iter.next().unwrap(),
        }
    }
}

impl Display for IVec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{},{}]", self.x, self.y, self.z)
    }
}

impl fmt::Debug for IVec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ x:{}, y:{}, z:{} ]", self.x, self.y, self.z)
    }
}
