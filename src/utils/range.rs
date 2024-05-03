use num::{Integer, ToPrimitive};
use ranges::{Domain, Ranges};
use std::cmp::Ord;
use std::iter::Sum;
use std::ops::{Bound::*, RangeBounds};

//Did not know you could implement custom traits on foreign types until now. Very convenient!
//Possibly a better idea to get rid of the dependency and use my own type
pub trait Range<T> {
    fn span(&self) -> T;
}

impl<T> Range<T> for Ranges<T>
where
    T: Domain + Ord + Integer + Sum + ToPrimitive + Copy,
{
    fn span(&self) -> T {
        self.as_ref()
            .iter()
            .map(|range| {
                let start = match range.start_bound() {
                    Included(&n) => n,
                    Excluded(&n) => n + T::one(),
                    _ => panic!(),
                };
                let end = match range.end_bound() {
                    Included(&n) => n + T::one(),
                    Excluded(&n) => n,
                    _ => panic!(),
                };
                end - start
            })
            .sum::<T>()
    }
}
