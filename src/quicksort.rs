// TASK 2: Quicksort

pub fn quicksort(array: &mut[u32])
{
    if array.len() < 2 { return }
    let pivot = 1;
    let mut left = 0;
    let mut right = array.len()-1;
    while left <= right
    {
        while array[left] < array[pivot]
        {
            left += 1;
        }
        while array[right] > array[pivot]
        {
            right -= 1;
        }
        if left <= right
        {
            array.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
    quicksort(&mut array[0..=right]);
    quicksort(&mut array[left..]);
}
