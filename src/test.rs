use crate::{model::{Primitive, Evaluatable, CellAddress}, environment::Environment, grid::Grid, lexer::{self, Token}, parser};

fn _testing() {
    fn _print_expression_title(expression: &str, title: &str, grid: &Grid) {
        println!("{title}");
        print_expression(expression, grid);
    }

    fn print_tokens(tokens: &Vec<Token>) {
        for token in tokens {
            print!("{:?}: {:?} | ", token.token_type, token.text);
        }
        println!();
    }


    fn print_expression(expression: &str, grid: &Grid) {
        println!("INPUT: {expression}");
        
        let tokens: Vec<lexer::Token>;
        match lexer::lex(expression) {
            Ok(result) => {
                print!("TOKENS: ");
                print_tokens(&result);
                tokens = result;
            }
            Err(err) => {
                println!("LEXING ERROR: {err}\n");
                return;
            }
        }
        
        let parsed_expression: Box<dyn Evaluatable>;
        match parser::parse(tokens) {
            Ok(result) => {
                println!("PARSED VALUE: {:?}", result);
                parsed_expression = result;
            } 
            Err(err) => {
                println!("PARSING ERROR: {err}\n");
                return;
            }
        }

        match parsed_expression.evaluate(&Environment::new(grid)) {
            Ok(result) => println!("EVALUATION: {}", result.to_string()),
            Err(err) => println!("ERROR: {err}\n"),
        }

        println!();
    }

    fn header(title: &str) {
        println!("{}", title.to_uppercase());
        println!("==============================================");
    }


    // Initialize grid
    let mut grid = Grid::new();
    for i in 1..=10 {
        for j in 1..=10 {
            grid.set_cell(&CellAddress(i, j), Ok(Primitive::Integer(i * j)));
        }
    }

    header("BASIC PRIMITIVES");
    print_expression("\"this is a string\"", &grid);
    print_expression("5", &grid);
    print_expression("-5", &grid);
    print_expression("7.234", &grid);
    print_expression("true", &grid);
    print_expression("false", &grid);
    print_expression("[5, 5]", &grid);

    header("basic expressions");
    print_expression("2 + 2", &grid);
    print_expression("4 - 2", &grid);
    print_expression("3 * 2", &grid);
    print_expression("6 / 2", &grid);
    print_expression("4 & 7", &grid);
    print_expression("4 | 7", &grid);
    print_expression("4 ^ 7", &grid);
    print_expression("~7", &grid);

    header("basic boolean");
    print_expression("false || true", &grid);
    print_expression("false && true", &grid);
    print_expression("!false", &grid);

    header("equality");
    print_expression("5 == 5", &grid);
    print_expression("-0.00001 == -0.00001", &grid);
    print_expression("\"this is a string\" == \"this is a string\"", &grid);
    print_expression("false == false", &grid);

    print_expression("5 == 3", &grid);
    print_expression("-0.00001 == 0.00001", &grid);
    print_expression("\"this is a string\" == \"this is a different string\"", &grid);
    print_expression("true == false", &grid);

    print_expression("\"this is a string\" == true", &grid);

    header("inequality");
    print_expression("5 != 5", &grid);
    print_expression("-0.00001 != -0.00001", &grid);
    print_expression("\"this is a string\" != \"this is a string\"", &grid);
    print_expression("false != false", &grid);

    print_expression("5 != 3", &grid);
    print_expression("-0.00001 != 0.00001", &grid);
    print_expression("\"this is a string\" != \"this is a different string\"", &grid);
    print_expression("true != false", &grid);

    print_expression("\"this is string\" != 5", &grid);

    header("statistics");
    print_expression("max([1,1],[5,5])", &grid);
    print_expression("min([1,1],[5,5])", &grid);
    print_expression("sum([1,1],[5,5])", &grid);
    print_expression("mean([1,1], [5,5])", &grid);

    header("precedence");
    print_expression("4 | 7 ^ 8", &grid);
    print_expression("4 ^ 7 & 8", &grid);
    print_expression("4 & 7 == 4", &grid);
    print_expression("4 < 7 == 7 > 4", &grid);
    print_expression("1 << 7 < 16", &grid);
    print_expression("1 << 5 + 2", &grid);
    print_expression("4445 >> 4 - 2", &grid);
    print_expression("27 - 81 / 5,800,074", &grid);
    print_expression("2 + 5 * 9", &grid);
    print_expression("1 + 2 % 5", &grid);
    print_expression("1 * 2 ** 5", &grid);
    print_expression("!true || true", &grid);
    print_expression("~5 + 7", &grid);
    print_expression("int 5.0 / 2", &grid);
    print_expression("float 5 / 2", &grid);

    header("parenthesis");
    print_expression("(4 | 7) ^ 8", &grid);
    print_expression("(4 ^ 7) & 8", &grid);
    print_expression("(4 & 7) == 4", &grid);
    print_expression("4 < (7 == 7) > 4", &grid);
    print_expression("1 << (7 < 16)", &grid);
    print_expression("(1 << 5) + 2", &grid);
    print_expression("(4445 >> 4) - 2", &grid);
    print_expression("(27 - 81) / 5,800,074", &grid);
    print_expression("(2 + 5) * 9", &grid);
    print_expression("(1 + 2) % 5", &grid);
    print_expression("(1 * 2) ** 5", &grid);
    print_expression("!(true || true)", &grid);
    print_expression("~(5 + 7)", &grid);
    print_expression("int (5.0 / 2)", &grid);
    print_expression("float (5 / 2)", &grid);
    print_expression("((((float (((((((((5 + 8)))))))*      2))))))", &grid);

    header("Combined expressions");
    print_expression("4 | 7 & 9 + 4 - 6 * 7 ** 2 / 8 ^ 7 + 9", &grid);
    print_expression("4 | (7) + (9 & 7) ** (6 - 6) + [2, 3]", &grid);
    print_expression("true && false && true == true || false || true && true", &grid);
    print_expression("[4, 5] - max([1, 1], [1, 5]) + 0.99 ** -2.0", &grid);

    header("errors");
    print_expression("5 + ", &grid);
    print_expression("17.99999 - )", &grid);
    print_expression("this is not a (spreadterm) string, it is not in quotes", &grid);
    print_expression("[5, 4", &grid);
    print_expression("max([1, 1],)", &grid);
    print_expression("(5 - 5", &grid)
}
