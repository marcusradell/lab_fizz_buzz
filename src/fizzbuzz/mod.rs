/**
 * Test list:
 * 1 => 1
 * 2 => 2
 * 3 => Fizz
 * 6 => Buzz
 * 9 => Fizz
 * 10 => Buzz
 * 15 => FizzBuzz
 * 30 => FizzBuzz
 */

pub fn token_at(n: i64) -> String {
    if n % 15 == 0 {
        return "FizzBuzz".to_string();
    }
    if n % 3 == 0 {
        return "Fizz".to_string();
    }
    if n % 5 == 0 {
        return "Buzz".to_string();
    }
    n.to_string()
}

pub fn run(n: i64) -> String {
    let mut result = String::new();

    for i in 1..=n {
        if i == n {
            result.push_str(&token_at(i))
        } else {
            result.push_str(&format!("{}, ", &token_at(i)));
        }
    }

    return result;
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn number_1() {
        assert_eq!(token_at(1), "1")
    }

    #[test]
    fn number_2() {
        assert_eq!(token_at(2), "2")
    }

    #[test]
    fn fizz_on_3() {
        assert_eq!(token_at(3), "Fizz")
    }
    #[test]
    fn fizz_on_6() {
        assert_eq!(token_at(6), "Fizz")
    }

    #[test]
    fn buzz_on_5() {
        assert_eq!(token_at(5), "Buzz")
    }

    #[test]
    fn buzz_on_10() {
        assert_eq!(token_at(10), "Buzz")
    }

    #[test]
    fn fizzbuzz_on_15() {
        assert_eq!(token_at(15), "FizzBuzz")
    }

    #[test]
    fn fizzbuzz_on_30() {
        assert_eq!(token_at(30), "FizzBuzz")
    }

    #[test]
    fn fizzbuzz_run() {
        assert_eq!(run(10), "1, 2, Fizz, 4, Buzz, Fizz, 7, 8, Fizz, Buzz")
    }
}
