use core::num;

use super::{AstBinOperatorKind, AstVisitor};

pub struct AstEvaluator{
    pub last_value:Option<i64>,

}
impl AstEvaluator{
    pub fn new()->Self{
        Self { last_value: None }
    }
}

impl AstVisitor for AstEvaluator{
    fn visit_number(&mut self,number:&super::AstNumberExpression) {
        self.last_value = Some(number.number);
    }
    fn visit_binary_expression(&mut self,binary:&super::BinaryExpr) {
           self.visit_expression(&binary.left);
           let left = self.last_value.unwrap();
           self.visit_expression(&binary.right);
           let right  = self.last_value.unwrap();

           self.last_value = Some(match binary.operator.kind {
            AstBinOperatorKind::Plus => left + right,
            AstBinOperatorKind::Minus => left - right,
            AstBinOperatorKind::Multiply => left*right,
            AstBinOperatorKind::Divide => left/ right


           });

    }
}