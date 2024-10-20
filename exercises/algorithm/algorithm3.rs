/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: std::cmp::PartialOrd+ Clone>(array: &mut [T]){
	//TODO
    let len = array.len();
    let mut i = 1;

    while i < len { // 确保不会越界
        let key = array[i].clone();
        let mut j = i as isize - 1; // 使用 isize 处理负索引

        while j >= 0 && key < array[j as usize] { // 将 j 转为 usize 进行索引
            array[j as usize + 1] = array[j as usize].clone(); // 这里 j 是有效的
            j -= 1;
        }

        // 在这里需要确保 j + 1 仍然在数组范围内
        if ((j + 1) as usize) < len {
            array[(j + 1) as usize] = key;
        }
        i += 1;
    }
       

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}