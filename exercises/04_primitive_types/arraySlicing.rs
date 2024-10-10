// Characters (`char`)

fn main() {
    // Note the _single_ quotes, these are different from the double quotes

    let list = [1, 2, 3, 4, 5];

    let a1 = &list[0..3];
    let a2 = &list[0..=2];

    println!("element in line : ");

    for element in a1.iter() {
        println!("{} ", element);
    }

    println!("khalhglbqie gl") ; 
    for i in 0..5 {
        println!("{}", &a2[i]);
    }

    // fn slice_out_of_array() {
    //     let a = [1, 2, 3, 4, 5];
    //     //       0  1  2  3  4  <- indices
    //     //          -------
    //     //             |
    //     //             +--- slice

    //     // Note that the upper index 4 is excluded.
    //     let nice_slice = a[1..4];
    //     assert_eq!([2, 3, 4], nice_slice);

    //     // The upper index can be included by using the syntax `..=` (with `=` sign)
    //     let nice_slice = a[1..=3];
    //     assert_eq!([2, 3, 4], nice_slice);
    // }
}
