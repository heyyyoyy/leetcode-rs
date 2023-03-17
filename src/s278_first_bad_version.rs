// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)


struct Solution;

impl Solution {

    // mock func for compile
    #[allow(non_snake_case)]
    fn isBadVersion(&self, _version:i32)-> bool {
        true
    }

    pub fn first_bad_version_linear(&self, n: i32) -> i32 {
		for i in 1..n {
            if self.isBadVersion(i) {
                return i;
            }
        }
        n
    }

    pub fn first_bad_version_binary_search(&self, n: i32) -> i32 {
		let mut start = 1;
        let mut end = n;
        while start <= end {
            let mid = start + (end - start) / 2;
            let is_bad = self.isBadVersion(mid);
            if is_bad {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
        end + 1
    }
}
