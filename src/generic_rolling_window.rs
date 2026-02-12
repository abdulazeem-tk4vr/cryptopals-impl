// Rolling window that stores at most capacity readings per sensor.
struct RollingWindow<T> {
    capacity: usize,
    items: Vec<T>,
}

impl<T> RollingWindow<T> {
    fn new(capacity: usize) -> Self {
        RollingWindow { capacity, items: Vec::new() }
    }

    // Push a new reading and drop the oldest when capacity is exceeded
    fn push(&mut self, value: T) {
        self.items.push(value);
        if self.items.len()> self.capacity {
            self.items.remove(0);
        }
        // todo!("Maintain capacity by removing the oldest element when necessary");
    }

    // Return how many readings are currently stored
    fn len(&self) -> usize {
        self.items.len()
    }

    // Return the newest reading without moving it out
    fn latest(&self) -> Option<&T> {
        self.items.last()
    }

    // Return an iterator over the stored values from oldest to newest
    fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
}

pub fn generic_rolling_window() {
    let mut window = RollingWindow::new(3);
    window.push("north");
    window.push("east");
    window.push("south");
    window.push("west");

    println!("Length: {}", window.len());
    println!("Latest: {:?}", window.latest());

    for direction in window.iter() {
        println!("Remaining direction: {}", direction);
    }
}
