#![allow(unused_macros, unused_imports, dead_code)]
use std::any::TypeId;
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::mem::swap;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign};

macro_rules! __debug_impl {
    ($x:expr) => {
        eprint!("{}={}  ", stringify!($x), &$x);
    };
    ($x:expr, $($y:expr),+) => (
        __debug_impl!($x);
        __debug_impl!($($y),+);
    );
}
macro_rules! __debug_line {
    () => {
        eprint!("L{}  ", line!());
    };
}
macro_rules! __debug_select {
    () => {
        eprintln!();
    };
    ($x:expr) => {
        __debug_line!();
        __debug_impl!($x);
        eprintln!();
    };
    ($x:expr, $($y:expr),+) => (
        __debug_line!();
        __debug_impl!($x);
        __debug_impl!($($y),+);
        eprintln!();
    );
}
macro_rules! debug {
    () => {
        if cfg!(debug_assertions) {
            __debug_select!();
        }
    };
    ($($xs:expr),+) => {
        if cfg!(debug_assertions) {
            __debug_select!($($xs),+);
        }
    };
}

pub trait ChangeMinMax {
    fn chmin(&mut self, rhs: Self) -> bool;
    fn chmax(&mut self, rhs: Self) -> bool;
}
impl<T: PartialOrd + Copy> ChangeMinMax for T {
    fn chmin(&mut self, rhs: Self) -> bool {
        if *self > rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
    fn chmax(&mut self, rhs: Self) -> bool {
        if *self < rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub trait RepeatedSquaring {
    fn power(self, p: usize) -> Self;
}
impl<T: MulAssign + Div<Output = T> + Copy> RepeatedSquaring for T {
    fn power(self, mut p: usize) -> Self {
        #[allow(clippy::eq_op)]
        let one = (self / self) as Self;
        let mut ret: Self = one;
        let mut mul: Self = self;
        while p > 0 {
            if p & 1 != 0 {
                ret *= mul;
            }
            p >>= 1;
            mul *= mul;
        }
        ret
    }
}

fn factorial_impl<
    T: Clone + Copy + From<usize> + Into<usize> + Mul<Output = T> + Div<Output = T>,
>(
    p: usize,
    memo: &mut Vec<usize>,
    update_op: fn(T, T) -> T,
) -> T {
    while memo.len() < 2_usize {
        memo.push(1_usize);
    }
    while memo.len() <= p + 1 {
        let last_val: T = T::from(*memo.last().unwrap());
        memo.push(update_op(last_val, T::from(memo.len())).into());
    }
    T::from(memo[p])
}

fn factorial<
    T: Clone + Copy + From<usize> + Into<usize> + Mul<Output = T> + Div<Output = T> + 'static,
>(
    p: usize,
) -> T {
    static mut MEMO: Vec<usize> = Vec::<usize>::new();
    unsafe { factorial_impl(p, &mut MEMO, |x: T, y: T| x * y) }
}

fn factorial_inv<
    T: Clone + Copy + From<usize> + Into<usize> + Mul<Output = T> + Div<Output = T> + 'static,
>(
    p: usize,
) -> T {
    static mut MEMO: Vec<usize> = Vec::<usize>::new();
    unsafe { factorial_impl(p, &mut MEMO, |x: T, y: T| x / y) }
}

fn combination<
    T: Clone + Copy + From<usize> + Into<usize> + Mul<Output = T> + Div<Output = T> + 'static,
>(
    n: usize,
    k: usize,
) -> T {
    if k == 0 {
        return T::from(1_usize);
    } else if k == 1 {
        return T::from(n);
    } else if k == 2 {
        return (T::from(n) * T::from(n - 1)) / T::from(2);
    }

    if TypeId::of::<mint>() == TypeId::of::<T>() {
        factorial::<T>(n) * factorial_inv::<T>(k) * factorial_inv::<T>(n - k)
    } else {
        factorial::<T>(n) / (factorial::<T>(k) * factorial::<T>(n - k))
    }
}

fn permutation<
    T: Clone + Copy + From<usize> + Into<usize> + Mul<Output = T> + Div<Output = T> + 'static,
>(
    n: usize,
    k: usize,
) -> T {
    if k == 0 {
        return T::from(1_usize);
    } else if k == 1 {
        return T::from(n);
    } else if k == 2 {
        return T::from(n) * T::from(n - 1);
    }

    if TypeId::of::<mint>() == TypeId::of::<T>() {
        factorial::<T>(n) * factorial_inv::<T>(n - k)
    } else {
        factorial::<T>(n) / factorial::<T>(n - k)
    }
}

mod union_find {
    #[derive(Debug, Clone)]
    pub struct UnionFind {
        pub graph: Vec<Vec<usize>>,
        parents: Vec<usize>,
        grp_sz: Vec<usize>,
        grp_num: usize,
    }

    impl UnionFind {
        pub fn new(sz: usize) -> Self {
            Self {
                graph: vec![vec![]; sz],
                parents: (0..sz).collect::<Vec<usize>>(),
                grp_sz: vec![1; sz],
                grp_num: sz,
            }
        }
        pub fn root(&mut self, v: usize) -> usize {
            if v == self.parents[v] {
                v
            } else {
                self.parents[v] = self.root(self.parents[v]);
                self.parents[v]
            }
        }
        pub fn same(&mut self, a: usize, b: usize) -> bool {
            self.root(a) == self.root(b)
        }
        pub fn unite(&mut self, into: usize, from: usize) {
            self.graph[into].push(from);
            self.graph[from].push(into);
            let r_into = self.root(into);
            let r_from = self.root(from);
            if r_into != r_from {
                self.parents[r_from] = r_into;
                self.grp_sz[r_into] += self.grp_sz[r_from];
                self.grp_sz[r_from] = 0;
                self.grp_num -= 1;
            }
        }
        pub fn group_num(&self) -> usize {
            self.grp_num
        }
        pub fn group_size(&mut self, a: usize) -> usize {
            let ra = self.root(a);
            self.grp_sz[ra]
        }
    }
}
use union_find::UnionFind;

mod segment_tree {
    use std::ops::{Add, Sub};
    #[derive(Debug, Clone)]
    pub struct SegmentTree<T> {
        n2: usize,   // implemented leaf num (2^n)
        neff: usize, // effective vector length
        dat: Vec<T>,
        pair_op: fn(T, T) -> T,
    }
    impl<T: Copy + Add<Output = T> + Sub<Output = T>> SegmentTree<T> {
        pub fn new(n: usize, pair_op: fn(T, T) -> T, ini_val: T) -> Self {
            let mut n2 = 1_usize;
            while n > n2 {
                n2 *= 2;
            }
            let mut s = Self {
                n2,
                neff: n,
                pair_op,
                dat: vec![ini_val; 2 * n2 - 1],
            };

            for i in 0..n {
                s.set(i, ini_val);
            }
            s
        }
        pub fn new_from(pair_op: fn(T, T) -> T, ini_values: &Vec<T>) -> Self {
            let n = ini_values.len();
            let mut n2 = 1_usize;
            while n > n2 {
                n2 *= 2;
            }
            let mut st = Self {
                n2,
                neff: n,
                pair_op,
                dat: vec![ini_values[0]; 2 * n2 - 1],
            };

            for (i, ini_val) in ini_values.iter().enumerate() {
                st.set(i, *ini_val);
            }
            st
        }
        pub fn set(&mut self, mut pos: usize, val: T) {
            pos += self.n2 - 1;
            self.dat[pos] = val;
            while pos > 0 {
                pos = (pos - 1) / 2; // parent
                self.dat[pos] = (self.pair_op)(self.dat[pos * 2 + 1], self.dat[pos * 2 + 2]);
            }
        }
        pub fn get(&self, pos: usize) -> T {
            self.dat[pos + self.n2 - 1]
        }
        pub fn add(&mut self, pos: usize, add_val: T) {
            self.set(pos, self.get(pos) + add_val);
        }
        pub fn sub(&mut self, pos: usize, sub_val: T) {
            self.set(pos, self.get(pos) - sub_val);
        }
        // get query value of [a, b]
        pub fn query(&self, a: usize, b: usize) -> T {
            self.query_sub(a, b + 1, 0, 0, self.n2)
        }
        pub fn query_whole(&self) -> T {
            let a = 0;
            let b = self.neff;
            self.query_sub(a, b + 1, 0, 0, self.n2)
        }
        pub fn query_geq(&self, a: usize) -> T {
            let b = self.neff;
            self.query_sub(a, b + 1, 0, 0, self.n2)
        }
        pub fn query_leq(&self, b: usize) -> T {
            let a = 0;
            self.query_sub(a, b + 1, 0, 0, self.n2)
        }
        // get query value of [a, b)
        fn query_sub(&self, a: usize, b: usize, node: usize, node_l: usize, node_r: usize) -> T {
            if (node_r <= a) || (b <= node_l) {
                panic!("invalid query range, ({a}, {b})");
            } else if (a <= node_l) && (node_r <= b) {
                // this not is covered by given interval.
                self.dat[node]
            } else if a < (node_l + node_r) / 2 {
                let vl = self.query_sub(a, b, node * 2 + 1, node_l, (node_l + node_r) / 2);
                if (node_l + node_r) / 2 < b {
                    let vr = self.query_sub(a, b, node * 2 + 2, (node_l + node_r) / 2, node_r);
                    (self.pair_op)(vl, vr)
                } else {
                    vl
                }
            } else if (node_l + node_r) / 2 < b {
                self.query_sub(a, b, node * 2 + 2, (node_l + node_r) / 2, node_r)
            } else {
                panic!("invalid query range, ({a}, {b})");
            }
        }
    }
}
use segment_tree::SegmentTree;

mod lazy_segment_tree {
    pub struct LazySegmentTree<X, M> {
        // https://algo-logic.info/segment-tree/#toc_id_3
        n2: usize,                    // num of node(integer power of 2)
        pair_op: fn(X, X) -> X,       // node_val x node_val -> node_val
        update_op: fn(X, M) -> X,     // node_val x update_val -> node
        update_concat: fn(M, M) -> M, // update_val x update_val -> update_val
        dat: Vec<X>,                  // node values
        lazy_ops: Vec<Option<M>>,     // reserved operations
        built: bool,
    }
    impl<X: Copy, M: Copy> LazySegmentTree<X, M> {
        pub fn new(
            n: usize,
            pair_op: fn(X, X) -> X,
            update_op: fn(X, M) -> X,
            update_concat: fn(M, M) -> M,
            ini_val: X,
        ) -> Self {
            let mut n2 = 1_usize;
            while n > n2 {
                n2 *= 2;
            }
            let mut ret = Self {
                n2,
                pair_op,
                update_op,
                update_concat,
                dat: vec![ini_val; n * 4],
                lazy_ops: vec![None; n * 4],
                built: false,
            };
            ret.init_all(ini_val);
            ret
        }
        pub fn new_from(
            pair_op: fn(X, X) -> X,
            update_op: fn(X, M) -> X,
            update_concat: fn(M, M) -> M,
            init_vals: &[X],
        ) -> Self {
            let n = init_vals.len();
            let mut n2 = 1_usize;
            while n > n2 {
                n2 *= 2;
            }
            let mut ret = Self {
                n2,
                pair_op,
                update_op,
                update_concat,
                dat: vec![init_vals[0]; n * 4],
                lazy_ops: vec![None; n * 4],
                built: false,
            };
            for (i, init_val) in init_vals.iter().enumerate() {
                ret.set(i, *init_val);
            }
            ret
        }
        pub fn query(&mut self, a: usize, b: usize) -> X // closed interval
        {
            self.query_sub(a, b + 1, 0, 0, self.n2) // half_open interval
        }
        pub fn reserve(&mut self, a: usize, b: usize, m: M) // closed interval
        {
            self.reserve_sub(a, b + 1, m, 0, 0, self.n2); // half_open interval
        }
        pub fn set(&mut self, i: usize, val: X) {
            self.dat[i + self.n2 - 1] = val;
        }
        fn init_all(&mut self, ini_val: X) {
            for i in 0..self.n2 {
                self.set(i, ini_val);
            }
        }
        fn build(&mut self) {
            for k in (0..self.n2).rev().skip(1) {
                self.dat[k] = (self.pair_op)(self.dat[2 * k + 1], self.dat[2 * k + 2]);
            }
        }
        fn lazy_eval(&mut self, node: usize) {
            if !self.built {
                self.build();
                self.built = true;
            }
            if let Some(lazy_val) = self.lazy_ops[node] {
                if node < self.n2 - 1 {
                    // if the target node is not a terminal one, propagate to its' children.
                    for d in 1..=2_usize {
                        let nc = node * 2 + d;
                        if let Some(nc_lazy_val) = self.lazy_ops[nc] {
                            self.lazy_ops[nc] = Some((self.update_concat)(nc_lazy_val, lazy_val));
                        } else {
                            self.lazy_ops[nc] = Some(lazy_val);
                        }
                    }
                }
                // update the target node
                self.dat[node] = (self.update_op)(self.dat[node], lazy_val);
                self.lazy_ops[node] = None;
            }
        }
        fn reserve_sub(
            &mut self,
            a: usize,
            b: usize,
            m: M,
            node: usize,
            node_l: usize,
            node_r: usize,
        ) {
            self.lazy_eval(node);
            if (a <= node_l) && (node_r <= b) {
                // this node is inside the query range.
                if let Some(lazy_val) = self.lazy_ops[node] {
                    self.lazy_ops[node] = Some((self.update_concat)(lazy_val, m));
                } else {
                    self.lazy_ops[node] = Some(m);
                }
                self.lazy_eval(node);
            } else if (node_r > a) && (b > node_l) {
                // this node and query range overlap partly.
                self.reserve_sub(a, b, m, node * 2 + 1, node_l, (node_l + node_r) / 2); // 左の子
                self.reserve_sub(a, b, m, node * 2 + 2, (node_l + node_r) / 2, node_r); // 右の子
                self.dat[node] = (self.pair_op)(self.dat[node * 2 + 1], self.dat[node * 2 + 2]);
            }
        }
        fn query_sub(
            &mut self,
            a: usize,
            b: usize,
            node: usize,
            node_l: usize,
            node_r: usize,
        ) -> X {
            self.lazy_eval(node);
            if (a <= node_l) && (node_r <= b) {
                // this node is inside the query range.
                self.dat[node]
            } else if (node_r > a) && (b > node_l) {
                // this node and query range overlap partly.
                let n0 = node * 2 + 1;
                let l0 = node_l;
                let r0 = (node_l + node_r) / 2;
                let n1 = node * 2 + 2;
                let l1 = (node_l + node_r) / 2;
                let r1 = node_r;
                if (a < r0) && (l0 < b) {
                    if (a < r1) && (l1 < b) {
                        (self.pair_op)(
                            self.query_sub(a, b, n0, l0, r0),
                            self.query_sub(a, b, n1, l1, r1),
                        )
                    } else {
                        self.query_sub(a, b, n0, l0, r0)
                    }
                } else if (a < r1) && (l1 < b) {
                    self.query_sub(a, b, n1, l1, r1)
                } else {
                    panic!("invalid arg range {}, {}", a, b);
                }
            } else {
                panic!(
                    "this node and query range have no common area, {}, {}",
                    a, b
                );
            }
        }
    }
}
use lazy_segment_tree::LazySegmentTree;

mod modint {
    use std::fmt;
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign};
    static mut MOD: i64 = 2;

    #[derive(Clone, Copy)]
    pub struct ModInt {
        x: i64,
    }
    impl ModInt {
        pub fn set_prime(val: i64) {
            unsafe {
                MOD = val;
            }
        }
        fn get_prime() -> i64 {
            unsafe { MOD }
        }
        pub fn new<T: Into<i64>>(sig: T) -> Self {
            let sig: i64 = sig.into();
            if sig < 0 {
                Self {
                    x: sig % Self::get_prime() + Self::get_prime(),
                }
            } else {
                Self {
                    x: sig % Self::get_prime(),
                }
            }
        }
        pub fn get(&self) -> i64 {
            self.x
        }
        fn inverse(&self) -> Self {
            // [Fermat's little theorem]
            // if p is prime, for any integer a, a^(p-1) = 1.
            let mut ret = Self { x: 1 };
            let mut mul: Self = Self { x: self.get() };
            let mut p = Self::get_prime() - 2;
            while p > 0 {
                if p & 1 != 0 {
                    ret *= mul;
                }
                p >>= 1;
                mul *= mul;
            }
            ret
        }
    }
    impl AddAssign<Self> for ModInt {
        fn add_assign(&mut self, rhs: Self) {
            *self = ModInt::new(self.x + rhs.get());
        }
    }
    impl AddAssign<i64> for ModInt {
        fn add_assign(&mut self, rhs: i64) {
            *self = ModInt::new(self.x + rhs);
        }
    }
    impl Add<ModInt> for ModInt {
        type Output = ModInt;
        fn add(self, rhs: Self) -> Self::Output {
            ModInt::new(self.x + rhs.get())
        }
    }
    impl Add<i64> for ModInt {
        type Output = ModInt;
        fn add(self, rhs: i64) -> Self::Output {
            ModInt::new(self.x + rhs)
        }
    }
    impl Add<ModInt> for i64 {
        type Output = ModInt;
        fn add(self, rhs: ModInt) -> Self::Output {
            ModInt::new(self + rhs.get())
        }
    }
    impl SubAssign<Self> for ModInt {
        fn sub_assign(&mut self, rhs: Self) {
            *self = ModInt::new(self.x - rhs.get());
        }
    }
    impl SubAssign<i64> for ModInt {
        fn sub_assign(&mut self, rhs: i64) {
            *self = ModInt::new(self.x - rhs);
        }
    }
    impl Sub<ModInt> for ModInt {
        type Output = ModInt;
        fn sub(self, rhs: Self) -> Self::Output {
            ModInt::new(self.x - rhs.get())
        }
    }
    impl Sub<i64> for ModInt {
        type Output = ModInt;
        fn sub(self, rhs: i64) -> Self::Output {
            ModInt::new(self.x - rhs)
        }
    }
    impl Sub<ModInt> for i64 {
        type Output = ModInt;
        fn sub(self, rhs: ModInt) -> Self::Output {
            ModInt::new(self - rhs.get())
        }
    }
    impl MulAssign<Self> for ModInt {
        fn mul_assign(&mut self, rhs: Self) {
            *self = ModInt::new(self.x * rhs.get());
        }
    }
    impl MulAssign<i64> for ModInt {
        fn mul_assign(&mut self, rhs: i64) {
            *self = ModInt::new(self.x * rhs);
        }
    }
    impl Mul<ModInt> for ModInt {
        type Output = ModInt;
        fn mul(self, rhs: Self) -> Self::Output {
            ModInt::new(self.x * rhs.get())
        }
    }
    impl Mul<i64> for ModInt {
        type Output = ModInt;
        fn mul(self, rhs: i64) -> Self::Output {
            ModInt::new(self.x * rhs)
        }
    }
    impl Mul<ModInt> for i64 {
        type Output = ModInt;
        fn mul(self, rhs: ModInt) -> Self::Output {
            ModInt::new(self * rhs.get())
        }
    }
    impl DivAssign<Self> for ModInt {
        fn div_assign(&mut self, rhs: Self) {
            *self = *self / rhs;
        }
    }
    impl DivAssign<i64> for ModInt {
        fn div_assign(&mut self, rhs: i64) {
            *self = *self / ModInt::new(rhs);
        }
    }
    impl Div<ModInt> for ModInt {
        type Output = ModInt;
        fn div(self, rhs: Self) -> Self::Output {
            #[allow(clippy::suspicious_arithmetic_impl)]
            ModInt::new(self.x * rhs.inverse().get())
        }
    }
    impl Div<i64> for ModInt {
        type Output = ModInt;
        fn div(self, rhs: i64) -> Self::Output {
            #[allow(clippy::suspicious_arithmetic_impl)]
            ModInt::new(self.x * ModInt::new(rhs).inverse().get())
        }
    }
    impl Div<ModInt> for i64 {
        type Output = ModInt;
        fn div(self, rhs: ModInt) -> Self::Output {
            #[allow(clippy::suspicious_arithmetic_impl)]
            ModInt::new(self * rhs.inverse().get())
        }
    }
    impl From<usize> for ModInt {
        fn from(x: usize) -> Self {
            ModInt::new(x as i64)
        }
    }
    impl PartialEq<ModInt> for ModInt {
        fn eq(&self, other: &ModInt) -> bool {
            self.get() == other.get()
        }
    }
    #[allow(clippy::from_over_into)]
    impl Into<usize> for ModInt {
        fn into(self) -> usize {
            self.x as usize
        }
    }
    impl fmt::Display for ModInt {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.x)
        }
    }
    impl fmt::Debug for ModInt {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.x)
        }
    }
}
use modint::ModInt as mint;

pub trait IntegerOperation {
    fn into_primes(self) -> BTreeMap<Self, usize>
    where
        Self: Sized;
    fn into_divisors(self) -> Vec<Self>
    where
        Self: Sized;
    fn squared_length(&self, rhs: Self) -> Self;
}
impl<
        T: Copy
            + Ord
            + AddAssign
            + DivAssign
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Rem<Output = T>,
    > IntegerOperation for T
{
    fn into_primes(self) -> BTreeMap<T, usize> // O(N^0.5 x logN)
    {
        #[allow(clippy::eq_op)]
        let zero = self - self;
        if self == zero {
            panic!("Zero has no divisors.");
        }
        #[allow(clippy::eq_op)]
        let one = self / self;
        let two = one + one;
        let mut n = self;
        let mut ans = BTreeMap::<T, usize>::new();
        {
            let mut i = two;
            while i * i <= n {
                while n % i == zero {
                    let v = ans.entry(i).or_insert(0_usize);
                    *v += 1_usize;
                    n /= i;
                }
                i += one;
            }
        }
        if n != one {
            let v = ans.entry(n).or_insert(0);
            *v += 1_usize;
        }
        ans
    }
    fn into_divisors(self) -> Vec<T> // O(N^0.5)
    {
        #[allow(clippy::eq_op)]
        let zero = self - self;
        if self == zero {
            panic!("Zero has no primes.");
        }
        #[allow(clippy::eq_op)]
        let one = self / self;
        let n = self;
        let mut ret: Vec<T> = Vec::new();
        {
            let mut i = one;
            while i * i <= n {
                if n % i == zero {
                    ret.push(i);
                    if i * i != n {
                        ret.push(n / i);
                    }
                }
                i += one;
            }
        }
        ret.sort();
        ret
    }
    fn squared_length(&self, rhs: Self) -> Self {
        *self * *self + rhs * rhs
    }
}

pub trait CoordinateCompress<T> {
    fn compress_encoder(&self) -> BTreeMap<T, usize>;
    fn compress_decoder(&self) -> Vec<T>;
    fn compress(self) -> Vec<usize>;
}
impl<T: Copy + Ord> CoordinateCompress<T> for Vec<T> {
    fn compress_encoder(&self) -> BTreeMap<T, usize> {
        let mut dict = BTreeMap::<T, usize>::new();
        for x in self.iter() {
            let _ = dict.entry(*x).or_insert(0); //keys.insert(*x);
        }
        for (i, kv) in dict.iter_mut().enumerate() {
            *kv.1 = i;
        }
        dict
    }
    fn compress_decoder(&self) -> Vec<T> {
        let mut keys = BTreeSet::<T>::new();
        for &x in self.iter() {
            keys.insert(x);
        }
        keys.into_iter().collect::<Vec<T>>()
    }
    fn compress(self) -> Vec<usize> {
        let dict = self.compress_encoder();
        self.into_iter().map(|x| dict[&x]).collect::<Vec<usize>>()
    }
}

mod xor_shift_64 {
    pub struct XorShift64(u64);
    impl XorShift64 {
        pub fn new() -> Self {
            XorShift64(88172645463325252_u64)
        }
        pub fn next_f64(&mut self) -> f64 {
            self.0 = self.0 ^ (self.0 << 7);
            self.0 = self.0 ^ (self.0 >> 9);
            self.0 as f64 * 5.421_010_862_427_522e-20
        }
        pub fn next_u64(&mut self) -> u64 {
            self.0 = self.0 ^ (self.0 << 7);
            self.0 = self.0 ^ (self.0 >> 9);
            self.0
        }
        pub fn next_usize(&mut self) -> usize {
            self.next_u64() as usize
        }
    }
}
use xor_shift_64::XorShift64;

mod rooted_tree {
    use std::mem::swap;

    use crate::union_find::UnionFind;
    pub struct RootedTree {
        n: usize,
        doubling_bit_width: usize,
        root: usize,
        rise_tbl: Vec<Vec<Option<usize>>>,
        dist: Vec<Option<i64>>,
        step: Vec<Option<usize>>,
        pub graph: Vec<Vec<(i64, usize)>>,
        edge_cnt: usize,
        uf: UnionFind,
    }
    impl RootedTree {
        pub fn new(n: usize, root: usize) -> RootedTree {
            let mut doubling_bit_width = 1;
            while (1 << doubling_bit_width) < n {
                doubling_bit_width += 1;
            }
            RootedTree {
                n,
                doubling_bit_width,
                root,
                rise_tbl: vec![vec![None; n]; doubling_bit_width],
                dist: vec![None; n],
                step: vec![None; n],
                graph: vec![vec![]; n],
                edge_cnt: 0,
                uf: UnionFind::new(n),
            }
        }
        pub fn unite(&mut self, a: usize, b: usize) {
            self.unite_with_distance(a, b, 1);
        }
        pub fn unite_with_distance(&mut self, a: usize, b: usize, delta: i64) {
            self.graph[a].push((delta, b));
            self.graph[b].push((delta, a));
            self.edge_cnt += 1;
            self.uf.unite(a, b);
            if self.edge_cnt >= self.n - 1 {
                if self.uf.group_num() != 1 {
                    panic!("nodes are NOT connected into one union.")
                }
                self.analyze(self.root);
            }
        }
        pub fn stepback(&self, from: usize, step: usize) -> usize {
            let mut v = from;
            for d in (0..self.doubling_bit_width - 1).rev() {
                if ((step >> d) & 1) != 0 {
                    v = self.rise_tbl[d][v].unwrap();
                }
            }
            v
        }
        fn dfs(
            v: usize,
            pre: Option<usize>,
            graph: &Vec<Vec<(i64, usize)>>,
            dist: &mut Vec<Option<i64>>,
            step: &mut Vec<Option<usize>>,
            rise_tbl: &mut Vec<Vec<Option<usize>>>,
        ) {
            for (delta, nv) in graph[v].iter() {
                if let Some(pre) = pre {
                    if *nv == pre {
                        continue;
                    }
                }
                if dist[*nv].is_none() {
                    dist[*nv] = Some(dist[v].unwrap() + *delta);
                    step[*nv] = Some(step[v].unwrap() + 1);
                    rise_tbl[0][*nv] = Some(v);
                    Self::dfs(*nv, Some(v), graph, dist, step, rise_tbl);
                }
            }
        }
        fn analyze(&mut self, root: usize) {
            self.dist[root] = Some(0);
            self.step[root] = Some(0);
            self.rise_tbl[0][root] = Some(root);
            Self::dfs(
                root,
                None,
                &self.graph,
                &mut self.dist,
                &mut self.step,
                &mut self.rise_tbl,
            );
            // doubling
            for d in (0..self.doubling_bit_width).skip(1) {
                for v in 0_usize..self.n {
                    self.rise_tbl[d][v] = self.rise_tbl[d - 1][self.rise_tbl[d - 1][v].unwrap()];
                }
            }
        }
        pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
            if self.step[a] > self.step[b] {
                swap(&mut a, &mut b);
            }
            assert!(self.step[a] <= self.step[b]);
            // bring up the deeper one to the same level of the shallower one.
            for d in (0..self.doubling_bit_width).rev() {
                let rise_v = self.rise_tbl[d][b].unwrap();
                if self.step[rise_v] >= self.step[a] {
                    b = rise_v;
                }
            }
            assert!(self.step[a] == self.step[b]);
            if a != b {
                // simultaneously rise to the previous level of LCA.
                for d in (0..self.doubling_bit_width).rev() {
                    if self.rise_tbl[d][a] != self.rise_tbl[d][b] {
                        a = self.rise_tbl[d][a].unwrap();
                        b = self.rise_tbl[d][b].unwrap();
                    }
                }
                // 1-step higher level is LCA.
                a = self.rise_tbl[0][a].unwrap();
                b = self.rise_tbl[0][b].unwrap();
            }
            assert!(a == b);
            a
        }
        pub fn distance(&self, a: usize, b: usize) -> i64 {
            let lca_v = self.lca(a, b);
            self.dist[a].unwrap() + self.dist[b].unwrap() - 2 * self.dist[lca_v].unwrap()
        }
    }
}
use rooted_tree::RootedTree;

mod btree_map_binary_search {
    use std::collections::BTreeMap;
    use std::ops::Bound::{Excluded, Included, Unbounded};
    pub trait BTreeMapBinarySearch<K, V> {
        fn greater_equal(&self, key: &K) -> Option<(&K, &V)>;
        fn greater_than(&self, key: &K) -> Option<(&K, &V)>;
        fn less_equal(&self, key: &K) -> Option<(&K, &V)>;
        fn less_than(&self, key: &K) -> Option<(&K, &V)>;
    }
    impl<K: Ord, V> BTreeMapBinarySearch<K, V> for BTreeMap<K, V> {
        fn greater_equal(&self, key: &K) -> Option<(&K, &V)> {
            self.range((Included(key), Unbounded)).next()
        }
        fn greater_than(&self, key: &K) -> Option<(&K, &V)> {
            self.range((Excluded(key), Unbounded)).next()
        }
        fn less_equal(&self, key: &K) -> Option<(&K, &V)> {
            self.range((Unbounded, Included(key))).next_back()
        }
        fn less_than(&self, key: &K) -> Option<(&K, &V)> {
            self.range((Unbounded, Excluded(key))).next_back()
        }
    }
}
use btree_map_binary_search::BTreeMapBinarySearch;

mod btree_set_binary_search {
    use std::collections::BTreeSet;
    use std::ops::Bound::{Excluded, Included, Unbounded};
    pub trait BTreeSetBinarySearch<T> {
        fn greater_equal(&self, key: &T) -> Option<&T>;
        fn greater_than(&self, key: &T) -> Option<&T>;
        fn less_equal(&self, key: &T) -> Option<&T>;
        fn less_than(&self, key: &T) -> Option<&T>;
    }
    impl<T: Ord> BTreeSetBinarySearch<T> for BTreeSet<T> {
        fn greater_equal(&self, key: &T) -> Option<&T> {
            self.range((Included(key), Unbounded)).next()
        }
        fn greater_than(&self, key: &T) -> Option<&T> {
            self.range((Excluded(key), Unbounded)).next()
        }
        fn less_equal(&self, key: &T) -> Option<&T> {
            self.range((Unbounded, Included(key))).next_back()
        }
        fn less_than(&self, key: &T) -> Option<&T> {
            self.range((Unbounded, Excluded(key))).next_back()
        }
    }
}
use btree_set_binary_search::BTreeSetBinarySearch;

mod sort_vec_binary_search {
    static mut VEC_IS_SORTED_ONCE: bool = false;
    #[allow(clippy::type_complexity)]
    fn sorted_binary_search<'a, 'b, T: PartialOrd>(
        vec: &'a Vec<T>,
        key: &'b T,
        earlier: fn(&T, &T) -> bool,
    ) -> (Option<(usize, &'a T)>, Option<(usize, &'a T)>) {
        unsafe {
            if !VEC_IS_SORTED_ONCE {
                for i in 1..vec.len() {
                    assert!(vec[i - 1] <= vec[i]);
                }
                VEC_IS_SORTED_ONCE = true;
            }
        }
        if vec.is_empty() {
            return (None, None);
        }

        if !earlier(&vec[0], key) {
            (None, Some((0, &vec[0])))
        } else if earlier(vec.last().unwrap(), key) {
            (Some((vec.len() - 1, &vec[vec.len() - 1])), None)
        } else {
            let mut l = 0;
            let mut r = vec.len() - 1;
            while r - l > 1 {
                let m = (l + r) / 2;
                if earlier(&vec[m], key) {
                    l = m;
                } else {
                    r = m;
                }
            }
            (Some((l, &vec[l])), Some((r, &vec[r])))
        }
    }
    pub trait SortVecBinarySearch<T> {
        #[allow(clippy::type_complexity)]
        fn greater_equal(&self, key: &T) -> Option<(usize, &T)>;
        fn greater_than(&self, key: &T) -> Option<(usize, &T)>;
        fn less_equal(&self, key: &T) -> Option<(usize, &T)>;
        fn less_than(&self, key: &T) -> Option<(usize, &T)>;
    }
    impl<T: Ord> SortVecBinarySearch<T> for Vec<T> {
        fn greater_equal<'a>(&self, key: &'a T) -> Option<(usize, &T)> {
            sorted_binary_search(self, key, |x: &T, y: &T| x < y).1
        }
        fn greater_than<'a>(&self, key: &'a T) -> Option<(usize, &T)> {
            sorted_binary_search(self, key, |x: &T, y: &T| x <= y).1
        }
        fn less_equal<'a>(&self, key: &'a T) -> Option<(usize, &T)> {
            sorted_binary_search(self, key, |x: &T, y: &T| x <= y).0
        }
        fn less_than<'a>(&self, key: &'a T) -> Option<(usize, &T)> {
            sorted_binary_search(self, key, |x: &T, y: &T| x < y).0
        }
    }
}
use sort_vec_binary_search::SortVecBinarySearch;

mod btree_multi_set {
    use crate::btree_map_binary_search::BTreeMapBinarySearch;
    use std::collections::BTreeMap;
    #[derive(Debug, Clone)]
    pub struct BTreeMultiSet<T> {
        mp: BTreeMap<T, usize>,
        cnt_sum: usize,
    }
    impl<T: Copy + Ord> BTreeMultiSet<T> {
        pub fn new() -> Self {
            BTreeMultiSet {
                mp: BTreeMap::<T, usize>::new(),
                cnt_sum: 0,
            }
        }
        pub fn is_empty(&self) -> bool {
            self.mp.is_empty()
        }
        pub fn len(&self) -> usize {
            self.cnt_sum
        }
        pub fn insert(&mut self, key: T) {
            let cnt = self.mp.entry(key).or_insert(0);
            *cnt += 1;
            self.cnt_sum += 1;
        }
        pub fn remove(&mut self, key: &T) -> bool {
            if let Some(cnt) = self.mp.get_mut(key) {
                *cnt -= 1;
                if *cnt == 0 {
                    self.mp.remove(key);
                }
                self.cnt_sum -= 1;
                true
            } else {
                false
            }
        }
        pub fn contains(&self, key: &T) -> bool {
            self.mp.contains_key(key)
        }
        pub fn remove_all(&mut self, key: &T) -> bool {
            if let Some(cnt) = self.mp.remove(key) {
                self.cnt_sum -= cnt;
                true
            } else {
                false
            }
        }
        pub fn first(&self) -> Option<&T> {
            if let Some((key, _cnt)) = self.mp.iter().next() {
                Some(key)
            } else {
                None
            }
        }
        pub fn pop_first(&mut self) -> Option<T> {
            if let Some(&key) = self.first() {
                self.remove(&key);
                Some(key)
            } else {
                None
            }
        }
        pub fn last(&self) -> Option<&T> {
            if let Some((key, _cnt)) = self.mp.iter().next_back() {
                Some(key)
            } else {
                None
            }
        }
        pub fn pop_last(&mut self) -> Option<T> {
            if let Some(&key) = self.last() {
                self.remove(&key);
                Some(key)
            } else {
                None
            }
        }
        pub fn clear(&mut self) {
            self.mp.clear();
            self.cnt_sum = 0;
        }
        pub fn greater_equal(&self, key: &T) -> Option<&T> {
            if let Some((key, _cnt)) = self.mp.greater_equal(key) {
                Some(key)
            } else {
                None
            }
        }
        pub fn greater_than(&self, key: &T) -> Option<&T> {
            if let Some((key, _cnt)) = self.mp.greater_than(key) {
                Some(key)
            } else {
                None
            }
        }
        pub fn less_equal(&self, key: &T) -> Option<&T> {
            if let Some((key, _cnt)) = self.mp.less_equal(key) {
                Some(key)
            } else {
                None
            }
        }
        pub fn less_than(&self, key: &T) -> Option<&T> {
            if let Some((key, _cnt)) = self.mp.less_than(key) {
                Some(key)
            } else {
                None
            }
        }
    }
}
use btree_multi_set::BTreeMultiSet;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Line2d(i64, i64, i64);
impl Line2d {
    // identify line from 2 differemt point
    fn new(y0: i64, x0: i64, y1: i64, x1: i64) -> Line2d {
        let mut b = y1 - y0;
        let mut a = x1 - x0;
        let mut c = x1 * y0 - x0 * y1;
        let r = gcd(a.abs(), gcd(b.abs(), c.abs()));
        a /= r;
        b /= r;
        c /= r;
        if (a == 0) && (b < 0) {
            a = -a;
            b = -b;
            c = -c;
        }
        if a < 0 {
            a = -a;
            b = -b;
            c = -c;
        }
        Line2d(a, b, c)
    }
}

mod strongly_connected_component {
    pub struct StronglyConnectedComponent {
        n: usize,
        pub graph: Vec<Vec<usize>>,
        bwd_graph: Vec<Vec<usize>>,
    }
    impl StronglyConnectedComponent {
        pub fn new(n: usize) -> StronglyConnectedComponent {
            StronglyConnectedComponent {
                n,
                graph: vec![vec![]; n],
                bwd_graph: vec![vec![]; n],
            }
        }
        pub fn add(&mut self, from: usize, into: usize) {
            self.graph[from].push(into);
            self.bwd_graph[into].push(from);
        }
        pub fn decompose(&mut self) -> Vec<Vec<usize>> {
            let mut scc = Vec::<Vec<usize>>::new();
            let mut fwd_seen = vec![false; self.n];
            let mut order = Vec::<usize>::new();
            for i in 0..self.n {
                if !fwd_seen[i] {
                    StronglyConnectedComponent::fwd_dfs(
                        &self.graph,
                        i,
                        None,
                        &mut fwd_seen,
                        &mut order,
                    );
                }
            }
            order.reverse();
            let mut bwd_seen = vec![false; self.n];
            for i_ in &order {
                let i = *i_;
                if !bwd_seen[i] {
                    let mut grp = Vec::<usize>::new();
                    StronglyConnectedComponent::bwd_dfs(
                        &self.bwd_graph,
                        i,
                        None,
                        &mut bwd_seen,
                        &mut grp,
                    );
                    grp.reverse();
                    scc.push(grp);
                }
            }
            scc
        }
        fn bwd_dfs(
            graph: &Vec<Vec<usize>>,
            v: usize,
            pre: Option<usize>,
            seen: &mut Vec<bool>,
            grp: &mut Vec<usize>,
        ) {
            seen[v] = true;
            for nv_ in &graph[v] {
                let nv = *nv_;
                if let Some(pre_v) = pre {
                    if nv == pre_v {
                        continue;
                    }
                }
                if !seen[nv] {
                    StronglyConnectedComponent::bwd_dfs(graph, nv, Some(v), seen, grp);
                }
            }
            grp.push(v);
        }
        fn fwd_dfs(
            graph: &Vec<Vec<usize>>,
            v: usize,
            pre: Option<usize>,
            seen: &mut Vec<bool>,
            order: &mut Vec<usize>,
        ) {
            seen[v] = true;
            for nv_ in &graph[v] {
                let nv = *nv_;
                if let Some(pre_v) = pre {
                    if nv == pre_v {
                        continue;
                    }
                }
                if !seen[nv] {
                    StronglyConnectedComponent::fwd_dfs(graph, nv, Some(v), seen, order);
                }
            }
            order.push(v);
        }
    }
}
use strongly_connected_component::StronglyConnectedComponent as Scc;

mod pair {
    use std::ops::{Add, AddAssign, Sub, SubAssign};
    #[derive(Debug, Clone, Copy)]
    pub struct Pair<X, Y> {
        pub x: X,
        pub y: Y,
    }
    impl<X: AddAssign, Y: AddAssign> AddAssign for Pair<X, Y> {
        fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }
    impl<X: Add<Output = X>, Y: Add<Output = Y>> Add for Pair<X, Y> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    impl<X: SubAssign, Y: SubAssign> SubAssign for Pair<X, Y> {
        fn sub_assign(&mut self, rhs: Self) {
            self.x -= rhs.x;
            self.y -= rhs.y;
        }
    }
    impl<X: Sub<Output = X>, Y: Sub<Output = Y>> Sub for Pair<X, Y> {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }
}
use pair::Pair;

mod usize_move_to {
    pub trait MoveTo<T> {
        fn move_to(self, delta: T, lim_lo: usize, lim_hi: usize) -> Option<usize>;
    }
    impl<T: Copy + Into<i64>> MoveTo<T> for usize {
        fn move_to(self, delta: T, lim_lo: usize, lim_hi: usize) -> Option<usize> {
            let delta: i64 = delta.into();
            let added: i64 = self as i64 + delta;
            let lim_lo: i64 = lim_lo as i64;
            let lim_hi: i64 = lim_hi as i64;
            if (lim_lo <= added) && (added <= lim_hi) {
                Some(added as usize)
            } else {
                None
            }
        }
    }
}
use usize_move_to::MoveTo;

fn exit_by<T: std::fmt::Display>(msg: T) {
    println!("{}", msg);
    std::process::exit(0);
}

pub trait Permutation<T> {
    fn next_permutation(&self) -> Option<Vec<T>>;
    fn prev_permutation(&self) -> Option<Vec<T>>;
}
impl<T: Copy + Ord> Permutation<T> for Vec<T> {
    fn next_permutation(&self) -> Option<Vec<T>> {
        let n = self.len();
        if n == 0 {
            return None;
        }
        let mut seen = BTreeMultiSet::<T>::new();
        seen.insert(*self.last().unwrap());
        for i in (0..n).into_iter().rev().skip(1) {
            seen.insert(self[i]);
            if self[i] < self[i + 1] {
                let mut p = vec![];
                for &lv in self.iter().take(i) {
                    p.push(lv);
                }
                let &rv = seen.greater_than(&self[i]).unwrap();
                p.push(rv);
                seen.remove(&rv);
                while let Some(rv) = seen.pop_first() {
                    p.push(rv);
                }
                return Some(p);
            }
        }
        None
    }
    fn prev_permutation(&self) -> Option<Vec<T>> {
        let n = self.len();
        if n == 0 {
            return None;
        }
        let mut seen = BTreeMultiSet::<T>::new();
        seen.insert(*self.last().unwrap());
        for i in (0..n).into_iter().rev().skip(1) {
            seen.insert(self[i]);
            if self[i] > self[i + 1] {
                let mut p = vec![];
                for &lv in self.iter().take(i) {
                    p.push(lv);
                }
                let &rv = seen.less_than(&self[i]).unwrap();
                p.push(rv);
                seen.remove(&rv);
                while let Some(rv) = seen.pop_last() {
                    p.push(rv);
                }
                return Some(p);
            }
        }
        None
    }
}
pub struct PermutationIterator<T> {
    v: Vec<T>,
    is_finished: bool,
}
impl<T: Copy + Ord + Clone> PermutationIterator<T> {
    pub fn new(mut v: Vec<T>) -> PermutationIterator<T> {
        v.sort();
        PermutationIterator {
            v,
            is_finished: false,
        }
    }
}
impl<T: Copy + Ord + Clone> Iterator for PermutationIterator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished {
            // next perm doesn't exist.
            None
        } else if let Some(nxt) = self.v.next_permutation() {
            // return self state, and update self for future use.
            let ret = Some(self.v.clone());
            self.v = nxt;
            ret
        } else {
            // this time is the last.
            self.is_finished = true;
            Some(self.v.clone())
        }
    }
}

pub trait IntoPermutations<T: Copy + Ord + Clone> {
    fn into_permutations(self) -> PermutationIterator<T>;
}
// implement for ones that has IntoIterator.
impl<T: Copy + Ord + Clone, I: IntoIterator<Item = T>> IntoPermutations<T> for I {
    fn into_permutations(self) -> PermutationIterator<T> {
        PermutationIterator::new(self.into_iter().collect())
    }
}

mod procon_reader {
    use std::collections::VecDeque;
    use std::fmt::Debug;
    use std::str::FromStr;
    pub struct ProConReader {
        que: VecDeque<String>,
        stdin: std::io::Stdin,
    }
    impl ProConReader {
        pub fn new() -> Self {
            Self {
                que: VecDeque::<String>::new(),
                stdin: std::io::stdin(),
            }
        }
        pub fn read<T: FromStr>(&mut self) -> T
        where
            <T as FromStr>::Err: Debug,
        {
            if self.que.is_empty() {
                self.enqueue_blocks();
            }
            self.que.pop_front().unwrap().parse().unwrap()
        }
        pub fn read_vec<T: FromStr>(&mut self, n: usize) -> Vec<T>
        where
            <T as FromStr>::Err: Debug,
        {
            (0..n)
                .into_iter()
                .map(|_| self.read::<T>())
                .collect::<Vec<T>>()
        }
        fn enqueue_blocks(&mut self) {
            let mut buf = String::new();
            let _ = self.stdin.read_line(&mut buf);
            for read_val in buf.split_whitespace() {
                self.que.push_back(read_val.to_string());
            }
        }
    }
}
use procon_reader::ProConReader;

mod add_header {
    pub trait AddHeader<T> {
        fn add_header(&mut self, add_val: T);
    }
    impl<T> AddHeader<T> for Vec<T>
    where
        Vec<T>: Clone,
    {
        fn add_header(&mut self, add_val: T) {
            let cpy = self.clone();
            self.clear();
            self.push(add_val);
            for cpy_val in cpy {
                self.push(cpy_val);
            }
        }
    }
}
use add_header::AddHeader;

mod auto_sort_vec {
    use crate::segment_tree::SegmentTree;
    pub struct AutoSortVec {
        max_val: usize,
        st: SegmentTree<usize>,
    }
    impl AutoSortVec {
        pub fn new(max_val: usize) -> AutoSortVec {
            AutoSortVec {
                max_val,
                st: SegmentTree::<usize>::new(max_val as usize + 1, |x, y| x + y, 0),
            }
        }
        pub fn len(&self) -> usize {
            self.st.query(0, self.max_val as usize)
        }
        pub fn push(&mut self, val: usize) {
            self.st.add(val, 1);
        }
        pub fn remove_value(&mut self, val: usize) {
            self.st.sub(val, 1);
        }
        pub fn value_to_index(&self, val: usize) -> usize {
            if val == 0 {
                0
            } else {
                self.st.query(0, val as usize - 1)
            }
        }
        pub fn at(&self, idx: usize) -> usize {
            let idx1 = idx + 1;
            if self.st.get(0) >= idx1 {
                0
            } else if self.st.query(0, self.max_val as usize - 1) < idx1 {
                self.max_val
            } else {
                let mut l = 0;
                let mut r = self.max_val;
                while r - l > 1 {
                    let m = (r + l) / 2;
                    let sm = self.st.query(0, m as usize);
                    if sm < idx1 {
                        l = m;
                    } else {
                        r = m;
                    }
                }
                r
            }
        }
    }
}
use auto_sort_vec::AutoSortVec;

/*************************************************************************************
*************************************************************************************/
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
const NMIN: usize = 4;
const NMAX: usize = 100;
const MMIN: usize = 10;
const MMAX: usize = 100;
const EPSUNIT: f64 = 0.01_f64;
const EPSMIN: usize = 0;
const EPSMAX: usize = 40;
const INF: usize = 1_usize << 60;
struct Student {
    ref_features: Vec<Vec<f64>>,
}
impl Student {
    pub fn new(v: &[Vec<char>], n: usize) -> Student {
        let mut features = vec![];
        for s in v.iter() {
            let mut g = vec![vec![]; n];
            Student::convert(s, &mut g, n);
            features.push(Student::calc_feature(&g, n));
        }
        Student {
            ref_features: features,
        }
    }
    fn calc_feature(g: &[Vec<usize>], n: usize) -> Vec<f64> {
        fn get_end(g: &[Vec<usize>], n: usize, ini: usize) -> usize {
            let mut dist = vec![INF; n];
            dist[ini] = 0;
            let mut que = VecDeque::<usize>::new();
            que.push_back(ini);
            let mut max_dist_i = ini;
            let mut max_dist = 0_usize;
            while let Some(v) = que.pop_front() {
                for &nv in &g[v] {
                    let nd = dist[v] + 1;
                    if dist[nv].chmin(nd) {
                        if max_dist.chmax(dist[nv]) {
                            max_dist_i = nv;
                        }
                        que.push_back(nv);
                    }
                }
            }
            max_dist_i
        }
        fn get_centers(g: &[Vec<usize>], n: usize, e0: usize, e1: usize) -> Vec<usize> {
            fn get_distance(g: &[Vec<usize>], n: usize, ini: usize) -> Vec<usize> {
                let mut dist = vec![INF; n];
                dist[ini] = 0;
                let mut que = VecDeque::<usize>::new();
                que.push_back(ini);
                while let Some(v) = que.pop_front() {
                    let nd = dist[v] + 1;
                    for &nv in g[v].iter() {
                        if dist[nv].chmin(nd) {
                            que.push_back(nv);
                        }
                    }
                }
                dist
            }
            let dist0 = get_distance(g, n, e0);
            let dist1 = get_distance(g, n, e1);
            let mut dif_min = INF;
            for (&d0, &d1) in dist0.iter().zip(dist1.iter()) {
                let dif = (d0 as i64 - d1 as i64).abs() as usize;
                dif_min.chmin(dif);
            }
            let mut centers = vec![];
            for (i, (d0, d1)) in dist0.into_iter().zip(dist1.into_iter()).enumerate() {
                let dif = (d0 as i64 - d1 as i64).abs() as usize;
                if dif_min == dif {
                    centers.push(i);
                }
            }
            centers
        }
        fn vote(g: &[Vec<usize>], n: usize, ini: usize, pole: &mut [(usize, usize)]) {
            let mut dist = vec![INF; n];
            dist[ini] = 0;
            let mut que = VecDeque::<usize>::new();
            que.push_back(ini);
            pole[ini] = (pole[ini].0, pole[ini].1 + 1);
            while let Some(v) = que.pop_front() {
                let nd = dist[v] + 1;
                for &nv in g[v].iter() {
                    if dist[nv].chmin(nd) {
                        que.push_back(nv);
                        pole[nv] = (pole[nv].0 + dist[nv], pole[nv].1 + 1);
                    }
                }
            }
        }

        let mut uf = UnionFind::new(n);
        for (i, to) in g.iter().enumerate() {
            for &j in to {
                uf.unite(i, j);
            }
        }
        let mut roots = BTreeMap::<i64, usize>::new();
        for i in 0..n {
            let r = roots
                .entry(-(uf.group_size(i) as i64))
                .or_insert(uf.root(i));
            *r = uf.root(i);
        }

        let mut pole = vec![(0_usize, 0_usize); n];
        for (_ngs, r) in roots.iter() {
            let end0 = get_end(g, n, *r);
            let end1 = get_end(g, n, end0);

            let centers = get_centers(g, n, end0, end1);
            for center in centers {
                vote(g, n, center, &mut pole);
            }
        }

        let mut f = pole
            .into_iter()
            .map(|(e, d)| {
                if d == 0 {
                    n as f64
                } else {
                    e as f64 / d as f64
                }
            })
            .collect::<Vec<f64>>();
        loop {
            let mut swp = false;
            for i in 1..n {
                if f[i - 1] > f[i] {
                    f.swap(i, i - 1);
                    swp = true;
                }
            }
            if !swp {
                break;
            }
        }
        f
    }
    fn convert(s: &[char], g: &mut [Vec<usize>], n: usize) {
        let mut idx = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                if s[idx] == '1' {
                    g[i].push(j);
                    g[j].push(i);
                }
                idx += 1;
            }
        }
    }
    fn distance(f1: &[f64], f2: &[f64]) -> f64 {
        f1.iter()
            .zip(f2.iter())
            .map(|(v1, v2)| (v1 - v2) * (v1 - v2))
            .sum::<f64>()//reduce(|x, y| x + y)
    }
    pub fn select(&self, s: &[char], n: usize, m: usize) -> usize {
        let mut g = vec![vec![]; n];
        Student::convert(s, &mut g, n);
        let mut sel = 0;
        let mut min_dist = INF as f64;
        let f = Student::calc_feature(&g, n);
        for (fi, ref_feature) in self.ref_features.iter().enumerate().take(m) {
            if min_dist.chmin(Student::distance(&f, ref_feature)) {
                sel = fi;
            }
        }
        sel
    }
}
struct Teacher {
    vs: Vec<Vec<char>>,
    n: usize,
    rand: XorShift64,
}
impl Teacher {
    pub fn new() -> Teacher {
        Teacher {
            vs: vec![vec![]; MMAX],
            n: 0,
            rand: XorShift64::new(),
        }
    }
    pub fn regenerate(&mut self) {
        self.n = NMAX; //self.rand.next_usize() % (NMAX - NMIN) + NMIN;
        for v in self.vs.iter_mut() {
            v.clear();
            let mut uf = UnionFind::new(self.n);
            let n2 = self.n * self.n;
            let mut remain = AutoSortVec::new(n2);
            for i in 0..n2 {
                remain.push(i);
            }
            let mut g = vec![HashSet::<usize>::new(); self.n];
            while uf.group_num() != 1 {
                let idx = self.rand.next_usize() % remain.len();
                remain.remove_value(remain.at(idx));
                let (i, j) = (idx / self.n, idx % self.n);
                if i >= j {
                    continue;
                }
                let (i, j) = (min(i, j), max(i, j));
                uf.unite(i, j);
                g[i].insert(j);
                g[j].insert(i);
            }
            for (i, to) in g.into_iter().enumerate() {
                for j in (i + 1)..self.n {
                    if to.contains(&j) {
                        v.push('1');
                    } else {
                        v.push('0');
                    }
                }
            }
            debug_assert!(v.len() == (self.n * (self.n - 1)) / 2);
            debug_assert!(
                v.iter().map(|&c| (c as u8 - b'0') as usize).sum::<usize>() >= self.n - 1
            );
        }
    }
    fn modify(&mut self, s: usize, eps: f64) -> Vec<char> {
        let mut ret: Vec<char> = vec![];
        let org = &self.vs[s];
        for &c in org {
            if self.rand.next_f64() < eps {
                if c == '0' {
                    ret.push('1');
                } else {
                    ret.push('0');
                }
            } else {
                ret.push(c);
            }
        }
        ret
    }
    pub fn trial(&mut self) -> f64 {
        let student = Student::new(&self.vs, self.n);
        let mut cnt = 0;
        let mut corr = 0;
        for m in (MMIN..=MMAX).step_by(1) {
            for e in (EPSMIN..=EPSMAX).rev().take(1) {
                let eps = e as f64 * EPSUNIT;
                for s in 0..m {
                    for _t in (0..100).take(1) {
                        let h = self.modify(s, eps);
                        let t = student.select(&h, self.n, m);
                        if s == t {
                            corr += 1;
                        }
                        cnt += 1;
                    }
                }
            }
        }
        corr as f64 / cnt as f64
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut teacher = Teacher::new();
    let mut best_rate = -1_f64;
    loop {
        teacher.regenerate();
        let rate = teacher.trial();
        println!("rate = {}", rate);
        if best_rate.chmax(rate) {
            println!("updated");
            let mut file = File::create("best_table.txt")?;
            for x in teacher.vs.iter() {
                for c in x {
                    write!(file, "{}", c)?;
                }
                writeln!(file)?;
            }
        }
    }
    Ok(())
}
