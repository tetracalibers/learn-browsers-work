use std::cmp::min;

#[derive(Debug)]
pub struct Stream<'a, T> {
  pub idx: usize,
  data: &'a [T],
}

impl<'a, T: Copy> Stream<'a, T> {
  pub fn current_cpy(&self) -> Option<T> {
    self.data.get(self.idx).copied()
  }
}

impl<'a, T: Eq + Copy> Stream<'a, T> {
  pub fn expect_and_skip(&mut self, expect: T) -> Option<T> {
    let c = self.current_cpy()?;
    if c == expect {
      self.advance();
      Some(c)
    } else {
      None
    }
  }

  pub fn expect_oneof_and_skip(&mut self, expect: &[T]) -> Option<T> {
    let c = self.current_cpy()?;

    if expect.contains(&c) {
      self.advance();
      return Some(c);
    }

    None
  }

  pub fn expect_and_skip_cond(&mut self, expect: T) -> bool {
    self.expect_and_skip(expect).is_some()
  }
}

impl<'a, T> Stream<'a, T> {
  pub fn new(data: &'a [T]) -> Stream<T> {
    Self { data, idx: 0 }
  }

  pub fn len(&self) -> usize {
    self.data.len()
  }

  pub fn data(&self) -> &[T] {
    self.data
  }

  pub fn advance(&mut self) {
    self.idx += 1;
  }

  pub fn advance_by(&mut self, step: usize) {
    self.idx += step;
  }

  pub fn current(&self) -> Option<&T> {
    self.data.get(self.idx)
  }

  pub fn is_eof(&self) -> bool {
    self.idx >= self.data.len()
  }

  pub fn slice(&self, from: usize, to: usize) -> &'a [T] {
    &self.data[from..to]
  }

  pub fn slice_checked(&self, from: usize, to: usize) -> &'a [T] {
    &self.data[from..min(self.data.len(), to)]
  }

  pub fn slice_len(&self, from: usize, len: usize) -> &'a [T] {
    self.slice_checked(from, self.idx + len)
  }
}
