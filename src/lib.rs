///
/// 簡単な計算文字列を解析し、計算した結果を取得します。
///
pub fn calculation(target: &String) -> Result<String, String> {
    // トークン単位に切り分け
    let mut tokens = tokenize(target)?;

    // 切り分けたトークンを元に計算し、返却する
    Ok(parse_token(&mut tokens))
}


/// 文字列をトークン単位に切り分けます
fn tokenize(target: &String) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    let chars = target.chars().collect::<Vec<char>>();
    let mut index = 0;
    while index < chars.len() {
        println!("1={}", chars.get(index).unwrap());
        match chars.get(index).unwrap() {
            ' ' => {
                index += 1;
                continue;
            },
            '(' => tokens.push(Token::Brackets(Brackets::Start)),
            ')' => tokens.push(Token::Brackets(Brackets::End)),
            '+' => tokens.push(Token::Operator(Operator::Plus)),
            '-' => tokens.push(Token::Operator(Operator::Minus)),
            '*' => tokens.push(Token::Operator(Operator::Multiply)),
            '/' => tokens.push(Token::Operator(Operator::Divide)),
            // 連続する数字は1つのトークンとして結合
            // 「.」が連続して出現した場合はエラーとする
            '0' | '1' | '2' | '3' | '4' |
            '5' | '6' | '7' | '8' | '9' => {
                println!("2={}", chars.get(index).unwrap());
                let mut num = chars.get(index).unwrap().to_string();

                index += 1;
                while index < chars.len() {
                    println!("3={}", chars.get(index).unwrap());
                    match chars.get(index).unwrap() {
                        '0' | '1' | '2' | '3' | '4' |
                        '5' | '6' | '7' | '8' | '9' => {
                            num.push_str(
                                chars.get(index).unwrap().to_string().as_str()
                            );
                        },
                        '.' => {
                            if num.ends_with(".") {
                                return Err(
                                    String::from("「.」が連続して出現しました。"));
                            }
                            num.push('.');
                        },
                        _ => break,
                    }
                    index += 1;
                }

                tokens.push(Token::Value(num));
                continue;
            },
            _ => return Err(
                format!("予期せぬ文字が出現しました。「{}」"
                        , chars.get(index).unwrap().to_string())
            ),
        }
        index += 1;
    }

    Ok(tokens)
}

fn parse_token(target: &mut Vec<Token>) -> String {
    todo!()
}


#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Value(String),
    Operator(Operator),
    Brackets(Brackets),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Brackets {
    Start,
    End,
}

enum Value {
    Val(String),
    Op(Operator),
    Block(Box<Block>),
}

struct Block {
    lhs: Value,
    rhs: Value,
    operator: Value,
}
impl Block {
    fn new(lhs: Value, rhs: Value, operator: Value) -> Self {
        Self {
            lhs,
            rhs,
            operator,
        }
    }

    fn execute(&self) -> String {
        // 左辺の計算
        let lhs = match &self.lhs {
            Value::Val(value) => {
                value.to_string()
            },
            Value::Block(value) => {
                value.execute()
            },
            _ => panic!("左辺に演算子が出現しました。"),
        };
        let lhs = lhs.parse::<f64>().unwrap();

        // 右辺の計算
        let rhs = match &self.rhs {
            Value::Val(value) => {
                value.to_string()
            },
            Value::Block(value) => {
                value.execute()
            },
            _ => panic!("右辺に演算子が出現しました。"),
        };
        let rhs = rhs.parse::<f64>().unwrap();

        // 演算子ごとに計算し、その結果を返却
        match &self.operator {
            Value::Op(value) => {
                match value {
                    Operator::Plus => (lhs + rhs).to_string(),
                    Operator::Minus => (lhs - rhs).to_string(),
                    Operator::Multiply => (lhs * rhs).to_string(),
                    Operator::Divide => (lhs / rhs).to_string(),
                }
            },
            _ => panic!("演算子を想定していましたが、演算子以外が出現しました。")
        }
    }
}




//----- TEST CODE --------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::Block;
    use crate::Operator;
    use super::*;

    #[test]
    fn tokenize_test1() {
        let expect = vec![
            Token::Value(String::from("1")),
            Token::Operator(Operator::Plus),
            Token::Value(String::from("2")),
            Token::Operator(Operator::Minus),
            Token::Value(String::from("3")),
        ];

        let formula = String::from("1+2 - 3");
        let tokens = tokenize(&formula).unwrap();

        assert_eq!(expect, tokens);
    }

    #[test]
    fn tokenize_test2() {
        let expect = vec![
            Token::Value(String::from("100")),
            Token::Operator(Operator::Multiply),
            Token::Value(String::from("20")),
            Token::Operator(Operator::Divide),
            Token::Value(String::from("1505")),
        ];

        let formula = String::from("100 *20 /1505");
        let tokens = tokenize(&formula).unwrap();

        assert_eq!(expect, tokens);
    }

    #[test]
    fn tokenize_test3() {
        let expect = vec![
            Token::Value(String::from("12.34")),
            Token::Operator(Operator::Minus),
            Token::Value(String::from("5.678")),
        ];

        let formula = String::from("12.34-5.678");
        let tokens = tokenize(&formula).unwrap();

        assert_eq!(expect, tokens);
    }

    #[test]
    fn tokenize_test4() {
        let expect = vec![
            Token::Brackets(Brackets::Start),
            Token::Value(String::from("2")),
            Token::Operator(Operator::Multiply),
            Token::Value(String::from("2")),
            Token::Brackets(Brackets::End),
            Token::Operator(Operator::Plus),
            Token::Value(String::from("30")),
        ];

        let formula = String::from("(2 * 2) + 30");
        let tokens = tokenize(&formula).unwrap();

        assert_eq!(expect, tokens);
    }

    #[test]
    fn tokenize_err_test1() {
        let formula = String::from("23.5 + 10..45");
        let tokens = tokenize(&formula);

        match tokens {
            Ok(_) => {panic!("エラーが発生しませんでした。")},
            Err(value) => assert_eq!("「.」が連続して出現しました。", value),
        }
    }

    #[test]
    fn tokenize_err_test2() {
        let formula = String::from("1 + あ");
        let tokens = tokenize(&formula);

        match tokens {
            Ok(_) => {panic!("エラーが発生しませんでした。")},
            Err(value) => {
                assert_eq!("予期せぬ文字が出現しました。「あ」", value)
            },
        }
    }

    // 1 + 2
    #[test]
    fn block_execute_plus_test() {
        let block = Block::new(
            Value::Val(String::from("1")),
            Value::Val(String::from("2")),
            Value::Op(Operator::Plus),
        );

        assert_eq!("3", block.execute().as_str());
    }

    // 1 - 2
    #[test]
    fn block_execute_minus_test() {
        let block = Block::new(
            Value::Val(String::from("1")),
            Value::Val(String::from("2")),
            Value::Op(Operator::Minus),
        );

        assert_eq!("-1", block.execute().as_str());
    }

    // 2 * 3
    #[test]
    fn block_execute_multiply_test() {
        let block = Block::new(
            Value::Val(String::from("2")),
            Value::Val(String::from("3")),
            Value::Op(Operator::Multiply),
        );

        assert_eq!("6", block.execute().as_str());
    }

    // 10 / 5
    #[test]
    fn block_execute_divide_test() {
        let block = Block::new(
            Value::Val(String::from("10")),
            Value::Val(String::from("5")),
            Value::Op(Operator::Divide),
        );

        assert_eq!("2", block.execute().as_str());
    }

    // 4 * 4 - 6
    #[test]
    fn block_execute_inner_block_left_test() {
        let block = Block::new(
            Value::Val(String::from("4")),
            Value::Val(String::from("4")),
            Value::Op(Operator::Multiply),
        );

        let block = Block::new(
            Value::Block(Box::new(block)),
            Value::Val(String::from("6")),
            Value::Op(Operator::Minus),
        );

        assert_eq!("10", block.execute().as_str());
    }

    // (5 + 5) / 2
    #[test]
    fn block_execute_inner_block_right_test() {
        let block = Block::new(
            Value::Val(String::from("5")),
            Value::Val(String::from("5")),
            Value::Op(Operator::Plus),
        );

        let block = Block::new(
            Value::Val(String::from("2")),
            Value::Block(Box::new(block)),
            Value::Op(Operator::Divide),
        );

        assert_eq!("0.2", block.execute().as_str());
    }

    // (3 + 7) * (6 + 4)
    #[test]
    fn block_execute_inner_block_left_right_test() {
        let block_l = Block::new(
            Value::Val(String::from("3")),
            Value::Val(String::from("7")),
            Value::Op(Operator::Plus),
        );

        let block_r = Block::new(
            Value::Val(String::from("6")),
            Value::Val(String::from("4")),
            Value::Op(Operator::Plus),
        );

        let block = Block::new(
            Value::Block(Box::new(block_l)),
            Value::Block(Box::new(block_r)),
            Value::Op(Operator::Multiply),
        );

        assert_eq!("100", block.execute().as_str());
    }

    // 24 / (2 * 2 + 4)
    #[test]
    fn block_execute_inner_block_double_test() {
        let block = Block::new(
            Value::Val(String::from("2")),
            Value::Val(String::from("2")),
            Value::Op(Operator::Multiply),
        );

        let block = Block::new(
            Value::Block(Box::new(block)),
            Value::Val(String::from("4")),
            Value::Op(Operator::Plus),
        );

        let block = Block::new(
            Value::Val(String::from("24")),
            Value::Block(Box::new(block)),
            Value::Op(Operator::Divide),
        );

        assert_eq!("3", block.execute().as_str());
    }
}