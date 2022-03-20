#[derive(Debug)]
pub struct GapBuffer {
    buffer: Vec<char>,
    gap_pos: usize, // self.buffer[self.gap_pos]这个位置已经是gap了
    gap_len: usize,
    capacity: usize,
    len: usize,
}

impl Default for GapBuffer {
    fn default() -> Self {
        Self {
            buffer: Vec::new(),
            gap_pos: 0,
            gap_len: 0,
            capacity: 0,
            len: 0,
        }
    }
}

impl GapBuffer {
    pub fn from_string(s: String) -> Self {
        let buffer: Vec<char> = s.chars().collect();
        let len = buffer.len();
        let capacity = buffer.capacity();
        Self {
            buffer,
            gap_pos: len,
            len,
            capacity,
            gap_len: capacity - len,
        }
    }

    pub fn load_string(&mut self, s: String) {
        *self = Self::from_string(s);
    }

    pub fn iter<'b>(&'b self) -> GapBufferIterator<'b> {
        GapBufferIterator {
            gap_buffer: &self,
            current_index: 0,
        }
    }

    /// 自己假装成一个正常buffer的话，有多长呢
    pub fn content_len(&self) -> usize {
        if self.gap_pos >= self.len {
            self.len
        } else {
            self.len - self.gap_len
        }
    }

    /// 请检查index和content_len的关系后使用，不然panic掉
    pub fn get(&self, index: usize) -> char {
        if index < self.gap_pos {
            self.buffer[index]
        } else {
            self.buffer[index + self.gap_len]
        }
    }

    pub fn move_gap_to(&mut self, pos: usize) {
        let max_gap_pos = self.capacity - self.gap_len;
        let target_pos = pos.min(max_gap_pos);
        if target_pos == self.gap_pos {
            return;
        }
        self.really_move_gap_to(target_pos);
    }

    // 移动必然不导致capacity变化
    fn really_move_gap_to(&mut self, target_pos: usize) {
        if target_pos < self.gap_pos {
            let offset = self.gap_pos - target_pos;
            for i in (target_pos..self.gap_pos).rev() {
                self.buffer[i + offset] = self.buffer[i];
            }
        } else {
            // target_pos > self.gap_pos
            for i in self.gap_pos..target_pos {
                self.buffer[i] = self.buffer[i + self.gap_len];
            }
        }
    }
}

pub struct GapBufferIterator<'b> {
    current_index: usize,
    gap_buffer: &'b GapBuffer,
}

impl<'b> Iterator for GapBufferIterator<'b> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.gap_buffer.content_len() {
            let result = Some(self.gap_buffer.get(self.current_index));
            self.current_index += 1;
            result
        } else {
            None
        }
    }
}
