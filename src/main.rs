trait Poolable<T> {
    fn get_value(&self) -> &T;
}

struct Item<T> {
    value: T,
}

impl Poolable<Item<i8>> for Item<i8> {
    fn get_value(&self) -> &Item<i8> {
        &self
    }
}

struct Pool<T: Poolable<T>> {
    items: Vec<T>,
}

impl<T: Poolable<T>> Pool<T> {
    fn push(&mut self, t: T) {
        self.items.push(t);
    }

    fn pop(&mut self) -> Option<T> {
        if self.items.len() == 0 {
            return None;
        }
        self.items.pop()
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    let v: Vec<Item<i8>> = Vec::new();
    let mut pool: Pool<Item<i8>> = Pool { items: v };

    pool.push(Item { value: 1 });
    pool.push(Item { value: 2 });
    pool.push(Item { value: 3 });

    println!("pool len: {}", pool.len());

    loop {
        match pool.pop() {
            None => {
                println!("empty pool!");
                break;
            }
            Some(item) => println!("retrieved value {}", item.get_value().value),
        }
    }
}
