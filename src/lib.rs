///
/// 簡単な計算文字列を解析し、計算した結果を取得します。
///
pub fn calculation(target: &String) -> Result<String, String> {
    // トークン単位に切り分け
    let tokens = tokenize(target)?;

    // 切り分けたトークンを元に計算
    blocking(&tokens);


    todo!()
}


/// 文字列をトークン単位に切り分けます
fn tokenize(target: &String) -> Result<Vec<Token>, String> {
    todo!()
}


///
fn blocking(target: &Vec<Token>) -> Block {
    todo!()
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

#[derive(Clone)]
enum Value {

}

struct Block {
    lhs: String,
    rhs: String,
    operator: Operator,
}
impl Block {
    fn new(lhs: String, rhs: String, operator: Operator) -> Self {
        Self {
            lhs,
            rhs,
            operator,
        }
    }

    fn execute(&self) -> String {
        let lhs = self.lhs.parse::<f64>().unwrap();
        let rhs = self.rhs.parse::<f64>().unwrap();

        let result = match self.operator {
            Operator::Plus => format!("{}", lhs + rhs),
            Operator::Minus => format!("{}", lhs - rhs),
            Operator::Multiply => format!("{}", lhs * rhs),
            Operator::Divide => {
                // 0除算になる場合は0を返す
                if rhs == 0_f64 {
                    return String::from("0")
                }
                format!("{}", lhs / rhs)
            },
        };

        // 小数点以下が0の場合、".0"を除去する
        if result.ends_with(".0") {
            return String::from(&result[..(result.len() - 2)])
        }

        result
    }
}




//----- TEST CODE --------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_add_test() {
    }
}