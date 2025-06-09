use crate::ast::{
    self, Assignment, BlockStatement, CallExpression, CompoundAssignment, Expression, ForStatement,
    FunctionDefinition, Identifier, IfStatement, InfixExpression, Operator, PostfixExpression,
    PrefixExpression, Program, ReturnStatement, Statement, UnpackExpression, WhileStatement,
};
use crate::token::{Token, TokenType};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Assign,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Exponent,
    Prefix,
    Postfix,
    Call,
    Index,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::default();
        let mut loop_count = 0;
        while !self.is_at_end() {
            loop_count += 1;
            
            if loop_count > 1000 {
                self.errors.push("Parser infinite loop detected".to_string());
                break;
            }
            
            // Skip any newlines before parsing statements
            while self.peek().token_type == TokenType::Newline {
                self.advance();
            }
            
            if self.is_at_end() {
                break;
            }
            
            match self.parse_statement() {
                Ok(stmt) => {
                    program.statements.push(stmt);
                }
                Err(e) => {
                    self.errors.push(e);
                    // Safety: advance past the problematic token to prevent infinite loops
                    if !self.is_at_end() {
                        self.advance();
                    }
                }
            }
        }
        program
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.peek().token_type {
            TokenType::Spell => self.parse_function_definition(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::If => self.parse_if_statement(),
            TokenType::While => self.parse_while_statement(),
            TokenType::For => self.parse_for_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_function_definition(&mut self) -> Result<Statement, String> {
        Err("Parsing for function definitions is not yet implemented".to_string())
    }

    fn parse_if_statement(&mut self) -> Result<Statement, String> {
        self.consume(TokenType::If, "Expected 'if' keyword.")?;
        let condition = self.parse_expression(Precedence::Lowest)?;
        self.consume(TokenType::Colon, "Expected ':' after if condition.")?;
        
        // Skip newline after colon
        if self.peek().token_type == TokenType::Newline {
            self.advance();
        }
        
        let consequence = self.parse_block_statement()?;
        
        let mut alternatives = Vec::new();
        let mut default = None;
        
        // Parse otherwise clauses with production limits
        let mut otherwise_count = 0;
        const MAX_OTHERWISE_CLAUSES: usize = 50; // Production limit
        
        while self.peek().token_type == TokenType::Otherwise && otherwise_count < MAX_OTHERWISE_CLAUSES {
            otherwise_count += 1;
            
            self.advance(); // consume 'otherwise'
            let alt_condition = self.parse_expression(Precedence::Lowest)?;
            self.consume(TokenType::Colon, "Expected ':' after otherwise condition.")?;
            
            // Skip newline after colon
            if self.peek().token_type == TokenType::Newline {
                self.advance();
            }
            
            let alt_consequence = self.parse_block_statement()?;
            alternatives.push((alt_condition, alt_consequence));
            
            // Skip newlines and dedents between otherwise clauses (with safety limit)
            let mut skip_count = 0;
            while (self.peek().token_type == TokenType::Newline || self.peek().token_type == TokenType::Dedent) && skip_count < 100 {
                self.advance();
                skip_count += 1;
            }
        }
        
        if otherwise_count >= MAX_OTHERWISE_CLAUSES {
            return Err(format!("Too many otherwise clauses: maximum {} allowed", MAX_OTHERWISE_CLAUSES));
        }
        
        // Parse else clause
        if self.peek().token_type == TokenType::Else {
            self.advance(); // consume 'else'
            self.consume(TokenType::Colon, "Expected ':' after else.")?;
            
            // Skip newline after colon
            if self.peek().token_type == TokenType::Newline {
                self.advance();
            }
            
            default = Some(self.parse_block_statement()?);
            
            // Skip newlines and dedents after the else block
            while self.peek().token_type == TokenType::Newline || self.peek().token_type == TokenType::Dedent {
                self.advance();
            }
        }
        
        Ok(Statement::If(IfStatement {
            condition: Box::new(condition),
            consequence,
            alternatives,
            default,
        }))
    }

    fn parse_while_statement(&mut self) -> Result<Statement, String> {
        self.consume(TokenType::While, "Expected 'while' keyword.")?;
        let condition = self.parse_expression(Precedence::Lowest)?;
        self.consume(TokenType::Colon, "Expected ':' after while condition.")?;
        
        // Skip newline after colon
        if self.peek().token_type == TokenType::Newline {
            self.advance();
        }
        
        let body = self.parse_block_statement()?;
        
        Ok(Statement::While(WhileStatement {
            condition: Box::new(condition),
            body,
        }))
    }

    fn parse_for_statement(&mut self) -> Result<Statement, String> {
        self.consume(TokenType::For, "Expected 'for' keyword.")?;
        
        let target_token = self.consume(TokenType::Identifier, "Expected variable name in for loop.")?;
        let target = Identifier(target_token.literal.clone());
        
        self.consume(TokenType::In, "Expected 'in' keyword in for loop.")?;
        let iter = self.parse_expression(Precedence::Lowest)?;
        self.consume(TokenType::Colon, "Expected ':' after for clause.")?;
        
        // Skip newline after colon
        if self.peek().token_type == TokenType::Newline {
            self.advance();
        }
        
        let body = self.parse_block_statement()?;
        
        Ok(Statement::For(ForStatement {
            target,
            iter: Box::new(iter),
            body,
        }))
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement, String> {
        self.parse_block_statement_with_limit(100) // Production limit
    }

    fn parse_block_statement_with_limit(&mut self, max_statements: usize) -> Result<BlockStatement, String> {
        let mut statements = Vec::new();
        
        // Skip any leading newlines
        while self.peek().token_type == TokenType::Newline {
            self.advance();
        }
        
        // Check for INDENT token (proper indented block)
        if self.peek().token_type == TokenType::Indent {
            self.advance(); // consume INDENT
            
            // Parse statements until we hit a DEDENT
            let mut statement_count = 0;
            while !self.is_at_end() && statement_count < max_statements {
                // Skip newlines within the block
                while self.peek().token_type == TokenType::Newline {
                    self.advance();
                }
                
                // Check for end of block
                match self.peek().token_type {
                    TokenType::Dedent | TokenType::Eof => break,
                    TokenType::Otherwise | TokenType::Else => break,
                    _ => {}
                }
                
                match self.parse_statement() {
                    Ok(stmt) => {
                        statements.push(stmt);
                        statement_count += 1;
                    },
                    Err(e) => return Err(e),
                }
            }
            
            if statement_count >= max_statements {
                return Err(format!("Block too large: maximum {} statements allowed", max_statements));
            }
            
            // Consume the DEDENT if present
            if self.peek().token_type == TokenType::Dedent {
                self.advance();
            }
        } else {
            // No indentation - parse a single statement (inline style)
            if !self.is_at_end() && 
               self.peek().token_type != TokenType::Otherwise && 
               self.peek().token_type != TokenType::Else && 
               self.peek().token_type != TokenType::Eof &&
               self.peek().token_type != TokenType::Dedent {
                match self.parse_statement() {
                    Ok(stmt) => statements.push(stmt),
                    Err(e) => return Err(e),
                }
            }
        }
        
        Ok(statements)
    }

    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        self.consume(TokenType::Return, "Expected 'return' keyword.")?;
        let value = if self.peek().token_type == TokenType::Newline
            || self.peek().token_type == TokenType::Eof
        {
            None
        } else {
            Some(self.parse_expression(Precedence::Lowest)?)
        };
        Ok(Statement::Return(ReturnStatement { value }))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, String> {
        // Check if this could be an assignment statement
        let checkpoint = self.current;
        
        // Try to parse assignment targets
        let mut targets = Vec::new();
        loop {
            match self.parse_expression(Precedence::Assign) {
                Ok(expr) => targets.push(expr),
                Err(_) => {
                    // Reset and parse as regular expression
                    self.current = checkpoint;
                    let expr = self.parse_expression(Precedence::Lowest)?;
                    if self.peek().token_type == TokenType::Newline {
                        self.advance();
                    }
                    return Ok(Statement::Expression(expr));
                }
            }
            
            // Check for comma (multiple assignment) or assignment operator
            match self.peek().token_type {
                TokenType::Comma => {
                    self.advance(); // consume comma
                    continue;
                }
                TokenType::Assign => {
                    self.advance(); // consume =
                    
                    // Parse comma-separated values for multiple assignment
                    let mut values = Vec::new();
                    loop {
                        values.push(self.parse_expression(Precedence::Assign)?);
                        if self.peek().token_type != TokenType::Comma {
                            break;
                        }
                        self.advance(); // consume comma
                    }
                    
                    // If only one value, use it directly; otherwise create a List
                    let value = if values.len() == 1 {
                        values.into_iter().next().unwrap()
                    } else {
                        Expression::List(values)
                    };
                    
                    if self.peek().token_type == TokenType::Newline {
                        self.advance();
                    }
                    return Ok(Statement::Assignment(Assignment {
                        targets,
                        value: Box::new(value),
                    }));
                }
                TokenType::PlusAssign | TokenType::MinusAssign | 
                TokenType::AsteriskAssign | TokenType::SlashAssign => {
                    if targets.len() != 1 {
                        return Err("Compound assignment requires exactly one target".to_string());
                    }
                    let op_token = self.advance();
                    let operator = match op_token.token_type {
                        TokenType::PlusAssign => ast::Operator::Plus,
                        TokenType::MinusAssign => ast::Operator::Minus,
                        TokenType::AsteriskAssign => ast::Operator::Multiply,
                        TokenType::SlashAssign => ast::Operator::Divide,
                        _ => unreachable!(),
                    };
                    let value = self.parse_expression(Precedence::Lowest)?;
                    if self.peek().token_type == TokenType::Newline {
                        self.advance();
                    }
                    return Ok(Statement::CompoundAssignment(CompoundAssignment {
                        target: targets.into_iter().next().unwrap(),
                        operator,
                        value: Box::new(value),
                    }));
                }
                _ => {
                    // Not an assignment, reset and parse as expression
                    self.current = checkpoint;
                    let expr = self.parse_expression(Precedence::Lowest)?;
                    if self.peek().token_type == TokenType::Newline {
                        self.advance();
                    }
                    return Ok(Statement::Expression(expr));
                }
            }
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, String> {
        let mut left_expr = match self.peek().token_type {
            TokenType::Identifier => self.parse_identifier(),
            TokenType::Integer => self.parse_integer_literal(),
            TokenType::Float => self.parse_float_literal(),
            TokenType::StringLit => self.parse_string_literal(),
            TokenType::True | TokenType::False => self.parse_boolean_literal(),
            TokenType::LeftParen => self.parse_grouped_expression(),
            TokenType::Minus | TokenType::Not | TokenType::Increment | TokenType::Decrement => {
                self.parse_prefix_expression()
            }
            TokenType::LeftBracket => self.parse_list_expression(),
            TokenType::LeftBrace => self.parse_dict_expression(),

            _ => Err(format!(
                "No prefix parsing function found for token: {}",
                self.peek()
            )),
        }?;

        while precedence < self.peek_precedence() {
            left_expr = match self.peek().token_type {
                TokenType::Plus
                | TokenType::Minus
                | TokenType::Asterisk
                | TokenType::Slash
                | TokenType::Mod
                | TokenType::Exponent
                | TokenType::Equality
                | TokenType::NotEqual
                | TokenType::LessThan
                | TokenType::GreaterThan
                | TokenType::LessThanEqual
                | TokenType::GreaterThanEqual
                | TokenType::And
                | TokenType::Or => self.parse_infix_expression(left_expr)?,

                TokenType::Increment | TokenType::Decrement => {
                    self.parse_postfix_expression(left_expr)?
                }

                TokenType::LeftParen => self.parse_call_expression(left_expr)?,

                TokenType::LeftBracket => self.parse_index_expression(left_expr)?,
                _ => return Ok(left_expr),
            }
        }

        Ok(left_expr)
    }

    fn parse_identifier(&mut self) -> Result<Expression, String> {
        let ident_token = self.advance();
        Ok(Expression::Identifier(Identifier(
            ident_token.literal.clone(),
        )))
    }

    fn parse_integer_literal(&mut self) -> Result<Expression, String> {
        let int_token = self.advance();
        match int_token.literal.parse::<i64>() {
            Ok(value) => Ok(Expression::IntegerLiteral(value)),
            Err(_) => Err(format!(
                "Could not parse '{}' as an integer.",
                int_token.literal
            )),
        }
    }
    fn parse_float_literal(&mut self) -> Result<Expression, String> {
        let float_token = self.advance();
        match float_token.literal.parse::<f64>() {
            Ok(value) => Ok(Expression::FloatLiteral(value)),
            Err(_) => Err(format!(
                "Could not parse '{}' as a float.",
                float_token.literal
            )),
        }
    }

    fn parse_string_literal(&mut self) -> Result<Expression, String> {
        let str_token = self.advance();
        Ok(Expression::StringLiteral(str_token.literal.clone()))
    }

    fn parse_boolean_literal(&mut self) -> Result<Expression, String> {
        let bool_token = self.advance();
        Ok(Expression::BooleanLiteral(
            bool_token.token_type == TokenType::True,
        ))
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, String> {
        self.consume(TokenType::LeftParen, "Expected '(' for grouped expression.")?;
        let expr = self.parse_expression(Precedence::Lowest)?;
        self.consume(
            TokenType::RightParen,
            "Expected ')' to close grouped expression.",
        )?;
        Ok(expr)
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, String> {
        let prefix_token = self.advance().clone();
        let operator = self.map_token_to_prefix_operator(prefix_token.token_type)?;
        let right = self.parse_expression(Precedence::Prefix)?;
        Ok(Expression::Prefix(PrefixExpression {
            operator,
            right: Box::new(right),
        }))
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, String> {
        let infix_token = self.advance().clone();
        let precedence = self.get_precedence(infix_token.token_type);
        let operator = self.map_token_to_infix_operator(infix_token.token_type)?;
        let right = self.parse_expression(precedence)?;
        Ok(Expression::Infix(InfixExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }

    fn parse_postfix_expression(&mut self, left: Expression) -> Result<Expression, String> {
        let postfix_token = self.advance().clone();
        let operator = self.map_token_to_postfix_operator(postfix_token.token_type)?;
        Ok(Expression::Postfix(PostfixExpression {
            left: Box::new(left),
            operator,
        }))
    }

    fn parse_call_expression(&mut self, function: Expression) -> Result<Expression, String> {
        self.consume(TokenType::LeftParen, "Expected '(' for function call.")?;
        let mut arguments = Vec::new();
        if self.peek().token_type != TokenType::RightParen {
            loop {
                arguments.push(self.parse_expression(Precedence::Lowest)?);
                if self.peek().token_type != TokenType::Comma {
                    break;
                }
                self.consume(TokenType::Comma, "Expected ',' between arguments.")?;
            }
        }
        self.consume(
            TokenType::RightParen,
            "Expected ')' to close function call.",
        )?;
        Ok(Expression::Call(CallExpression {
            function: Box::new(function),
            arguments,
        }))
    }

    fn parse_index_expression(&mut self, array: Expression) -> Result<Expression, String> {
        self.consume(TokenType::LeftBracket, "Expected '[' for index expression.")?;
        let index = self.parse_expression(Precedence::Lowest)?;
        self.consume(
            TokenType::RightBracket,
            "Expected ']' to close index expression.",
        )?;
        Ok(Expression::Index(ast::IndexExpression {
            object: Box::new(array),
            index: Box::new(index),
        }))
    }

    fn parse_list_expression(&mut self) -> Result<Expression, String> {
        self.consume(TokenType::LeftBracket, "Expected '[' for list literal.")?;
        let mut elements = Vec::new();
        
        while self.peek().token_type != TokenType::RightBracket && !self.is_at_end() {
            elements.push(self.parse_expression(Precedence::Lowest)?);
            
            if self.peek().token_type == TokenType::Comma {
                self.advance();
            } else if self.peek().token_type != TokenType::RightBracket {
                return Err("Expected ',' or ']' in list literal.".to_string());
            }
        }
        
        self.consume(TokenType::RightBracket, "Expected ']' to close list literal.")?;
        Ok(Expression::List(elements))
    }

    fn parse_dict_expression(&mut self) -> Result<Expression, String> {
        self.consume(TokenType::LeftBrace, "Expected '{' for dictionary literal.")?;
        let mut pairs = Vec::new();
        
        while self.peek().token_type != TokenType::RightBrace && !self.is_at_end() {
            let key = self.parse_expression(Precedence::Lowest)?;
            self.consume(TokenType::Colon, "Expected ':' after dictionary key.")?;
            let value = self.parse_expression(Precedence::Lowest)?;
            pairs.push((key, value));
            
            if self.peek().token_type == TokenType::Comma {
                self.advance();
            } else if self.peek().token_type != TokenType::RightBrace {
                return Err("Expected ',' or '}' in dictionary literal.".to_string());
            }
        }
        
        self.consume(TokenType::RightBrace, "Expected '}' to close dictionary literal.")?;
        Ok(Expression::Dict { pairs })
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, String> {
        if self.peek().token_type == token_type {
            Ok(self.advance())
        } else {
            Err(message.to_string())
        }
    }

    fn get_precedence(&self, token_type: TokenType) -> Precedence {
        match token_type {
            TokenType::Assign
            | TokenType::PlusAssign
            | TokenType::MinusAssign
            | TokenType::AsteriskAssign
            | TokenType::SlashAssign => Precedence::Assign,
            TokenType::Or => Precedence::Or,
            TokenType::And => Precedence::And,
            TokenType::Equality | TokenType::NotEqual => Precedence::Equality,
            TokenType::LessThan
            | TokenType::GreaterThan
            | TokenType::LessThanEqual
            | TokenType::GreaterThanEqual => Precedence::Comparison,
            TokenType::Plus | TokenType::Minus => Precedence::Term,
            TokenType::Asterisk | TokenType::Slash | TokenType::Mod => Precedence::Factor,
            TokenType::Exponent => Precedence::Exponent,
            TokenType::LeftParen => Precedence::Call,
            TokenType::LeftBracket => Precedence::Index,
            TokenType::Increment | TokenType::Decrement => Precedence::Postfix,
            _ => Precedence::Lowest,
        }
    }

    fn peek_precedence(&self) -> Precedence {
        self.get_precedence(self.peek().token_type)
    }

    fn map_token_to_prefix_operator(&self, tt: TokenType) -> Result<Operator, String> {
        match tt {
            TokenType::Minus => Ok(Operator::Minus),
            TokenType::Not => Ok(Operator::Not),
            TokenType::Increment => Ok(Operator::Increment),
            TokenType::Decrement => Ok(Operator::Decrement),
            _ => Err(format!(
                "Cannot map token type {:?} to a prefix operator.",
                tt
            )),
        }
    }

    fn map_token_to_infix_operator(&self, tt: TokenType) -> Result<Operator, String> {
        match tt {
            TokenType::Plus => Ok(Operator::Plus),
            TokenType::Minus => Ok(Operator::Minus),
            TokenType::Asterisk => Ok(Operator::Multiply),
            TokenType::Slash => Ok(Operator::Divide),
            TokenType::Equality => Ok(Operator::Equal),
            TokenType::NotEqual => Ok(Operator::NotEqual),
            TokenType::LessThan => Ok(Operator::LessThan),
            TokenType::GreaterThan => Ok(Operator::GreaterThan),
            TokenType::LessThanEqual => Ok(Operator::LessThanEqual),
            TokenType::GreaterThanEqual => Ok(Operator::GreaterThanEqual),

            _ => Err(format!(
                "Cannot map token type {:?} to an infix operator.",
                tt
            )),
        }
    }

    fn map_token_to_postfix_operator(&self, tt: TokenType) -> Result<Operator, String> {
        match tt {
            TokenType::Increment => Ok(Operator::Increment),
            TokenType::Decrement => Ok(Operator::Decrement),
            _ => Err(format!(
                "Cannot map token type {:?} to a postfix operator.",
                tt
            )),
        }
    }
}
