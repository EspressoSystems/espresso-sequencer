use std::ops::{Range, RangeBounds, RangeInclusive};

// TODO we can take an <impl RangeBound> generic param in order to support
// this w/ `RangeInclusive` and friends.
/// Type to generate chunks of `chunk_size` from a given range.
pub struct ChunkGenerator {
    range: Range<u64>,
    chunk_size: u64,
    next: Option<Range<u64>>,
}

impl ChunkGenerator {
    /// Given a `start`, `end` and `chunk_size`, create a `ChunkGenerator`.
    /// As with `Range`, inclusive `start`, exclusive `end`.
    /// Panics if `chunk_size > end`.
    // TODO if we make type generic we would take `Range` types here too.
    pub fn new(start: u64, end: u64, chunk_size: u64) -> Self {
        // TODO maybe instead of panic, behave as an iterator with a
        // single item of start..end
        if end < chunk_size {
            panic!("End bound must be greater than chunk_size");
        }
        let range = start..end;
        Self {
            range,
            chunk_size,
            next: Some(start..chunk_size),
        }
    }
}

impl Iterator for ChunkGenerator {
    type Item = Range<u64>;
    /// Generate the next chunk.
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next.clone()?;
        let next_end = current.end + self.chunk_size;

        if next_end < self.range.end {
            self.next = Some(current.end..next_end);
        } else if current.end < self.range.end {
            self.next = Some(current.end..self.range.end);
        } else {
            self.next = None
        }
        Some(current.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generator() {
        let mut g = ChunkGenerator::new(1, 11, 3);

        assert_eq![Some(1..3), g.next()];
        assert_eq![Some(3..6), g.next()];
        assert_eq![Some(6..9), g.next()];
        assert_eq![Some(9..11), g.next()];
        assert_eq![None, g.next()];
    }
    #[test]
    #[should_panic]
    fn test_generator_panics() {
        ChunkGenerator::new(1, 3, 4);
    }
}
