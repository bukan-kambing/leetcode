use std::mem::replace;

// trait Solution {}
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        // check for 0
        if num1.eq("0") || num2.eq("0") {
            return "0".to_string();
        }

        // length
        let len1 = num1.len();
        let len2 = num2.len();

        //reversed string
        let no1: String = Solution::reverse_string(num1);
        let no2: String = Solution::reverse_string(num2);

        // create vector with size (m+n)
        let mut result: Vec<u32> = vec![0; len1 + len2];

        for i in 0..len1 {
            for j in 0..len2 {
                let n1 = no1.chars().nth(i).unwrap().to_digit(10).unwrap();
                let n2 = no2.chars().nth(j).unwrap().to_digit(10).unwrap();
                let digit = n1 * n2;

                let sum = result[i + j] + digit;
                let carry = result[i + j + 1] + (sum / 10);
                let digit = sum % 10;

                replace(&mut result[i + j + 1], carry);
                replace(&mut result[i + j], digit);

                // debugs lots of debugs
                // println!("{} {}", i, no1.as_bytes()[i] - "0".as_bytes()[0]);
                // println!("{} {}", i, (no1.as_bytes()[i] - "0".as_bytes()[0]) * 2);
                // println!("{} {}", i, no1.as_bytes()[i] );
                // println!("{} {}", i, no1.as_bytes()[i] * 2 );
                // let mut c = result.clone();
                // c.reverse();
                // println!("{} {} {} {} {}", n1, n2, carry, digit, sum);
                // println!("{:?}", c);
            }
        }
        // println!("{:?}", result);
        // reverse vectors then change them sto String
        let result_str: String = result.iter().rev().map(ToString::to_string).collect();

        // find index where it doesn't lead with 0
        let mut begin = 0;
        for i in result_str.chars() {
            if i != '0' {
                break;
            }
            begin += 1
        }

        // return String
        return result_str[begin..].to_string();
    }

    fn reverse_string(string: String) -> String {
        let mut new: String = String::new();
        for byte in string.chars().rev() {
            new.push(byte);
        }
        return new;
    }
}

// fn main() {
//     let num1 = String::from("123");
//     let num2 = String::from("456");
//     println!("{}", Solution::multiply(num1, num2))
// }
