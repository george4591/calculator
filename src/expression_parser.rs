mod stack;
use stack::Stack;

pub struct ExpressionParser {
    m_expression: String,
    m_infix_expression: String,
    m_index: u16
}

impl ExpressionParser {
    pub fn new(expression: &str) -> Self {
        ExpressionParser{m_expression: expression, m_infix_expression: expression, m_index: 0}
    }

    fn convert_brackets(&self) -> String {
        string.chars()
        .map(|c| match c {
            ']' => ')',
            '[' => '(',
            _ => c
        })
        .collect()
    }

    fn infix_to_postfix(&self) {
        let mut output: String;
        let mut stack: Stack<isize> = Stack::new();

        self.m_expression = convert_brackets(self.m_expression);
    }
}