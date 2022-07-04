macro_rules! lc {
    [$v:ident, $ans:expr; $i:ident <- $range:expr] => { for $i in $range {$v.push($ans)} };
    [$v:ident, $ans:expr; $i:ident <- $range:expr, $($tail:tt)*] => {
        {
            for $i in $range {
                lc![$v, $ans; $($tail)*] 
            } 
        }
    };
    [$ans: expr; $($i:ident <- $range:expr),+] => {
        {
            let mut v = Vec::new();
            lc![v, $ans; $($i <- $range),+];
            v
        }
    };
    [$v:ident, $ans:expr; $i:ident <- $range:expr; $($cond:expr),+] => {
        {
            for $i in $range {
                let mut val = true;
                $(val = val && $cond;)+
                if val { $v.push($ans) };
            }
        }
    };
    [$v:ident, $ans:expr; $i:ident <- $range:expr, $($train:tt)*; $($cond:expr),+] => {
        {
            for $i in $range {
                lc![$v, $ans; $($trail)*; $($cond:expr),+];
            }
        }
    };
    [$ans:expr; $($i:ident <- $range:expr),+;$($cond:expr),+] => {
        {
            let mut v = Vec::new();
            lc![v, $ans; $($i <- $range),+;$($cond),+];
            v
        }
    }
}

pub(crate) use lc;
// fn main() {
//     let v = lc![(x, y, z) ; x <- 1..100, y <- 1..100, z <- 1..100; x*x + y*y == z*z, x < y];
//     for trip in v.iter() { println!("{trip:?}") };
// }
