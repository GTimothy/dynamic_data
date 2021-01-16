pub trait DataLoader<T> {
    fn get_next_n(&self, n: isize, from: isize) -> Vec<T>;
    fn get_previous_n(&self, n: isize, from: isize) -> Vec<T>;
}

pub struct DynamicData<T, D>
where
    D: DataLoader<T>,
{
    current_start: isize,
    capacity: usize,
    data_source: D,
    data: Vec<T>,
}

impl<T, D: DataLoader<T>> DynamicData<T, D> {
    pub fn new(data_source: D) -> Self {
        DynamicData {
            current_start: 0,
            capacity: 40,
            data_source,
            data: Vec::new(),
        }
    }
    pub fn current_start(mut self, s: isize) -> Self {
        self.current_start = s;
        self
    }

    pub fn capacity(mut self, k: usize) -> Self {
        self.capacity = k;
        self
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn fetch_next(&mut self, n: isize) {
        let from = self.current_start + (self.data.len() as isize);
        let data_to_add = self.data_source.get_next_n(n, from);
        let over_capacity = (data_to_add.len() + self.data.len()) as isize - self.capacity as isize;

        if over_capacity > 0 {
            self.data = self.data.split_off(over_capacity as usize);
            self.current_start += over_capacity as isize;
        }

        self.data.extend(data_to_add);
    }

    pub fn fetch_previous(&mut self, n: isize) {
        let from = self.current_start;
        let mut data_to_add = self.data_source.get_previous_n(n, from);
        self.current_start -= data_to_add.len() as isize;
        self.data.reverse();
        data_to_add.reverse();
        self.data.extend(data_to_add);
        self.data.reverse();
        self.data.truncate(self.capacity);
    }
}

#[cfg(test)]
mod tests {
    struct DummyEntryLoader();

    #[derive(Debug, PartialEq, Eq)]
    struct Entry(isize);

    impl crate::DataLoader<Entry> for DummyEntryLoader {
        fn get_next_n(&self, n: isize, from: isize) -> Vec<Entry> {
            (from..from + n).map(|i| Entry(i)).collect()
        }

        fn get_previous_n(&self, n: isize, from: isize) -> Vec<Entry> {
            (from - n..from).map(|i| Entry(i)).collect()
        }
    }

    #[test]
    fn next10() {
        let mut dyn_data = crate::DynamicData::new(DummyEntryLoader {}).capacity(100);
        dyn_data.fetch_next(10);
        assert_eq!(*dyn_data.data(), (0..10).map(| i | Entry(i)).collect::<Vec<Entry>>())
    }
    #[test]
    fn prev10() {
        let mut dyn_data = crate::DynamicData::new(DummyEntryLoader {}).capacity(100);
        dyn_data.fetch_previous(10);
        assert_eq!(*dyn_data.data(), (-10..0).map(| i | Entry(i)).collect::<Vec<Entry>>())
    }

    #[test]
    fn next1000() {
        let mut dyn_data = crate::DynamicData::new(DummyEntryLoader {}).capacity(100);
        dyn_data.fetch_next(1000);
        assert_eq!(*dyn_data.data(), (900..1000).map(| i | Entry(i)).collect::<Vec<Entry>>())
    }
}
