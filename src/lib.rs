///
/// 簡単な計算文字列を解析し、計算した結果を取得します。
///
pub fn calculation(target: &String) -> Result<String, String> {
    // トークン単位に切り分け
    // let mut tokens = tokenize(target)?;

    // 切り分けたトークンを元に計算
    // parse_token(&mut tokens);


    todo!()
}


/// 文字列をトークン単位に切り分けます
fn tokenize(target: &String) -> Result<Vec<Token>, String> {
    todo!()
}

fn parse_token(target: &mut Vec<Token>, index: usize) -> (Block, usize) {
    let mut stack = Vec::new();


    for mut index in index..target.len() {
        match target.get(index).unwrap() {
            Token::Brackets(value) => {
                match value {
                    Brackets::Start => {
                        let (b, i)
                            = parse_token(&mut target[1..].to_vec(), index);
                        stack.push(Value::Block(Box::new(b)));
                        index = i;
                    },
                    Brackets::End => {
                        let b = Block::new(
                            stack.pop().unwrap(),
                            stack.pop().unwrap(),
                            stack.pop().unwrap(),
                        );

                        return (b, index);
                    },
                }
            },
            Token::Value(value) => {
                stack.push(Value::Val(value.to_string()));
            },
            Token::Operator(value) => {
                let operation = match value {
                    Operator::Plus => Operator::Plus,
                    Operator::Minus => Operator::Minus,
                    Operator::Multiply => Operator::Multiply,
                    Operator::Divide => Operator::Divide,
                };

                if stack.len() == 3 {
                    stack.clear();

                    let block = Block::new(
                        stack.pop().unwrap(),
                        stack.pop().unwrap(),
                        stack.pop().unwrap(),
                    );
                    stack.push(Value::Block(Box::new(block)));
                }
                stack.push(Value::Op(operation));
            },
        }
    }

    let block = Block::new(
        stack.pop().unwrap(),
        stack.pop().unwrap(),
        stack.pop().unwrap(),
    );

    (block, 0)
}


#[derive(Clone)]
enum Token {
    Value(String),
    Operator(Operator),
    Brackets(Brackets),
}

#[derive(Clone)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Clone)]
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