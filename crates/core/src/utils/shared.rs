use std::{
    cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut},
    rc::Rc,
};

pub struct Shared<T>(Rc<RefCell<T>>);

impl<T> Default for Shared<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Shared<T> {
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

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
