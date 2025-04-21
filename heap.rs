fn left_child(item: usize)->usize {
    return item*2;
}

fn right_child(item: usize)->usize {
    return item*2+1;
}

fn sort(array: &mut Vec<i32>) {
    let mut range = array.len();

    let tail = range >> 1;
    range = range - tail;
    for item in 0usize..range {
        let lc = left_child(item);
        if array[item]> array[lc] {
            let temp = array[lc];
            array[lc] = array[item];
            array[item] = temp;
        }
    }
}

fn main() {
  let mut a = vec!(1i32, 2, 4, 6, 3);

  for item in &a {
     println!("{}",item);
  }

  sort(&mut a);
  println!("Sorted");
    for item in &a {
        println!("{}", item);
    }
}

