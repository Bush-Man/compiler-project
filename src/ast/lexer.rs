use std::collections::HashSet;

#[derive(PartialEq,Eq,Clone,Debug)]
pub struct TextSpan{
     start: usize,
     end:usize,
     literal:String,
     
}
impl TextSpan{
    pub fn new(start:usize,end:usize,literal:String)->Self{
        TextSpan{start,end,literal}
    }
    pub fn length(&self)->usize{
        self.end - self.start
    }

}
#[derive(PartialEq,Eq,Clone,Copy,Debug)]
#[allow(dead_code)]
pub enum TokenKind{
   Number(i64),
   Minus,
   Plus,
   Asterisk,
   Slash,
   LeftParam,
   RightParam,
   Eof,
   Bad
}
#[derive(PartialEq,Eq,Clone,Debug)]
pub struct Token{
    kind:TokenKind,
    span:TextSpan
}
impl Token{
    pub fn new(kind:TokenKind,span:TextSpan)->Self{
        Token{kind,span}
    }
}
#[derive(PartialEq,Eq,Clone, Copy)]
pub struct Lexer<'a>{
    input : &'a str,
    current_position:usize,


}

impl<'a> Lexer<'a>{
    pub fn new(input: &'a str)->Self{
        Lexer{input,current_position:0}
    }


   
  pub  fn next_token(&mut self)->Option<Token>{
        // if self.current_position > self.input.len(){
        //     return None;
        // }

        if self.current_position == self.input.len(){
            let eof_char = '\0';
            let eof_token_kind = TokenKind::Eof;
            let span = TextSpan::new(0, 0, eof_char.to_string());
            self.current_position+=1;
            return Some(Token::new(eof_token_kind, span));

        }
        let start = self.current_position;
        let c = self.current_char();
        return c.map(|c|{
       let mut kind = TokenKind::Bad;
        if self.is_number_start(&c){
            let number = self.consume_number();
            kind = TokenKind::Number(number);
            

        }else if self.is_binary_operator(&c) {
            kind = self.consume_operator().unwrap();
        
            println!("{:?}",kind);
        }else{
            kind = TokenKind::Bad;
        }
        let end = self.current_position;
        let literal = c.to_string();
        let span = TextSpan::new(start, end, literal);
        let token = Token::new(kind,span);
        return token;
        });
        
        
    }
    fn consume(&mut self)->Option<char>{
        if self.current_position > self.input.len(){
            return None;
        }
        let current_char = self.current_char()?;
        self.current_position += 1;
        return Some(current_char);
    }
    fn consume_number(&mut self)->i64{
        let mut number:i64 = 0;
        let c =self.consume().unwrap();
        
           
            if c.is_digit(10){
            
             number = number * 10 + c.to_digit(10).unwrap() as i64;
             
            }
        
        return number;
       
    }
    fn consume_operator(&mut self)->Option<TokenKind>{
        let c = self.consume().unwrap();
        return match c{
            '+' =>Some(TokenKind::Plus),
            '-' =>Some(TokenKind::Minus),
            '*' => Some(TokenKind::Asterisk),
            '/' => Some(TokenKind::Slash),
            _ => Some(TokenKind::Bad)
        };
    }
    fn is_number_start(&self,c:&char)->bool{
        c.is_digit(10)
    }
    fn is_binary_operator(&self,c:&char)->bool{
        let operators: HashSet<char> = ['+', '-', '*', '/', '%', '&', '|'].iter().cloned().collect();
        operators.contains(c)
    }

    // fn peek_char(&self)->Option<char>{
    //     self.input.chars().nth(self.current_position + 1)
    // }
    
    

    fn current_char(&self)->Option<char>{
        return self.input.chars().nth(self.current_position);
    }
    

}
