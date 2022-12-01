use std::ops::{Add, Sub};
use bevy::prelude::Vec2;

#[derive(Debug, Default, Eq, Hash, PartialEq, Clone, Copy)]
pub struct Vec2I {
  pub x: i8,
  pub y: i8,
}

impl Add for Vec2I {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    Self { x: self.x + rhs.x, y: self.y + rhs.y }
  }
}
impl Sub for Vec2I {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    Self { x: self.x - rhs.x, y: self.y - rhs.y }
  }
}

impl Vec2I {
  pub fn new(x: i8, y: i8) -> Self {
    Self { x, y }
  }
  pub fn zero() -> Self {
    Self { x: 0, y: 0 }
  }
  pub fn one() -> Self {
    Self { x: 1, y: 1 }
  }
  pub fn unit_x() -> Self {
    Self { x: 1, y: 0 }
  }
  pub fn unit_y() -> Self {
    Self { x: 0, y: 1 }
  }
  pub fn splat(v: i8) -> Self {
    Self { x: v, y: v }
  }
  pub fn x(self) -> i8 {
    self.x
  }
  pub fn y(self) -> i8 {
    self.y
  }
  pub fn mut_x(&mut self) -> &mut i8 {
    &mut self.x
  }
  pub fn mut_y(&mut self) -> &mut i8 {
    &mut self.y
  }
  pub fn abs(self) -> Self {
    Self { x: self.x.abs(), y: self.y.abs() }
  }
  fn smallest_square(self) -> i8 {
    let w = self.abs();
    if w.x < w.y {
      return w.x;
    }
    w.y
  }

  // D&D distance, because of course this is what the original game does...
  pub fn distance(self, w: Self) -> i8 {
    let x = (self - w).abs();
    let ss = x.smallest_square();
    let y = x - Self::splat(ss); // 1 is has 0 X or 0 Y so just add X and Y 'remainder'
    square_distance(ss) + y.x + y.y
  }
}

// Distance of diagonal - first square is 1, second is 3, third is 4 etc
fn square_distance(i: i8) -> i8 {
  if i == 0 {
    return 0;
  }
  i + (f64::from(i)/2.0).floor() as i8
}

impl From<Vec2> for Vec2I {
  fn from(v: Vec2) -> Self {
      Self{ x: v.x.trunc() as i8, y: v.y.trunc() as i8 }
  }
}

impl From<Vec2I> for Vec2 {
  fn from(v: Vec2I) -> Self {
    Self{ x: f32::from(v.x), y: f32::from(v.y) }
}
}

mod tests {
  use bevy::prelude::Vec2;
  use super::Vec2I;

  #[test]
  fn new() {
    let v = Vec2I::new(1, 2);
    assert!(v.x == 1);
    assert!(v.y == 2);
  }

  #[test]
  fn zero() {
    let v = Vec2I::zero();
    assert!(v.x == 0);
    assert!(v.y == 0);
  }

  #[test]
  fn one() {
    let v = Vec2I::one();
    assert!(v.x == 1);
    assert!(v.y == 1);
  }

  #[test]
  fn unit_x() {
    let v = Vec2I::unit_x();
    assert!(v.x == 1);
    assert!(v.y == 0);
  }

  #[test]
  fn unit_y() {
    let v = Vec2I::unit_y();
    assert!(v.x == 0);
    assert!(v.y == 1);
  }

  #[test]
  fn splat() {
    let v = Vec2I::splat(2);
    assert!(v.x == 2);
    assert!(v.y == 2);
  }

  #[test]
  fn x() {
    let v = Vec2I::unit_x();
    assert!(v.x == 1);
    assert!(v.x() == 1);
  }

  #[test]
  fn y() {
    let v = Vec2I::unit_y();
    assert!(v.y == 1);
    assert!(v.y() == 1);
  }

  #[test]
  fn mut_x() {
    let mut v = Vec2I::unit_x();
    {
      let x = v.mut_x();
      *x = 2;
    }
    assert!(v.x == 2);
  }

  #[test]
  fn mut_y() {
    let mut v = Vec2I::unit_y();
    {
      let y = v.mut_y();
      *y = 2;
    }
    assert!(v.y == 2);
  }

  #[test]
  fn from() {
      let f = Vec2{ x: 1.0, y: 1.0 };
      let v = Vec2I::from(f);
      assert!(v.x == 1);
      assert!(v.y == 1);
  }

  #[test]
  fn into() {
      let v = Vec2I { x: 1, y: 1 };
      let f: Vec2 = v.into();
      assert!((f.x - 1.0).abs() < f32::EPSILON);
      assert!((f.y - 1.0).abs() < f32::EPSILON);
  }

  #[test]
  fn square_distance() {
    let one = super::square_distance(1);
    assert!(one == 1);
    let two = super::square_distance(2);
    assert!(two == 3);
    let three = super::square_distance(3);
    assert!(three == 4);
  }

  #[test]
  fn add() {
    let w = Vec2I::new(1, 2) + Vec2I::new(4, 8);
    assert!(w.x == 5);
    assert!(w.y == 10);
  }

  #[test]
  fn sub() {
    let w = Vec2I::new(1, 2) - Vec2I::new(4, 8);
    assert!(w.x == -3);
    assert!(w.y == -6);
  }

  #[test]
  fn abs() {
    let w = Vec2I::new(1, 2).abs();
    assert!(w.x == 1);
    assert!(w.y == 2);
    let v = Vec2I::new(-1, -2).abs();
    assert!(v.x == 1);
    assert!(v.y == 2);
  }

  #[test]
  fn smallest_square() {
    let w = Vec2I::new(1, 2).smallest_square();
    assert!(w == 1);
    let v = Vec2I::new(2, 1).smallest_square();
    assert!(v == 1);
  }

  #[test]
  fn distance() {
    assert!(Vec2I::zero().distance(Vec2I::zero()) == 0);
    assert!(Vec2I::zero().distance(Vec2I::one()) == 1);
    assert!(Vec2I::zero().distance(Vec2I::splat(2)) == 3);
    assert!(Vec2I::zero().distance(Vec2I::splat(3)) == 4);
    assert!(Vec2I::zero().distance(Vec2I::splat(4)) == 6);
    assert!(Vec2I::zero().distance(Vec2I::new(10, 4)) == 12);
  }
}
