pragma circom 2.0.6;

include "inc1.circom";

template fibtemplate2() {
    signal input tmp;
}

template fibonacci(n) {
    signal input fib1;
    signal input fib2;
    signal output fibn;

    var a = fib1;
    var b = fib2;

    //set the output
    fibn <== func1(n, fib1, fib2) * fib1;
    // Constants: 5 or 7
    // Linear Expressions: 5*fib1 + 7*fib2 + 12
    // Quadratic Expressions: (5*fib1+7*fib2+12) * (9*fib1+11*fib2+20) ----> OK 
    // Non Quadratic Expressions: (5*fib1+7*fib2+12) * (9*fib1+11*fib2+20) * fib1

    var carray[n];
    carray[0] = fib1;
    carray[1] = fib2;

    component fibtmp1 = fibtemplate2();
    fibtmp1.tmp <== fib1;
    
}

component main = fibonacci(1000);
//component main {public [fib1,fib2]} = fibonacci(1000);
