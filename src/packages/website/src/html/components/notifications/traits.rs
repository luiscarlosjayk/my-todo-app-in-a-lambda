use maud::Render;
use std::fmt::Debug;

pub trait Notification: Render + Debug {}