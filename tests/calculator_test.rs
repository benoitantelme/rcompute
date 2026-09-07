#[cfg(test)]
mod calculator_test {
    use rcompute::components::calculator::Calculator;

    #[test]
    fn instantiation() {
        let calc: Calculator = Calculator::new([[0; 3]; 3], [[0; 3]; 3]);

        assert_eq!(calc.a, [[0; 3]; 3]);
        assert_eq!(calc.b, [[0; 3]; 3]);
    }

    #[test]
    fn calculate_ones() {
        let calc: Calculator = Calculator::new(
            [[1, 1, 1], [1, 1, 1], [1, 1, 1]],
            [[1, 1, 1], [1, 1, 1], [1, 1, 1]],
        );

        println!("{}", calc);
        let c = calc.calculate();

        assert_eq!(c, [[3, 3, 3], [3, 3, 3], [3, 3, 3]]);
    }

    #[test]
    fn identity() {
        let a = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let identity = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];

        let calc: Calculator = Calculator::new(a, identity);
        assert_eq!(calc.calculate(), a);

        let calc: Calculator = Calculator::new(identity, a);
        assert_eq!(calc.calculate(), a);
    }

    #[test]
    fn zero() {
        let a = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let zero = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];

        let calc: Calculator = Calculator::new(a, zero);
        assert_eq!(calc.calculate(), zero);

        let calc: Calculator = Calculator::new(zero, a);
        assert_eq!(calc.calculate(), zero);
    }

    #[test]
    fn example() {
        let a = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let b = [[9, 8, 7], [6, 5, 4], [3, 2, 1]];

        let expected = [[30, 24, 18], [84, 69, 54], [138, 114, 90]];

        let calc: Calculator = Calculator::new(a, b);
        assert_eq!(calc.calculate(), expected);
    }

    #[test]
    fn diagonal() {
        let a = [[2, 0, 0], [0, 3, 0], [0, 0, 4]];
        let b = [[5, 0, 0], [0, 6, 0], [0, 0, 7]];
        let expected = [[10, 0, 0], [0, 18, 0], [0, 0, 28]];

        let calc: Calculator = Calculator::new(a, b);
        assert_eq!(calc.calculate(), expected);
    }

    #[test]
    fn column() {
        let a = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let b = [[0, 0, 0], [0, 1, 0], [0, 0, 0]];
        let expected = [[0, 2, 0], [0, 5, 0], [0, 8, 0]];

        let calc: Calculator = Calculator::new(a, b);
        assert_eq!(calc.calculate(), expected);
    }

    #[test]
    fn random() {
        let a = [[2, 1, 3], [0, 4, 2], [5, 2, 1]];
        let b = [[1, 3, 2], [4, 0, 1], [2, 5, 3]];
        let expected = [[12, 21, 14], [20, 10, 10], [15, 20, 15]];

        let calc: Calculator = Calculator::new(a, b);
        assert_eq!(calc.calculate(), expected);
    }
}
