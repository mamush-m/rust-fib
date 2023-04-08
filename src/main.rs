fn main() {

    fib(3);
    // fib_rec(71);
}

fn fib(n: i32) -> i32 {
    /*
     0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89... // Fib number
     1, 2, 3, 4, 5, 6, 7, 08, 09, 10, 11, 12... // Number in sequence
    */ 

    let mut a: i32 = 0;
    let mut b: i32 = 1;

    if n < 0 {
        println!("incorrect input");
        return n;
    }else if n == 0  {
        println!("{a}");
        return n;
    }else if n == 1 {
        println!("{b}");
        return n;
    }else if n > 1 {
        for i in 1..n {
        //    let c = a + b;
        //    a = b;
        //    b = c;
        //    println!("The Fibonacci number at position {i} is {c}");
        //    return b;
        println!("{}", i);
        // return i;
        }
        return b;
    }else {
        return -1;
    }
}

// fn fib_rec(n: i32) {
//     if n < 0 {
//         println!("Invalid input");
//     }
//     else if n == 1 {
//         println!("1")
//     }
//     else {
//         let mut c = (n - 1) + (n-2);
//         println!("haha claus {c}");
//     }
// }