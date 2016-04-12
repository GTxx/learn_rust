pub fn fib(n: u64) -> u64 {
    if n == 1 || n == 2 {
        return 1
    }else{
        return fib(n-1) + fib(n-2)
    }
}