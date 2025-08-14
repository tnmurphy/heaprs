fn left_child(item: usize)->usize {
    return item*2;
}

fn right_child(item: usize)->usize {
    return item*2+1;
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

  for item in &a {
     println!("{}",item);
  }

  for item in &a {
      b.push(*item);
      sort(&mut b);
      println!("Inserted {}", item);
      for item2 in &b {
        println!("{}", item2);
      }
  }
}

