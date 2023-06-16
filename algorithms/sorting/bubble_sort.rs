fn bubble_sort<T: Ord>(array: &mut [T]) {
  let len = array.len();
  let mut swapped = true;

  while swapped {
      swapped = false;

      for i in 0..len - 1 {
          if array[i] > array[i + 1] {
              array.swap(i, i + 1);
              swapped = true;
          }
      }
  }
}

fn main() {
  let mut array = vec![5, 3, 8, 4, 2];
  bubble_sort(&mut array);
  println!("{:?}", array); // Saída: [2, 3, 4, 5, 8]
}