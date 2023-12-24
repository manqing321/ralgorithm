pub fn minimum_perimeter(needed_apples: i64) -> i64 {
    let mut beg = 0;
    let mut end = 20_0000;
    while beg < end {
        let mid = beg + ((end - beg) >> 1);
        let apples = mid * (mid + 1) * (2 * mid + 1) * 2;
        if apples == needed_apples {
            return mid * 8;
        }
        if apples < needed_apples {
            let rear = (mid + 1) * (mid + 2) * (2 * mid + 3) * 2;
            if rear >= needed_apples {
                return (mid + 1) * 8;
            }
            beg = mid + 1;
        } else {
            let pre = (mid - 1) * mid * (2 * mid - 1) * 2;
            if pre == needed_apples {
                return (mid - 1) * 8;
            } else if pre < needed_apples {
                return mid * 8;
            }
            end = mid - 1;
        }
    }
    return end * 8;
}
