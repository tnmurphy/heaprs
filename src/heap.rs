// heap sort 
// Copyright (c) 2025 Timothy Norman Murphy
//
// Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation
// files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy,
// modify, merge, publish, distribute, sublicense, and/or sell copies
// of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY
// KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
// WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE
// AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
// HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// 


pub struct Heap<T> {
    array: Vec<T>,
}


impl<T: std::fmt::Display + Ord + Copy + Eq> Heap<T> {
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

    pub fn add(&mut self, item: T) {

        self.array.push(item);
        self.sortup();
    }

    pub fn pop(&mut self) -> T {
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

    fn check_order<T: std::fmt::Display + std::fmt::Debug>(items: &[T], expected: &[T]) where T: Copy, T: Eq, T: Ord {
        let h = &mut Heap { array: Vec::<T>::with_capacity(items.len()+1) };
        

        // Push the input onto the heap
        for item in items {
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
     fn stringsort() {
        let names = ["Zane", "John", "Alice", "Catherine", "Mercedes", "Bob"];
        let sorted_names = ["Alice", "Bob", "Catherine", "John", "Mercedes", "Zane"];
        check_order(&names, &sorted_names);
     }

     #[test]
     fn random1() {
         let mut numbers = [0i32; 4000];
         rand::fill(&mut numbers);
         for i in 0..numbers.len() {
             numbers[i] %= 100000;
             numbers[i] = numbers[i].abs();
         }
         let mut sorted_numbers = numbers.clone();
         sorted_numbers.sort();
         check_order(&numbers, &sorted_numbers);
     }
}
