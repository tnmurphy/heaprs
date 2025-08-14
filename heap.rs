fn left_child(item: usize)->usize {
    return item*2;
}

fn right_child(item: usize)->usize {
    return item*2+1;
}

fn sortup(array: &mut Vec<i32>) {
    let mut index = array.len()-1;

    while index > 0 {
        let parent = index >> 1;
        println!("index = {} parent={}", index, parent);
        if array[index] < array[parent] {
        println!("swap {} {}", array[index], array[parent]);
            array.swap(index,parent);
        }
        index = parent;
    }
}

fn add(array: &mut Vec<i32>, item: i32) {

    array.push(item);
    sortup(array);
}

fn sort(array: &mut Vec<i32>) {
    let range = array.len();

    for item in 0usize..range {
        let lc = left_child(item);
        if lc < range {
          if array[item] > array[lc] {
              array.swap(item,lc);
          }
        }
        let rc = right_child(item);
        if rc < range {
          if array[item] > array[rc] {
            array.swap(item,rc);
          }
        }
    }
}

fn main() {
  let a = vec!(1i32, 4,3,2);
  let mut b:Vec<i32> = Vec::<i32>::with_capacity(20);
  println!(" 3 >> 1 = {}", 3 >> 1);
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
}

