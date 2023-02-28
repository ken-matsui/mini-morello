use std::fmt::{Debug, Formatter};

// Can send to threads + Mutable + Ptr
#[derive(Clone)]
pub(crate) struct DpTablePtr<V>(pub(crate) *mut Vec<V>);
unsafe impl<V> Sync for DpTablePtr<V> {}
unsafe impl<V> Send for DpTablePtr<V> {}

impl<V: Copy> DpTablePtr<V> {
    #[inline]
    pub(crate) unsafe fn get(&self, x: usize, y: usize) -> &V {
        &((*self.0.add(x))[y])
    }
    #[inline]
    pub(crate) unsafe fn insert(&self, x: usize, y: usize, val: V) {
        (*self.0.add(x))[y] = val;
    }
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn as_const_ptr(&self) -> *const Vec<V> {
        self.0
    }
}

#[allow(non_snake_case)]
pub(crate) struct DpTable<V> {
    X: usize,
    Y: usize,

    data: Vec<Vec<V>>,
}

impl<V: Default + Clone> DpTable<V> {
    #[inline]
    pub(crate) fn new(x: usize, y: usize) -> Self {
        Self {
            X: x,
            Y: y,
            data: vec![vec![V::default(); x + 1]; y + 1],
        }
    }
    #[inline]
    pub(crate) fn get(&self, x: usize, y: usize) -> &V {
        &self.data[x][y]
    }
    #[inline]
    pub(crate) fn insert(&mut self, x: usize, y: usize, val: V) {
        self.data[x][y] = val;
    }

    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> DpTablePtr<V> {
        DpTablePtr(self.data.as_mut_ptr())
    }
}

impl<V: Default + Clone + Debug> Debug for DpTable<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in 0..=self.X {
            for y in 0..=self.Y {
                write!(f, "{:?} ", self.get(x, y))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
