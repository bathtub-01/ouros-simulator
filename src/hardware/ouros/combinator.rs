pub type Arity = u8;

pub enum Pat {
    X,
    At(Box<Pat>, Box<Pat>),
}

pub type Idx = u8;

// Static with computed value using const fn
static FIB_3: u64 = {
    const fn fib(n: u32) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            n => fib(n - 1) + fib(n - 2),
        }
    }

    fib(3)
};
