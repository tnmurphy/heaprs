// Little heap sort demo to learn
// rust.
// (c) T N Murphy.  This software is in the Public Domain


pub struct Heap {
    array: Vec<i32>,
}


impl Heap {
    pub fn sortup(&mut self) {
        let mut index = self.array.len() - 1;

        while index > 0 {
            let parent = index >> 1;
            if self.array[index] < self.array[parent] {
                self.array.swap(index, parent);
            }
            index = parent;
        }
    }

    pub fn add(&mut self, item: i32) {

        self.array.push(item);
        self.sortup();
    }

    pub fn pop(&mut self) -> i32 {
        let end = self.array.len();

        //          0
        //     1          2
        //   3   4     5     6
        //  7 8 9 10 11 12 13 14
        //


        let item = self.array[0];
        let mut parent = 0;
        while parent < end {
            let left_child = (parent << 1) + 1;
            if left_child >= end {
                break;
            }

            let right_child = left_child + 1;
            if right_child >= end {
                break;
            }

            if self.array[left_child] < self.array[right_child] {
                self.array[parent] = self.array[left_child];
                parent = left_child;
            } else {
                self.array[parent] = self.array[right_child];
                parent = right_child;
            }
        }
        self.array.remove(parent);
        return item;
    }
}

#[cfg(test)]
mod test {

    use ::Heap;

    #[test]
    fn basic() {
        let h = &mut Heap { array: Vec::<i32>::with_capacity(20) };
        let numbers = vec![1i32, 4, 7, 1, 9, 12, 10, 3, 2, 8, 7];
        let sorted_numbers = vec![1i32, 1, 2, 3, 4, 7, 7, 8, 9, 10, 12];
        println!("The unsorted list:");
        for item in &numbers {
            println!("{}", item);
        }

        for item in &numbers {
            println!("Insertng {}", item);
            h.add(*item);
            for item2 in &h.array {
                println!("{}", item2);
            }
        }
        println!("Now popping in order");
        for index in 0..h.array.len() {
            let item = h.pop();
            let expected = sorted_numbers[index];
            println!("{}-{}", item, expected);
            assert_eq!(item, expected);
        }
    }
}
