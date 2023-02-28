fn test(my_vec: &mut Vec<u32>)-> &mut Vec<u32>{
    // Write code here!

    // remove last and then the middle element.
    let middle = (my_vec.len())/2;
    my_vec.pop();
    // Then insert the sum of the remaining elements to the end of the resulting vector.
    let mut value : u32 = (middle - 1).try_into().unwrap();
    let index = my_vec.iter().position(|&r| r == value).unwrap();
    // call the built-in remove method
    my_vec.remove(index);
    my_vec.push(value);
    my_vec

}

// working
fn test2(my_vec: &mut Vec<u32>)-> &mut Vec<u32>{
    let middle = (my_vec.len())/2;
    my_vec.pop();
    my_vec.remove(middle - 1);
    let mut sum : u32 = 0;
    for v in my_vec.iter()
    {
        sum = sum + v;
    }
    my_vec.push(sum);
    my_vec
}