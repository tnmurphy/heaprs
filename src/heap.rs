// Little heap sort demo to learn
// rust.
// (c) T N Murphy.  This software is in the Public Domain


pub struct Heap {
    array: Vec<i32>,
}


impl Heap {
    pub fn sortup(&mut self) {
        let mut index = self.array.len();
        if index == 0 { return; }
        index -= 1;

        while index > 0 {
            let parent = (index - 1) >> 1;
            if self.array[index] < self.array[parent] {
                self.array.swap(index, parent);
            }
            index = parent;
        }
    }

    pub fn print(&mut self) {
        let mut index = 0;
        let mut level_index = 0;
        let end = self.array.len();
        let mut power_of_two = 1;

        println!("Heap: ");
        while index < end {
            print!("{} ",self.array[index]);
            if index == level_index {
                level_index  += 1 << power_of_two;
                power_of_two += 1;
                println!("");
            }
            index += 1;
        }
        println!("\n------\n");
    }

    pub fn add(&mut self, item: i32) {

        self.array.push(item);
        self.sortup();
    }

    pub fn pop(&mut self) -> i32 {
        let end = self.array.len()-1;

        //             0
        //       1           2         2
        //    3     4     5     6      4
        //  07 08 09 10 11 12 13 14    8 
        //15 

        let item = self.array[0];
        self.array[0] = self.array[end];
        self.array.remove(end);

        let mut parent = 0;
        while parent < end {
            let left_child = (parent << 1) + 1;
            let right_child = left_child + 1;

            if left_child >= end {
                break;
            }
            
            if right_child >= end {
                if self.array[parent] > self.array[left_child] {
                    self.array.swap(parent, left_child);
                    parent = left_child;
                }
                break;
            }

            if self.array[left_child] > self.array[right_child] {
                if self.array[parent] > self.array[right_child] {
                    self.array.swap(parent, right_child);
                    parent = right_child;
                    continue;
                }
            } else {
                if self.array[parent] > self.array[left_child] {
                    self.array.swap(parent, left_child);
                    parent = left_child;
                    continue;
                }
            }
            break;
        }
        return item;
    }
}

#[cfg(test)]
mod test {

    use ::Heap;
    extern crate rand;
    use std::vec::Vec;

    fn check_order(numbers: &[i32], expected: &[i32]) {
        let h = &mut Heap { array: Vec::<i32>::with_capacity(numbers.len()+1) };
        

        // Push the input onto the heap
        for item in numbers {
            println!("----push {}", item);
            h.print();
            h.add(*item);
            h.print();
            println!("----pushed");
        }

        // pop the numbers off the heap in order
        println!("Popping:");
        for index in 0..h.array.len() {
            let item = h.pop();

            h.print();
            let expected = expected[index];
            if item != expected {
                println!("index {}:  got {}, expected {}", index, item, expected);
            } else {
                println!("index {}:  got {}", index, item);
            }
            assert_eq!(item, expected);
        }

    }

     #[test]
     fn basic() {
         let numbers = vec![1i32, 4, 7, 1, 9, 12, 10, 3, 2, 8, 7];
         let sorted_numbers = vec![1i32, 1, 2, 3, 4, 7, 7, 8, 9, 10, 12];
         check_order(&numbers, &sorted_numbers);
     }
  
     #[test]
     fn one() {
         let numbers = vec![1i32];
         let sorted_numbers = vec![1i32];
         check_order(&numbers, &sorted_numbers);
     }
  
     #[test]
     fn reverse2() {
         let numbers = vec![2i32, 1];
         let sorted_numbers = vec![1i32, 2];
         check_order(&numbers, &sorted_numbers);
     }
  
     #[test]
     fn reverse3() {
         let numbers = vec![3i32, 2, 1];
         let sorted_numbers = vec![1i32, 2, 3];
         check_order(&numbers, &sorted_numbers);
     }
  
     #[test]
     fn reverse4() {
         let numbers = vec![4i32, 3, 2, 1];
         let sorted_numbers = vec![1i32, 2, 3, 4];
         check_order(&numbers, &sorted_numbers);
     }
  
     #[test]
     fn presorted2() {
         let numbers = vec![1i32, 2];
         let sorted_numbers = vec![1i32, 2];
         check_order(&numbers, &sorted_numbers);
     }
  
     #[test]
     fn presorted3() {
         let numbers = vec![1i32, 2, 3];
         let sorted_numbers = vec![1i32, 2, 3];
         check_order(&numbers, &sorted_numbers);
     }
  
     #[test]
     fn presorted4() {
         let numbers = vec![1i32, 2, 3, 4];
         let sorted_numbers = vec![1i32, 2, 3, 4];
         check_order(&numbers, &sorted_numbers);
     }
  
     #[test]
     fn crossing0() {
         let numbers = vec![1i32, 2, -1, 0];
         let sorted_numbers = vec![-1i32, 0, 1, 2];
         check_order(&numbers, &sorted_numbers);
     }
  
     #[test]
     fn poptest() {
         let numbers = vec![8,6,7,5,2,3,4,1];
         let sorted_numbers = vec![1,2,3,4,5,6,7,8];
         check_order(&numbers, &sorted_numbers);
     }

     #[test]
     fn random1() {
         let rng = rand::rng();
         let mut numbers = [0i32; 40];
         rand::fill(&mut numbers);
         for i in 0..numbers.len() {
             numbers[i] %= 10000;
             numbers[i] = numbers[i].abs();
         }
         let mut sorted_numbers = numbers.clone();
         sorted_numbers.sort();
         check_order(&numbers, &sorted_numbers);
     }
}
