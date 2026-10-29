use core::{error, panic};
use std::{fmt::{Error, format}, fs, io::{self, ErrorKind::QuotaExceeded, Read, Write}};

use crate::{Expression::Term_, Program::{Return_value, SegmentLista}, Segment::Tilldela, Term::{tal, variabel}, Token::{Loops, Tal, Variabel}};
//start of program.

#[cfg(test)]
mod tests;

//  #################
//  #               #
//  # Main function #
//  #               #
//  #################

fn main() -> io::Result<()> {
    // Taking a input
    let mut buffer = fs::read_to_string("../fibonacci_src_code.txt")?;
    // Runs lexical on tokenizer
    let token_list = tokenizer(buffer);
    // Setting up parser object
    let mut parser = Parser {
        tokenlist: token_list,
        offset: 0,
    };
    // Prints syntax_tree using debug
    let (Segment_syntax_tree, return_value) = parser.parse_program();
    // Work in progress QBE translator
    let QBE_string = ast_to_qbe(&Segment_syntax_tree, &return_value);
    println!("{}", QBE_string);



    Ok(())
}   

//  #################
//  #               #
//  # Lexical logic #
//  #               #
//  #################

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Programstart, // a)
    Programslut, // V.S.V
    Loops, // loops

    Variabel(String),
    Tal(String),

    Plus, // +
    Minus, // -
    Likamed, // =
    InteLikamed, // =
    Anta, // <-
    Punkt, // .
    Kolon, // :
    VParantes, // (
    HParantes, // )
}  

// List of valid heltal / bokstäver
static BOKSTAV_LISTA: [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
static HELTAL_LISTA: [char; 10] = ['0','1','2','3','4','5','6','7','8','9'];

// Lexical analysis
fn tokenizer(src: String) -> Vec<Token> {
    let bytes = src.as_bytes();
    let mut token_list: Vec<Token> = Vec::new();
    let mut offset = 0;

    while offset < bytes.len() {
        // Skips any and all whitespace / newlines        
        let cha = src[offset..].chars().next().unwrap();
        if cha.is_whitespace() {
            offset += cha.len_utf8();
            continue;
        }
        // Checkar för nästa token, förutom variabel och tal
        if src[offset..].starts_with("a)") {
            token_list.push(Token::Programstart);
            offset += "a)".len();
            continue;
        } else if src[offset..].starts_with("V.S.V") {
            token_list.push(Token::Programslut);
            offset += "V.S.V".len();
        } else if src[offset..].starts_with("loop") {
            token_list.push(Token::Loops);
            offset += "loop".len();
        } else if src[offset..].starts_with("+") {
            token_list.push(Token::Plus);
            offset += "+".len();
        } else if src[offset..].starts_with("-") {
            token_list.push(Token::Minus);
            offset += "-".len();
        } else if src[offset..].starts_with("!=") {
            token_list.push(Token::InteLikamed);
            offset += "!=".len();
        } else if src[offset..].starts_with("=") {
            token_list.push(Token::Likamed);
            offset += "=".len();
        } else if src[offset..].starts_with("<-") {
            token_list.push(Token::Anta);
            offset += "<-".len();
        } else if src[offset..].starts_with(".") {
            token_list.push(Token::Punkt);
            offset += ".".len();
        } else if src[offset..].starts_with(":") {
            token_list.push(Token::Kolon);
            offset += ":".len();
        } else if src[offset..].starts_with("(") {
            token_list.push(Token::VParantes);
            offset += "(".len();
        } else if src[offset..].starts_with(")") {
            token_list.push(Token::HParantes);
            offset += ")".len();
        } else if BOKSTAV_LISTA.contains(&cha) {
            let start = offset;
            offset += 1;
                    while offset < bytes.len() && BOKSTAV_LISTA.contains(&src[offset..].chars().next().unwrap()) {
                        offset += 1;
                    }
                    token_list.push(Token::Variabel(src[start..offset].to_string()));
        } else if HELTAL_LISTA.contains(&cha) {
            let start = offset;
            offset += 1;
                    while offset < bytes.len() && HELTAL_LISTA.contains(&src[offset..].chars().next().unwrap()) {
                        offset += 1;
                    }
                    token_list.push(Token::Tal(src[start..offset].to_string()));
        } 
        // Panics if no token can be assigned.
        else {
            panic!("lexError")
        }
    }

    return token_list
}

//  ################
//  #              #
//  # Parser logic #
//  #              #
//  ################

// Specificerar states för <Program>.
#[derive(Debug)]
enum Program {
    SegmentLista(Vec<Segment>),
    Return_value(Term),
}

// Specificerar states för <Segment>
#[derive(Debug)]
enum Segment {
    Expression(Expression),
    Loops {
        segments: Vec<Segment>,
        claim: Claim,
    },
    Tilldela {
        variabel: String,
        expression: Box<Expression>,
    },
}

// Specificerar states för en <expression>
#[derive(Debug)]
enum Expression {
    Term_(Term),
    Binary {
        v_l: Term,
        operator: Operator,
        h_l: Box<Expression>
    }
}
// Specificerar states för en <term>
#[derive(Debug)]
enum Term {
    variabel(String),
    tal(String),
}
// Specificerar states för en <Claim>
#[derive(Debug)]
enum Claim {
    Binary {
        v_l: Expression,
        opertor: Ekvivalensrelation,
        h_l: Expression,
    }
}
// Specificerar states för en <Ekvivalensrelation>
#[derive(Debug)]
enum Ekvivalensrelation {
    Likamed,
    InteLikamed,
}
// Specificerar states för en <Operator>
#[derive(Debug)]
enum Operator {
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
struct Parser {
    tokenlist: Vec<Token>,
    offset: usize,
}


impl Parser {
    // return nuvarande Token,
    fn peek(&self) -> Token {
        let token = self.tokenlist[self.offset].clone();
        return token
    }
    // Return, nästa Token,
    fn peek_next(&self) -> Token {
        let token = self.tokenlist[self.offset + 1].clone();
        return token
    }
    // Itererar offset
    fn next_token(&mut self) {
        self.offset += 1;
    }
    // Parsar ett program,
    fn parse_program(&mut self) -> (Vec<Segment>, Term) {
        // Check if first token is Programstart, else Error
        if self.peek() != Token::Programstart {panic!("Parse Error")}
        self.next_token();
        // Init variabel to track program segments.
        let mut segmentlista:Vec<Segment> = Vec::new();

        // Loops while new segments or programend is left
        while self.offset < self.tokenlist.len() {
            // Checkar om programmet är slut annars.
            if self.peek() == Token::Programslut {
                self.next_token();
                if matches!(self.peek(), Token::Variabel(_)) || matches!(self.peek(), Token::Tal(_)) {
                    let return_value = self.parse_term();
                    if self.offset < self.tokenlist.len() {
                        panic!("Program ends early")
                    }
                    // Returnerar segmentlistan för programmet.
                    return (segmentlista, return_value)
                }
            } else {
                segmentlista.push(self.parse_segment());
            }
        } panic!("Parse Error - No V.S.V");
    }
    // Needs to jump to next token before returning, Segment should include entier segment.
    fn parse_segment(&mut self) -> Segment {
        // Checks if segment is a loop.
        if matches!(self.peek(), Token::Loops) {
            // pushes to the next token.
            self.next_token();
            // spawns a child process to parse all segments in a loop.
            let (segmentlist, claim) = self.parse_loops();
            // pushes to the next token. Checks "."
            if self.peek() != Token::Punkt {panic!("No dot found after segment")}
            self.next_token();
            // Return the segment.
            return Segment::Loops {
                segments: segmentlist,
                claim: claim,
            }
        } 
        // If the token is a Variabel => expression or tilldela
        // Segment is a "tilldela"
        else if matches!(self.peek_next(), Token::Anta) && matches!(self.peek(), Token::Variabel(_)) {
            // om nästa token är anta -> parse tilldela
            let (variabel_temp, expression_temp) = self.parse_tilldela();
            // pushes to the next token. Checks "."
            if self.peek() != Token::Punkt {panic!("No dot found after segment")}
            self.next_token();
            return Segment::Tilldela { variabel: variabel_temp, expression: expression_temp }
        } 
        else if matches!(self.peek(), Token::Variabel(_)) || matches!(self.peek(), Token::Tal(_)) {
            // Segment is a "expression" 
            if matches!(self.peek_next(), Token::Plus) || matches!(self.peek_next(), Token::Minus) {
                // Kör parse_expression på uttrycket.
                let (term_temp, operator, expression_temp) = self.parse_expression();
                // pushes to the next token. Skips "."
                self.next_token();

                return Segment::Expression(Expression::Binary { v_l: term_temp, operator: operator, h_l: expression_temp })
            }
            panic!("Cant find Operator / Anta")
        }
        
        panic!("Cant find segment")
    }

    // Parse loops.
    fn parse_loops(&mut self) -> (Vec<Segment>, Claim) {
        let mut loop_segmentlist:Vec<Segment> = Vec::new();
        let mut v_l = Expression::Term_(Term::variabel("Temp".to_string()));
        // Check first parathesis.
        if matches!(self.peek(), Token::VParantes) {
            self.next_token();
            while self.peek() != Token::Kolon {
                loop_segmentlist.push(self.parse_segment());
            } 
            // Checks if the token is a Kolon, if so -> starts check claim
            if matches!(self.peek(), Token::Kolon) {
                // To push past kolon
                self.next_token();
                // Checks if first token is term or expression
                if matches!(self.peek_next(), Token::Likamed) || matches!(self.peek_next(), Token::InteLikamed) {
                    // if next token is some"ekvivalensrelation" -> token is term, else token is expression.
                    v_l = Expression::Term_(self.parse_term());
                } else {
                    // Token is expression.
                    let (v_l_temp, operator_temp, h_l_temp) = self.parse_expression();
                    v_l = Expression::Binary { v_l: v_l_temp, operator: operator_temp, h_l: h_l_temp };
                }
                // Sets relation to the right value.
                let relation = match self.peek() {
                    Token::Likamed => Ekvivalensrelation::Likamed,
                    Token::InteLikamed => Ekvivalensrelation::InteLikamed,
                    _ => panic!("Unknown error")
                }; 
                self.next_token();
                if matches!(self.peek_next(), Token::HParantes) {
                    let h_l = Expression::Term_(self.parse_term());
                    // Pushes past HParantes
                    self.next_token();
                    // returns claim and loop_segmentlist
                    return (loop_segmentlist, Claim::Binary { v_l: v_l, opertor: relation, h_l: h_l })
                } else {
                    let (v_l_temp, operator_temp, h_l_temp) = self.parse_expression();
                    let h_l = Expression::Binary { v_l: v_l_temp, operator: operator_temp, h_l: h_l_temp };
                    // Pushes past HParantes
                    self.next_token();
                    // returns claim and loop_segmentlist
                    return 
                    (loop_segmentlist, Claim::Binary { v_l: v_l, opertor: relation, h_l: h_l })
                }
            }
        }
        panic!("Could not parse loop")
    }
    // Parse tilldela. Return (variabel, tal)
    fn parse_tilldela(&mut self) -> (String, Box<Expression>) {
        let v_l = match self.peek() {
            Token::Variabel(text) => text,
            _ => panic!("Could no find variabel when parsing tilldela")
        };
        self.next_token();
        if matches!(self.peek(), Token::Anta) {
            self.next_token();
            // If expression logic.
            if matches!(self.peek(), Token::Variabel(_)) || matches!(self.peek(), Token::Tal(_)){
                if matches!(self.peek_next(), Token::Plus) || matches!(self.peek_next(), Token::Minus)  {
                    let (term_temp, operator_temp, expression_temp) = self.parse_expression();
                    let expression_temp = Box::new(Expression::Binary { v_l: term_temp, operator: operator_temp, h_l: expression_temp });
                    return (v_l, expression_temp)
                } else {
                    let term_temp = Box::new(Expression::Term_(self.parse_term()));
                    return (v_l, term_temp)
                }
            }
            panic!("Could no find expression or term when parsing tilldela")
        };
        // To push onto the next token (For other function to work.)
        panic!("Could not find Anta");
    }

    // Parse expression. Rekursive. 
    fn parse_expression(&mut self) -> (Term, Operator, Box<Expression>) {
        // Checks if first token is a term.
        if matches!(self.peek(), Token::Variabel(_)) || matches!(self.peek(), Token::Tal(_)) {
            let v_l = self.parse_term();
            // Checks if second token is operator.
            if matches!(self.peek(), Token::Plus) || matches!(self.peek(), Token::Minus) {
                let operator = match self.peek() {
                    Token::Plus => Operator::Plus,
                    Token::Minus => Operator::Minus,
                    _ => panic!("Could not parse operator")
                };
                self.next_token();
                // Checks if next part is expression or term.
                if matches!(self.peek(), Token::Variabel(_)) || matches!(self.peek(), Token::Tal(_)) {
                    let h_l = Box::new(Expression::Term_(self.parse_term()));
                    
                    return (v_l, operator, h_l)
                } else if matches!(self.peek(), Token::VParantes) {
                    // Push past VParantes
                    self.next_token();
                    let (v_l_temp, operator_temp, h_l_temp) = self.parse_expression();
                    let h_l = Box::new(Expression::Binary {v_l: v_l_temp, operator: operator_temp, h_l: h_l_temp});
                    
                    if self.peek() != Token::HParantes {panic!("Could no find HParantes")}
                    self.next_token(); 

                    return (v_l, operator, h_l)
                }
            }
        }
        panic!("Could not parse expression")
    }
    // parses one singular term, than pushes offset
    fn parse_term(&mut self) -> Term {
        let contents = match self.peek() {
            Token::Variabel(text) => Term::variabel(text),
            Token::Tal(text) => Term::tal(text),
            _ => panic!("Could not parse term")
        };
        self.next_token();
        return contents;
    }

}

//  #################
//  #               #
//  # Print syntax  #
//  #               #
//  #################

fn print_syntaxtree(syntaxtree: &[Segment], return_value: &Term) {
    println!("{syntaxtree:#?}");
    println!("{return_value:#?}")
}

//  ##################
//  #                #
//  # QBE Translator #
//  #                #
//  ##################

fn fetch_new_or_used_variabel(variable: &String, nested_counter: &mut usize, variable_alloc: &mut Vec<(String, String)>) -> String {
    // first checks if a variabel exists
    let variable_copy = variable_alloc.iter();

    for (x,y) in variable_copy {
        if x == variable {
            return y.to_string();
        }
    }
    // else creates a new variable
    let new_variable_slot = temp_variable_name_generation(nested_counter);

    variable_alloc.push((variable.to_string(), new_variable_slot.clone()));
    return new_variable_slot;
}

fn ast_to_qbe(syntaxtree: &[Segment], return_value: &Term) -> String {
    // Buffer for entire QBE program.
    let mut program_buffer = String::new();
    let mut nested_counter: usize = 0;
    let mut variabel_alloc: Vec<(String, String)> = Vec::new();
    let mut program_body = String::new();

    program_buffer.push_str("data $fmt_int = { b \"%d\\n\", b 0 }\n");
    // To create the main function
    program_buffer.push_str("export function w $main() {\n@start\n");


    // Placeholder, segments will be here.
    for segment in syntaxtree {
        program_body += &translate_segment(segment, &mut nested_counter, &mut variabel_alloc);
    }
    // push return to program_buff
    program_body.push_str(&translate_return_term(return_value, &mut nested_counter, &mut variabel_alloc));

    for (variable, slot) in variabel_alloc {
        program_buffer.push_str(&format!("   {slot} =l alloc4 4\n"));
        program_buffer.push_str(&format!("   storew 0, {slot}\n"));
    }

    program_buffer += &program_body;

    // To end the main function
    program_buffer.push_str("}");
    return program_buffer
}

// translates a segment into QBE
fn translate_segment(segment: &Segment, nested_counter: &mut usize, variable_alloc: &mut Vec<(String, String)>) -> String {
    let mut exterior_expression = String::new();

    match segment {
        Segment::Expression(expression) => {
            translate_expression(expression, &mut exterior_expression, nested_counter, variable_alloc);
            exterior_expression
        },
        Segment::Loops{ segments: input_segments, claim: input_claim } => translate_loop(input_segments, input_claim, nested_counter, variable_alloc),
        Segment::Tilldela { variabel: input_variabel, expression: expression} => {
            translate_tilldela(input_variabel, expression, nested_counter, variable_alloc, &mut exterior_expression);
            exterior_expression
        },
    }
}
// translate a tilldela to QBE
fn translate_tilldela(input_variabel: &String, expression: &Box<Expression>, nested_counter: &mut usize, variable_alloc: &mut Vec<(String, String)>, instruction_buff: &mut String) {
    // fetches or creates a variable
    let qbe_variable = fetch_new_or_used_variabel(input_variabel, nested_counter, variable_alloc);

    let expression_value = translate_expression(expression, instruction_buff, nested_counter, variable_alloc);
    instruction_buff.push_str(&format!("storew {expression_value}, {qbe_variable}\n"));
}


// translates a loop to QBE
fn translate_loop(input_segments: &Vec<Segment>, input_claim: &Claim, nested_counter: &mut usize, variable_alloc: &mut Vec<(String, String)>) -> String {
    let loop_id = *nested_counter;
    *nested_counter += 1;


    let loop_check = format!("@loop_check{loop_id}");
    let loop_body = format!("@loop_body{loop_id}");
    let loop_end = format!("@loop_end{loop_id}");
    
    let mut output = format!("  jmp {loop_check}\n{loop_check}\n");

    let claim = translate_claim(&input_claim, nested_counter, variable_alloc, &mut output);
    
    let mut segments = String::new();
    for segment in input_segments {
        segments += &translate_segment(&segment, nested_counter, variable_alloc);
    }
    
    output += &format!("   jnz {claim}, {loop_end}, {loop_body}\n");
    output += &format!("{loop_body}\n");
    output += &segments;
    output += &format!("   jmp {loop_check}\n{loop_end}\n");

    output
}

// translate a claim to QBE
fn translate_claim(claim: &Claim, nested_counter: &mut usize, variable_alloc: &mut Vec<(String, String)>, instruction_buff: &mut String) -> String {
    match claim {
        Claim::Binary { v_l: vänster, opertor: operator, h_l: höger } => {
            let left = translate_expression(vänster, instruction_buff, nested_counter, variable_alloc);
            let right = translate_expression(höger, instruction_buff, nested_counter, variable_alloc);
            
            let result = temp_variable_name_generation(nested_counter);
            let qbe_operator = translate_ekvivalensoperator(operator);
            
            instruction_buff.push_str(&format!(
                "   {result} =w {qbe_operator} {left}, {right}\n"
            ));
            result
        }
    }
}

fn translate_ekvivalensoperator(operator: &Ekvivalensrelation) -> String {
    match operator {
        Ekvivalensrelation::Likamed => "ceqw".to_string(),
        Ekvivalensrelation::InteLikamed => "cnew".to_string()
    }
}

// translates a expression to QBE
fn translate_expression(expression: &Expression, exterior_expression: &mut String, nested_counter: &mut usize, variable_alloc: &mut Vec<(String, String)>) -> String {
    // matches on expression case.
    match expression {
        Expression::Binary { v_l: v_l_value, operator: operator_value, h_l: h_l_value } => {
            let left = translate_term(v_l_value, nested_counter, variable_alloc, exterior_expression);
            let right = translate_expression(h_l_value, exterior_expression, nested_counter, variable_alloc);
            let temp = temp_variable_name_generation(nested_counter);
            let operator = translate_operator(operator_value);

            exterior_expression.push_str(&format!(
                "   {temp} =w {operator} {left}, {right}\n"
            ));
            return temp
        } ,
        Expression::Term_(value) => translate_term(value, nested_counter, variable_alloc, exterior_expression), // expression_buffer.push_str(value),
        _ => todo!()
    }

    //return expression_buffer
}

// generates a unique variable name based on a index.
fn temp_variable_name_generation(nested_counter: &mut usize) -> String {
    let mut temp_varible_name = String::new();
    temp_varible_name.push_str("%t");
    temp_varible_name.push_str(&nested_counter.to_string());
    // adds one to nested_counter outside this scope (* <=> pointer)
    *nested_counter += 1;
    return temp_varible_name;
}

// Takes a Term and returns the valid qbe string
fn translate_term(term: &Term, nested_counter: &mut usize, variable_alloc: &mut Vec<(String, String)>, instruction_buff: &mut String) -> String {
    match term {
        Term::tal(value) => value.clone(),
        Term::variabel(variable) => {
            let slot_number = fetch_new_or_used_variabel(variable, nested_counter, variable_alloc);
            let value = temp_variable_name_generation(nested_counter);
            instruction_buff.push_str(&format!("   {value} =w loadw {slot_number}\n"));
            value
        }
    }
}

// Takes in a operator and returns the valid qbe string.
fn translate_operator(operator: &Operator) -> String {
    match operator {
        Operator::Plus => "add".to_string(),
        Operator::Minus => "sub".to_string(),
    }
}

// translates a &Term input into QBE code.
fn translate_return_term(input_term: &Term, nested_counter: &mut usize, variable_alloc: &mut Vec<(String, String)>) -> String {
    // matches onto different term values.
    let result = match input_term {
        Term::tal(value) => translate_constant(value),
        Term::variabel(value) => {
            let mut buffer = String::new();
            let slot = fetch_new_or_used_variabel(value, nested_counter, variable_alloc);

            let result = temp_variable_name_generation(nested_counter);
            buffer.push_str(&format!("  {result} =w loadw {slot}\n"));
            buffer.push_str(&format!("%printed =w call $printf(l $fmt_int, ..., w {result})\n")); // %printed =w call $printf(l $fmt_int, w %result)
            buffer.push_str(&format!("  ret {result}\n"));
            buffer
        }
    };
    return result
}

// Translates a return constant, takes &str input
fn translate_constant(input_tal: &str) -> String {
    // Creates a new string
    let mut new_string = String::new();
    // pushes all values onto string
    new_string.push_str(&format!("%printed =w call $printf(l $fmt_int, ..., w {input_tal})\n"));
    new_string.push_str(&format!("    ret {input_tal}\n"));
    // returns string once everything has been pushed
    return new_string
}