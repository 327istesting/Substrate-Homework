fn main() {
    fn bubble_sort(arr: &mut [i32]) {
        let mut p = arr.len() - 1;
        while p > 0{
            for i in 0..p{
                if arr[i] > arr[i+1]{
                    let temp = arr[i];
                    arr[i] = arr[i+1];
                    arr[i+1] = temp;
                }
            }
            p -= 1;
        }
    }


    fn bubble_sort_2<T: PartialOrd+Clone>(arr: &mut [T]) {
        let mut p = arr.len() - 1;
        while p > 0{
            for i in 0..p{
                if arr[i] > arr[i+1]{
                    let temp1 = arr[i].clone();
                    let temp2 = arr[i+1].clone();
                    arr[i] = temp2;
                    arr[i+1] = temp1;
                }
            }
            p -= 1;
        }
    }

    let mut test = [32, 1, 78, 33, 178, 256, 74];

    println!("original array: {:?}", test);

    bubble_sort(&mut test);

    println!("sorted array: {:?}", test);
    



    let mut test_2 = ["xyyz", "xj7", "326", "sdg", "327", "xz"];

    println!("Using Generics - original array: {:?}", test_2);

    bubble_sort_2(&mut test);

    println!("Using Generics - sorted array: {:?}", test_2);
}
