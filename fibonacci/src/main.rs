fn fibonacci(n: u32) -> u32 {
    // Define a variable to store the previous Fibonacci number
    let mut prev1 = 0;
  
    // Define a variable to store the current Fibonacci number
    let mut prev2 = 1;
  
    // Use a for loop to iterate over the numbers from 0 to n
    for _ in 0..n {
      // Compute the next Fibonacci number by adding the previous two numbers
      let current = prev1 + prev2;
  
      // Update the previous two numbers
      prev1 = prev2;
      prev2 = current;
    }
  
    // Return the current (nth) Fibonacci number
    prev2
}


fn main() {
    let n = 5;

    let result = fibonacci(n);
    println!("The {}th Fibonacci number is {}", n, result); // Output: The 5th Fibonacci number is 3

}
