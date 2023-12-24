mod leetcode;
use leetcode::no1954;

#[test]
fn test(){
    let ans = no1954::minimum_perimeter(1000000000);
    assert!(ans == 5040);
}
fn main() {
    let ans = no1954::minimum_perimeter(1);
    println!("{0}", ans);
    let ans = no1954::minimum_perimeter(13);
    println!("{0}", ans);
    let ans = no1954::minimum_perimeter(1000);
    println!("{0}", ans);
    let ans = no1954::minimum_perimeter(1000000000);
    println!("{0}", ans);
}
