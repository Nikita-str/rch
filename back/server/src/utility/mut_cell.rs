use std::ops::{Deref, DerefMut};


pub enum MutCell<'x, T> {
    Owned(T),
    MutRef(&'x mut T),
}

impl<'x, T> MutCell<'x, T> {
    pub fn new_owned(value: T) -> Self {
        Self::Owned(value)
    }
    pub fn new_mut_ref(value: &'x mut T) -> Self {
        MutCell::MutRef(value)
    }

    fn as_ref(&self) -> &T {
        match self {
            Self::Owned(value) => value,
            Self::MutRef(value) => value,
        }
    }

    fn as_mut(&mut self) -> &mut T {
        match self {
            Self::Owned(value) => value,
            Self::MutRef(value) => value,
        }
    }
}

impl<'x, T> Deref for MutCell<'x, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<'x, T> DerefMut for MutCell<'x, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}