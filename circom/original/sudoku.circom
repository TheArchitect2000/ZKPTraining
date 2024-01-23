pragma circom  2.0.6;

template NonEqual() {
      signal input in0;
      signal input in1;
      // check that in0-in1 is nonzero
      signal inverse; 

      inverse <-- 1/(in0-in1);
      inverse*(in0-in1) === 1;
}

// all elements are unique in the array
template Distinct(n) {
     signal input in[n];
     component nonEqual[n][n];

     for(var i=0; i<n; i++){
        for(var j=0; j<i; j++){
           // // all elements are unique in signal in
           nonEqual[i][j] = NonEqual();
           nonEqual[i][j].in0 <== in[i];
           nonEqual[i][j].in1 <== in[j];
        }
    }
}

// Enforce that 0<=in<16
template Bits4() {
    signal input in;
    signal bits[4];
    var bitsum = 0;
    for(var i=0; i<4; i++){
       bits[i] <--(in>>i)&1;
       bits[i]*(bits[i]-1) === 0;
       bitsum = bitsum+2**i*bits[i];
    }
   bitsum === in;
}

// Enforce that 1<=in<=9
template OneToNine(){
      signal input in;
      component lowerBound = Bits4();
      component upperBound = Bits4();

      lowerBound.in <== in-1;
      upperBound.in <== in+6;
}

template Sudoku(n){
    signal input solution[n][n];
    //puzzle is the same, but a zero indicates a blank
    signal input puzzle[n][n];
    signal output solved;

    //ensure that each solution is in-range
    component inRange[n][n];
    for (var i=0; i<n; i++){
        for (var j=0; j<n; j++){
            inRange[i][j] = OneToNine();
            inRange[i][j].in <== solution[i][j];
        }
    } 
    //ensure that puzzle and solution agree
    for (var i=0; i<n; i++){
        for (var j=0; j<n; j++){
            puzzle[i][j]*(puzzle[i][j]-solution[i][j]) === 0;
        }
    } 
    //ensure uniqueness in Rows
    component distinctRow[n];
    for (var i=0; i<n; i++) {
       distinctRow[i] = Distinct(n);
       for (var j=0; j<n; j++) {
         distinctRow[i].in[j] <== solution[i][j];
       }
    }
    //ensure uniqueness in Columns
    component distinctCol[n];
    for (var i=0; i<n; i++) {
       distinctCol[i] = Distinct(n);
       for (var j=0; j<n; j++) {
         distinctCol[i].in[j] <== solution[j][i];
       }
    }
}
component main{public[puzzle]} = Sudoku(9);
