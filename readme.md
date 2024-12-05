# Ackermann function in C, Rust, and Python

Ackermann function is an (probably the first) example of a computable function that is fundamentally recursive. What this means is that it is not possible to recast the Ackermann function in terms of for loops (unlike, say, the Fibonacci series).

The function is an example of computational complexity that blows up quite rapidly. 
Mathematically, Ackermann function is defined by:

$$ A(0,n) = n + 1 $$
$$ A(m,0) = A(m-1, 1) $$
$$ A(m,n) = A(m-1, A(m-1, n - 1)) $$

The recursive call to Ackermann in the third condition is what makes this function so nasty! The fact that the arguments to the Ackermann function in each recursion decreases imply that the function will end (i.e. the Ackermann function is computable). 

## Usage
A simple
```
make 
```
in the root directory will create C (ackermann_c) and Rust (ackermann_rust) binaries.

Pass on the arguments to the Ackermann function as:
```
./ackermann_rust m n
```
For the python implementation:
```
python src/Ackermann.py m n
```
