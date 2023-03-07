use std::fmt::{Debug, Formatter};

use colored::Colorize;

// Can send to threads + Mutable + Ptr
#[derive(Clone)]
pub(crate) struct DpTablePtr<V>(pub(crate) *mut Vec<Vec<V>>);
unsafe impl<V> Sync for DpTablePtr<V> {}
unsafe impl<V> Send for DpTablePtr<V> {}

impl<V> DpTablePtr<V> {
    #[inline]
    pub(crate) unsafe fn get(&self, x: usize, y: usize, z: usize) -> &V {
        &((*self.0.add(x))[y][z])
    }
    #[inline]
    pub(crate) unsafe fn insert(&self, x: usize, y: usize, z: usize, val: V) {
        (*self.0.add(x))[y][z] = val;
    }
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn as_const_ptr(&self) -> *const Vec<Vec<V>> {
        self.0
    }
}

#[allow(non_snake_case)]
pub(crate) struct DpTable<V> {
    X: usize,
    Y: usize,
    Z: usize,

    data: Vec<Vec<Vec<V>>>,
}

impl<V: Default + Clone> DpTable<V> {
    #[inline]
    pub(crate) fn new(x: usize, y: usize, z: usize) -> Self {
        Self {
            X: x,
            Y: y,
            Z: z,
            data: vec![vec![vec![V::default(); z]; y]; x],
        }
    }
    #[inline]
    pub(crate) fn get(&self, x: usize, y: usize, z: usize) -> &V {
        &self.data[x][y][z]
    }
    #[inline]
    pub(crate) fn insert(&mut self, x: usize, y: usize, z: usize, val: V) {
        self.data[x][y][z] = val;
    }

    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> DpTablePtr<V> {
        DpTablePtr(self.data.as_mut_ptr())
    }
}

impl<V: Default + Clone + Debug> Debug for DpTable<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for z in 0..self.Z {
            writeln!(f, "z = {z}")?;
            for x in 0..self.X {
                for y in 0..self.Y {
                    write!(f, "|{}", format!("{:?}", self.get(x, y, z)).cyan())?;
                }
                writeln!(f, "|")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
