// vecs2.rs
// A Vec of even numbers is given. Your task is to complete the loop
// so that each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.



fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for i in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        //*i * 2
        //let mut vec: Vec<i32> = Vec::new();
        //v.push(*i * 2)
        //*v[i] = *i * 2;
        *i *= 2;
        //v[*i] *= 2; 
        //println!("the value is {:?}", v);
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
    //vec
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|num| {
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        num * 2
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}


/* 
FOR FUNCTION vec_map
In Rust, v.iter() returns an iterator over the elements of the vector v, 
without actually modifying the vector. The iter() method returns an immutable 
reference to the vector's elements.


The map() method, which is called on the iterator returned by v.iter(), 
takes a closure as an argument and creates a new iterator that applies the closure to each element 
of the original iterator. In this case, the closure takes a single argument num, 
which represents the value of each element in the iterator, and multiplies it by 2 
using the expression num * 2.


Note that the map() method does not modify the original vector v; 
it only creates a new iterator that contains the transformed values. 
To collect the transformed values into a new vector, we need to call the collect() 
method on the resulting iterator.




 */

/* 

In Rust, a closure is a block of code that can capture variables from its surrounding environment and is 
defined inline with the function call. Closures are similar to anonymous functions in other programming languages.

In the code v.iter().map(|num| num * 2), 
the closure is defined as the argument to the map() method call.
 The closure is enclosed in vertical bars (|), which denote the arguments to the closure. 
 In this case, the closure takes a single argument num, which represents the value of each element in the iterator, 
 and multiplies it by 2 using the expression num * 2.

The closure captures the num variable from its surrounding environment, which is the iterator returned by v.iter(). 
This allows the closure to access the value of each element in the iterator and transform it using the expression num * 2.

Closures are a powerful feature of Rust that allow for functional programming patterns 
and can be used to create higher-order functions like map(), filter(), and fold().


 */


/* 
FOR vec_loop: 

In Rust, v.iter_mut() returns an iterator over mutable references to the elements of the vector v. 
The iter_mut() method returns an iterator that allows us to modify the elements of the vector.

The for loop that follows this iterator expression, for i in v.iter_mut() {...}, 
iterates over each element in the iterator and binds it to the mutable reference i. 
The iter_mut() method returns a mutable reference to each element of the vector in turn, 
allowing us to modify the element using the dereference operator *.


In this specific case, the loop body multiplies each element by 2 using the expression *i *= 2. 
This modifies the original element in the vector, doubling its value. 
Once the loop is complete, the modified vector is returned by the function.

Note that using iter_mut() allows us to modify the elements of the vector in place, without creating a new vector. 
This can be more efficient than creating a new vector using map() or other methods that require allocating new memory.


 */