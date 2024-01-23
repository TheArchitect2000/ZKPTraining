function func1(n,a,b) {
    var c;

    // fibonacci sequence
    // for (var i = 2; i < n; i++) {
    //     c = a + b;
    //     a = b;
    //     b = c;
    // }
    if (n==1) {
        c = a;
    }
    else if (n==2) {
        c = b;
    }
    else {
        while (n > 2) {
            c = a + b;
            a = b;
            b = c;
            n = n - 1;
        }
    }

    //return the result
    return c;
}
