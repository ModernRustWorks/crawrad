//! Dense bitset over `Vec<u64>` representing validity masks and filter results.

/// A dense bitset with arbitrary bit length.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Bitmap {
    words: Vec<u64>,
    len: usize,
}

impl Bitmap {
    /// Creates a new bitmap of length `len` initialized to `value`.
    pub fn new(len: usize, value: bool) -> Self {
        if len == 0 {
            return Self {
                words: Vec::new(),
                len: 0,
            };
        }
        let num_words = len.div_ceil(64);
        let mut words = if value {
            vec![!0u64; num_words]
        } else {
            vec![0u64; num_words]
        };

        if value {
            let rem = len % 64;
            if rem != 0 {
                let mask = (1u64 << rem) - 1;
                let last = words.last_mut().expect("words is not empty");
                *last &= mask;
            }
        }

        Self { words, len }
    }

    /// Constructs a bitmap directly from its raw words and bit length.
    pub fn from_words(mut words: Vec<u64>, len: usize) -> Self {
        if len == 0 {
            return Self {
                words: Vec::new(),
                len: 0,
            };
        }
        let rem = len % 64;
        if rem != 0 {
            let mask = (1u64 << rem) - 1;
            if let Some(last) = words.last_mut() {
                *last &= mask;
            }
        }
        Self { words, len }
    }

    /// Returns the number of bits in the bitmap.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the bitmap contains zero bits.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the boolean value at index `i`.
    #[inline]
    pub fn get(&self, i: usize) -> bool {
        assert!(
            i < self.len,
            "bitmap index out of bounds: {} >= {}",
            i,
            self.len
        );
        let word = i / 64;
        let bit = i % 64;
        (self.words[word] & (1u64 << bit)) != 0
    }

    /// Sets the boolean value at index `i`.
    #[inline]
    pub fn set(&mut self, i: usize, value: bool) {
        assert!(
            i < self.len,
            "bitmap index out of bounds: {} >= {}",
            i,
            self.len
        );
        let word = i / 64;
        let bit = i % 64;
        if value {
            self.words[word] |= 1u64 << bit;
        } else {
            self.words[word] &= !(1u64 << bit);
        }
    }

    /// Returns the total number of set bits (ones).
    pub fn count_ones(&self) -> usize {
        self.words.iter().map(|&w| w.count_ones() as usize).sum()
    }

    /// Performs bitwise AND with another bitmap of identical length.
    pub fn and(&self, other: &Bitmap) -> Bitmap {
        assert_eq!(
            self.len, other.len,
            "bitmap length mismatch in AND operation"
        );
        let mut words = Vec::with_capacity(self.words.len());
        for (w1, w2) in self.words.iter().zip(other.words.iter()) {
            words.push(w1 & w2);
        }
        Bitmap {
            words,
            len: self.len,
        }
    }

    /// Performs bitwise OR with another bitmap of identical length.
    pub fn or(&self, other: &Bitmap) -> Bitmap {
        assert_eq!(
            self.len, other.len,
            "bitmap length mismatch in OR operation"
        );
        let mut words = Vec::with_capacity(self.words.len());
        for (w1, w2) in self.words.iter().zip(other.words.iter()) {
            words.push(w1 | w2);
        }
        Bitmap {
            words,
            len: self.len,
        }
    }

    /// Performs bitwise NOT, returning an inverted bitmap.
    pub fn not(&self) -> Bitmap {
        let mut words = Vec::with_capacity(self.words.len());
        for &w in &self.words {
            words.push(!w);
        }
        let rem = self.len % 64;
        if rem != 0
            && let Some(last) = words.last_mut()
        {
            let mask = (1u64 << rem) - 1;
            *last &= mask;
        }
        Bitmap {
            words,
            len: self.len,
        }
    }

    /// Returns an iterator yielding the indices of all set bits (ones).
    #[inline]
    pub fn ones(&self) -> BitmapOnes<'_> {
        BitmapOnes {
            words: &self.words,
            word_idx: 0,
            current_word: 0,
            total_len: self.len,
        }
    }

    /// Internal word slice access.
    #[inline]
    pub fn words(&self) -> &[u64] {
        &self.words
    }
}

/// Iterator over indices of set bits in a `Bitmap`.
pub struct BitmapOnes<'a> {
    words: &'a [u64],
    word_idx: usize,
    current_word: u64,
    total_len: usize,
}

impl<'a> Iterator for BitmapOnes<'a> {
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.current_word == 0 {
            if self.word_idx >= self.words.len() {
                return None;
            }
            self.current_word = self.words[self.word_idx];
            self.word_idx += 1;
        }

        let tz = self.current_word.trailing_zeros();
        let bit_idx = (self.word_idx - 1) * 64 + tz as usize;
        if bit_idx >= self.total_len {
            return None;
        }
        self.current_word &= self.current_word - 1;
        Some(bit_idx as u32)
    }
}

#[cfg(test)]
#[path = "test/bitmap/test.rs"]
mod test;
