pragma circom  2.0.6;

template fibonacci(n) {
   signal input fib1;
   signal input fib2;
   signal output fibn;

   // Set the first two numbers
   var a = fib1;
   var b = fib2;
   var c;

   // Compute Fibonacci sequence
    for (var i = 2; i <n; i++) {
        c = a + b;
        a = b;
        b = c;
    }

   // Set the output
   fibn <== c * fib1;

   log();
}

component main = fibonacci(1000);
