use core::{
    fmt::Display,
    ops::{Add, Sub},
};
use std::usize;

/// [start, stop)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Span {
    pub start: BytePos,
    pub stop: BytePos,
}

impl Span {
    pub(crate) fn new(start: BytePos, stop: BytePos) -> Self {
        Self { start, stop }
    }

    #[inline]
    pub(crate) const fn dummy() -> Self {
        Self {
            start: BytePos(0),
            stop: BytePos(0),
        }
    }

    #[inline]
    pub fn is_dummy(&self) -> bool {
        self == &Self::dummy()
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("need to print span from source map")
    }
}

impl Add for Span {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.start.min(rhs.start), self.stop.max(rhs.stop))
    }
}

impl Add for &Span {
    type Output = Span;

    fn add(self, rhs: Self) -> Self::Output {
        *self + *rhs
    }
}

// Is this necessary??
pub trait Pos: From<usize> + From<u32> + Into<usize> + Into<u32> + Add + Sub {}

macro_rules! impl_pos {
  (
      $(
          $(#[$attr:meta])*
          $vis:vis struct $ident:ident($inner_vis:vis $inner_ty:ty);
      )*
  ) => {
      $(
          $(#[$attr])*
          $vis struct $ident($inner_vis $inner_ty);

          impl From<usize> for $ident {
            fn from(n: usize) -> Self {
              $ident(n as $inner_ty)
            }
          }

          impl From<$ident> for usize {
            fn from(n: $ident) -> Self {
              n.0 as usize
            }
          }

          impl From<u32> for $ident {
            fn from(n: u32) -> Self {
              $ident(n as $inner_ty)
            }
          }

          impl From<$ident> for u32 {
            fn from(n: $ident) -> Self {
              n.0 as u32
            }
          }

          impl Add for $ident {
              type Output = $ident;

              #[inline(always)]
              fn add(self, rhs: $ident) -> $ident {
                  $ident(self.0 + rhs.0)
              }
          }

          impl Sub for $ident {
              type Output = $ident;

              #[inline(always)]
              fn sub(self, rhs: $ident) -> $ident {
                  $ident(self.0 - rhs.0)
              }
          }

          impl Pos for $ident {

          }
      )*
  };
}

impl_pos! {
  /// A byte offset.
  #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
  pub struct BytePos(pub u32);

  /// A character offset.
  ///
  /// Because of multibyte UTF-8 characters,
  /// a byte offset is not equivalent to a character offset.
  #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
  pub struct CharPos(pub usize);
}
