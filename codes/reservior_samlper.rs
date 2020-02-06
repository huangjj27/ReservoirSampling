use rand::random;

struct ReserviorSampler<Item> {
    n: usize,
    K: usize,
    samples: Vec<Item>,
}

impl<Item> ReserviorSampler<Item> {
    fn with_cap(K: usize) -> Self {
        Self {
            n: 0,
            K,
            samples: Vec::with_capacity(K),
        }
    }

    fn process(&mut self, it: Item) {
        self.n += 1;

        if self.n <= self.K {
            self.samples.push(it);
            return;
        }

        let r = random::<usize>() % self.n;
        if r < self.K {
            self.samples[r] = it;
        }
    }
}
