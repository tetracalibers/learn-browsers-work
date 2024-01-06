use std::collections::VecDeque;

pub struct InputStream<T, I>
where
  T: Iterator<Item = I>,
  I: Clone,
{
  source: T,
  is_reconsume: bool,
  last_consumed: Option<I>,
  buffer: VecDeque<I>,
}

pub type CharInputStream<T> = InputStream<T, char>;

impl<T, I> InputStream<T, I>
where
  T: Iterator<Item = I>,
  I: Clone,
{
  pub fn new(source: T) -> Self {
    Self {
      source,
      is_reconsume: false,
      last_consumed: None,
      buffer: VecDeque::new(),
    }
  }

  pub fn next(&mut self) -> Option<I> {
    let is_reconsume = self.is_reconsume;
    self.is_reconsume = false;

    if is_reconsume {
      return self.last_consumed.clone();
    }

    self.consume_source_to_buffer();

    let consumed = self.buffer.pop_front();
    self.last_consumed = consumed.clone();
    consumed
  }

  fn consume_source_to_buffer(&mut self) {
    let consumed = self.source.next();

    if let Some(item) = consumed {
      self.buffer.push_back(item);
    }
  }
}
