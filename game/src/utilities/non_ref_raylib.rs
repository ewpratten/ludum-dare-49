use std::{borrow::Borrow, cell::{Cell, RefCell, RefMut}, ops::{Deref, DerefMut}, rc::Rc, sync::Arc};

use raylib::{prelude::RaylibDraw, RaylibHandle};

pub struct HackedRaylibHandle(RaylibHandle);

impl RaylibDraw for HackedRaylibHandle {}

impl From<RaylibHandle> for HackedRaylibHandle {
    fn from(handle: RaylibHandle) -> Self {
        Self(handle)
    }
}

impl Deref for HackedRaylibHandle {
    type Target = RaylibHandle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HackedRaylibHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
