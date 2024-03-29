use std::{collections::VecDeque, iter::FromIterator};

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

  pub fn reconsume(&mut self) {
    self.is_reconsume = true;
  }

  pub fn peek(&mut self) -> Option<I> {
    if self.is_reconsume {
      return self.last_consumed.clone();
    }

    self.consume_source_to_buffer();
    self.buffer.front().cloned()
  }

  pub fn peek_next_repeat<S: FromIterator<I>>(
    &mut self,
    n: usize,
  ) -> Option<S> {
    self.consume_source_to_buffer();

    if self.buffer.len() < n {
      return None;
    }

    let n = if self.is_reconsume { n - 1 } else { n };

    let mut result =
      self.buffer.iter().take(n).cloned().collect::<VecDeque<I>>();

    if self.is_reconsume {
      if let Some(current) = &self.last_consumed {
        result.push_front(current.clone());
      }
    }

    Some(result.iter().cloned().collect())
  }

  pub fn peek_max(&mut self) -> Vec<I> {
    for item in self.source.by_ref() {
      self.buffer.push_back(item);
    }

    let mut result = self.buffer.iter().cloned().collect::<VecDeque<I>>();

    if self.is_reconsume {
      if let Some(current) = &self.last_consumed {
        result.push_front(current.clone());
      }
    }

    result.iter().cloned().collect()
  }
}
