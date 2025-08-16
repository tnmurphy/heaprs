// Little heap sort demo to learn 
// rust. 
// (c) T N Murphy.  This software is in the Public Domain


fn sortup(array: &mut Vec<i32>) {
    let mut index = array.len()-1;

    while index > 0 {
        let parent = index >> 1;
        if array[index] < array[parent] {
            array.swap(index,parent);
        }
        index = parent;
    }
}

fn add(array: &mut Vec<i32>, item: i32) {

    array.push(item);
    sortup(array);
}

fn pop(array: &mut Vec<i32>) -> i32 {
    let end = array.len();

    //          0
    //     1          2
    //   3   4     5     6
    //  7 8 9 10 11 12 13 14
    //
    

    let item = array[0];
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

        if array[left_child] < array[right_child] {
            array[parent] = array[left_child];
            parent = left_child;
        } else {
            array[parent] = array[right_child];
            parent = right_child;
        }
    }
    array.remove(parent);
    return item;
}

fn main() {
  let a = vec!(1i32, 4,7,1,9,12,10,3,2,8,7);
  let mut b:Vec<i32> = Vec::<i32>::with_capacity(20);
  println!("The unsorted list:");
  for item in &a {
     println!("{}",item);
  }

  for item in &a {
      println!("Insertng {}", item);
      add(&mut b, *item);
      for item2 in &b {
        println!("{}", item2);
      }
  }
  println!("Now popping in order");
  for _ in 0..b.len() {
      let item = pop(&mut b);
      println!("{}", item);
  }
}

