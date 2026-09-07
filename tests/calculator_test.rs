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
}
