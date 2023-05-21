use std::{cell::RefCell, collections::VecDeque, rc::Rc};

struct Inner<T> {
    queue: RefCell<VecDeque<T>>,
}

pub struct Sender<T> {
    inner: Rc<Inner<T>>,
}

impl<T> Sender<T> {
    pub fn send(&self, value: T) {
        self.inner.queue.borrow_mut().push_back(value);
    }
}

impl<T> Clone for Sender<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

pub struct Receiver<T> {
    inner: Rc<Inner<T>>,
}

impl<T> Receiver<T> {
    pub fn try_recv(&self) -> Option<T> {
        self.inner.queue.borrow_mut().pop_front()
    }
}

impl<T> Clone for Receiver<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Rc::new(Inner {
        queue: RefCell::new(VecDeque::new()),
    });

    (
        Sender {
            inner: inner.clone(),
        },
        Receiver { inner },
    )
}
