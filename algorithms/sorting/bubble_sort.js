function bubbleSort(array) {
  let len = array.length;
  let swapped;
  
  do {
    swapped = false;
    
    for (let i = 0; i < len-1; i++) {
      if (array[i] > array[i+1]) {
        let temp = array[i];
        array[i] = array[i+1];
        array[i+1] = temp;
        swapped = true;
      }
    }
  } while (swapped);

  return array;
}

let arr = [5, 3, 8, 4, 2];
console.log(bubbleSort(arr));