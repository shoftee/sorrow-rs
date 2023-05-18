use std::{
    cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut},
    rc::Rc,
};

pub struct RcCell<T>(Rc<RefCell<T>>);

impl<T> RcCell<T> {
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }

    #[inline(always)]
    pub fn borrow(&self) -> Ref<T> {
        self.0.as_ref().borrow()
    }

    #[inline(always)]
    pub fn try_borrow(&self) -> Result<Ref<T>, BorrowError> {
        self.0.as_ref().try_borrow()
    }

    #[inline(always)]
    pub fn borrow_mut(&self) -> RefMut<T> {
        self.0.as_ref().borrow_mut()
    }

    #[inline(always)]
    pub fn try_borrow_mut(&self) -> Result<RefMut<T>, BorrowMutError> {
        self.0.as_ref().try_borrow_mut()
    }
}

impl<T> Clone for RcCell<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> From<T> for RcCell<T> {
    #[inline(always)]
    fn from(value: T) -> Self {
        RcCell::new(value)
    }
}
