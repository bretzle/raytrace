pub trait Dim: PartialEq {
	const SIZE: usize;
}
pub trait Sub: Dim {
	type SUB;
}
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct M1;
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct M2;
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct M3;
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct M4;
impl Dim for M1 {
	const SIZE: usize = 1;
}

impl Dim for M2 {
	const SIZE: usize = 2;
}
impl Dim for M3 {
	const SIZE: usize = 3;
}
impl Dim for M4 {
	const SIZE: usize = 4;
}

impl Sub for M3 {
	type SUB = M2;
}
impl Sub for M4 {
	type SUB = M3;
}
