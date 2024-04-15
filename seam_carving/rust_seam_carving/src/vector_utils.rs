pub trait MinInSection {
    fn min(&self, start_index: usize, end_index: usize) -> Option<(usize, f32)>;
}

impl MinInSection for Vec<f32> {
    fn min(&self, start_index: usize, end_index: usize) -> Option<(usize, f32)> {
        let section = &self[start_index..=end_index];

        let min_index = start_index
            + section
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(i, _)| i)?;

        Some((min_index, section[min_index - start_index]))
    }
}

pub trait RemoveMultiple<T> {
    // Remove multiple indices
    fn remove_multiple(&mut self, indices: Vec<usize>);
    // Remove multiple pixels from a vector (by 3 items)
    fn remove_multiple_pixels(&mut self, indices: Vec<usize>);
}

impl<T> RemoveMultiple<T> for Vec<T> {
    fn remove_multiple(&mut self, mut indices: Vec<usize>) {
        indices.sort();
        indices.reverse();
        for index in indices {
            self.remove(index);
        }
    }

    fn remove_multiple_pixels(&mut self, indices: Vec<usize>) {
        let mut new_pixels = Vec::new();
        for i in indices.iter() {
            new_pixels.push(*i * 3);
            new_pixels.push((*i * 3) + 1);
            new_pixels.push((*i * 3) + 2);
        }
        self.remove_multiple(new_pixels);
    }
}
