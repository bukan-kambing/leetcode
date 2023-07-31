impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut product = vec![0; num1.len() + num2.len()];

        for (idx1, ch1) in num1.chars().rev().enumerate() {
            for (idx2, ch2) in num2.chars().rev().enumerate() {
                let num1 = ch1 as u8 - '0' as u8;
                let num2 = ch2 as u8 - '0' as u8;

                let res = num1 * num2 + product[idx1 + idx2];
                product[idx1 + idx2] = res % 10;
                product[idx1 + idx2 + 1] += res / 10;
            }
        }

        while product.len() > 1 && *product.last().unwrap() == 0 {
            product.pop();
        }

        product
            .into_iter()
            .rev()
            .map(|num| num.to_string())
            .collect::<String>()
    }
}
