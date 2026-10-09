use core::{error, panic};
use std::{fmt::Error, fs, io::{self, ErrorKind::QuotaExceeded, Read, Write}};

use crate::{Expression::Term_, Segment::Tilldela, Term::tal, Token::{Loops, Tal, Variabel}};
//start of program.

//  #################
//  #               #
//  # Main function #
//  #               #
//  #################

fn main() -> io::Result<()> {
    // Taking a input
    let mut buffer = fs::read_to_string("../test_src_code.txt")?;
    // Runs lexical on tokenizer
    let token_list = tokenizer(buffer);
    // Setting up parser object
    let mut parser = Parser {
        tokenlist: token_list,
        offset: 0,
    };
    // Prints syntax_tree using debug
    let syntax_tree = parser.parse_program();
    print_syntaxtree(&syntax_tree);

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
static BOKSTAV_LISTA: [char; 7] = ['x','z','y','q','h','j','k'];
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
    fn parse_program(&mut self) -> Vec<Segment> {
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
                if self.offset < self.tokenlist.len() {
                    panic!("Program ends early")
                }
                // Returnerar segmentlistan för programmet.
                return segmentlista
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
                    return (loop_segmentlist, Claim::Binary { v_l: v_l, opertor: relation, h_l: h_l })
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

fn print_syntaxtree(syntaxtree: &[Segment]) {
    println!("{syntaxtree:#?}");
}