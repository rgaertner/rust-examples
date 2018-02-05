fn main() {
    let vec1 = vec![1,2,3];
    let vec2 = vec![4,5,6];

    // 'iter()' for vecs yields '&i32'. Destructure to 'i32'
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // 'into_iter()' for vecs yiels 'i32'. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1,2,3];
    let array2 = [4,5,6];

    // 'iter()' for arrays yields '&i32'
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // 'into_iter()' for arrays unusually yields '&i32'
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
    //println!("2 in array2: {}", array2.iter().into_iter().collect().into_iter().any(|x| x == 2));
    
    // Experimental section: Can we pass the argument to the closure without
    // an enforced reference?
    // first naive try: call 
    //println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));

    // second: try to cope with typings
    fn make_iterable_work<T>(collection: T, i: T::Item) -> bool 
        where T: IntoIterator, T::Item: std::cmp::Eq,
    {
        collection.into_iter().any(|x| x == i)
    }
    println!("2 in array2: {}", make_iterable_work(array2.into_iter(), &2));
    // since T::Item cannot be generally be dereferenced, I do not see a way how to get values from
    // an array iterator into the closure
}
