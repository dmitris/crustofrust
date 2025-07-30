use std::cell::UnsafeCell;

#[derive(Debug)]
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// implied by UnsafeCell
// impl<T> !Sync for Cell<T> {}

impl<T: Clone /* : Copy */> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value), // value.into(),
        }
    }

    pub fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        }
        // *(self.value.get_mut()) = value;
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod tests {
    use super::Cell;

    #[test]
    fn it_works() {
        let c = Cell::new(42);
        let (p1, p2) = (&c, &c);
        assert_eq!(c.get(), 42);
        assert_eq!(p1.get(), 42);
        assert_eq!(p2.get(), 42);
        c.set(43);
        assert_eq!(c.get(), 43);
        // assert_eq!(p1.get(), 43);
        // assert_eq!(p2.get(), 43);
    }
}
