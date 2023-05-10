use std::{
    cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut},
    rc::Rc,
};

pub struct RcCell<T>(Rc<RefCell<T>>);

impl<T> RcCell<T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }

    pub fn borrow(&self) -> Ref<T> {
        self.0.as_ref().borrow()
    }

    pub fn try_borrow(&self) -> Result<Ref<T>, BorrowError> {
        self.0.as_ref().try_borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.0.as_ref().borrow_mut()
    }

    pub fn try_borrow_mut(&self) -> Result<RefMut<T>, BorrowMutError> {
        self.0.as_ref().try_borrow_mut()
    }
}

impl<T> Clone for RcCell<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> From<T> for RcCell<T> {
    fn from(value: T) -> Self {
        RcCell::new(value)
    }
}
