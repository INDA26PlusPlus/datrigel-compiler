use crate::{Expression::{self, Term_}, Segment, Term::{self, tal}, ast_to_qbe, temp_variable_name_generation, translate_constant, translate_expression, translate_return_term, translate_segment, translate_tilldela};

#[test]
fn test_translate_const() {
    println!("{}", translate_constant("42"))
}

#[test]
fn test_translate_return_term() {
    let test_term = Term::tal("100".to_string());
}

#[test]
fn test_expression_translator_onlyterm() {
    let test_expression = Expression::Term_(tal("100".to_string()));
    let mut temp_string = String::new();
    let mut temp: usize = 0;
    let mut variable_alloc: Vec<(String, String)> = Vec::new();
    println!("{}", translate_expression(&test_expression, &mut temp_string, &mut temp, &mut variable_alloc))
}

#[test]
fn test_varible_name_generation() {
    let mut temp_counter: usize = 0;
    let variable_name_1 = temp_variable_name_generation(&mut temp_counter);
    let variable_name_2 = temp_variable_name_generation(&mut temp_counter);
    assert_ne!(variable_name_1, variable_name_2)
}

#[test]
fn test_nested_expression() {
    let temp_const: Box<Expression> = Box::new(Expression::Term_(Term::tal("5".to_string())));
    let temp_inner_expression:Box<Expression> = Box::new(Expression::Binary { v_l: (Term::tal("3".to_string())), operator: (crate::Operator::Minus), h_l: (temp_const) });
    let temp_outer_expression = Expression::Binary { v_l: Term::tal("1".to_string()), operator: (crate::Operator::Plus), h_l: temp_inner_expression };
    let mut nested_counter: usize = 0;
    let mut exterior_expression = String::new();
    let mut variable_alloc: Vec<(String, String)> = Vec::new();

    translate_expression(&temp_outer_expression, &mut exterior_expression, &mut nested_counter, &mut variable_alloc);

    println!("{}", exterior_expression)
}

#[test]
fn test_multi_segments() {
    let mut temp_segments: Vec<Segment> = Vec::new();
    let mut nested_counter: usize = 0;
    let mut exterior_expression = String::new();
    
    for i in 0..=1 {
        let temp_const: Box<Expression> = Box::new(Expression::Term_(Term::tal("5".to_string())));
        let temp_inner_expression:Box<Expression> = Box::new(Expression::Binary { v_l: (Term::tal("3".to_string())), operator: (crate::Operator::Minus), h_l: (temp_const) });
        let temp_outer_expression = Expression::Binary { v_l: Term::tal("1".to_string()), operator: (crate::Operator::Plus), h_l: temp_inner_expression };
        
        temp_segments.push(Segment::Expression(temp_outer_expression));
    }
    println!("{}", ast_to_qbe(&temp_segments, &Term::tal("100".to_string())));
}


#[test]
fn test_multi_segments_with_variables() {
    let mut temp_segments: Vec<Segment> = Vec::new();
    let mut nested_counter: usize = 0;
    let mut exterior_expression = String::new();
    
    for i in 0..=1 {

        let temp_const: Box<Expression> = Box::new(Expression::Term_(Term::variabel("absd".to_string())));
        let temp_inner_expression:Box<Expression> = Box::new(Expression::Binary { v_l: (Term::variabel("absd".to_string())), operator: (crate::Operator::Minus), h_l: (temp_const) });
        let temp_outer_expression = Expression::Binary { v_l: Term::variabel("qfe".to_string()), operator: (crate::Operator::Plus), h_l: temp_inner_expression };
        
        temp_segments.push(Segment::Expression(temp_outer_expression));
    }


    println!("{}", ast_to_qbe(&temp_segments, &Term::tal("100".to_string())));
}

#[test]
fn test_multi_tilldela_with_variables() {
    let mut temp_segments: Vec<Segment> = Vec::new();
    let mut nested_counter: usize = 0;
    let mut exterior_expression = String::new();
    
    for i in 0..=1 {

        let temp_const: Box<Expression> = Box::new(Expression::Term_(Term::variabel("absd".to_string())));
        let temp_inner_expression:Box<Expression> = Box::new(Expression::Binary { v_l: (Term::variabel("absd".to_string())), operator: (crate::Operator::Minus), h_l: (temp_const) });
        
        let temp_tilldela: Segment = Segment::Tilldela { variabel: ("geiafaif".to_string()), expression: (temp_inner_expression) };
        
        temp_segments.push(temp_tilldela);
    }


    println!("{}", ast_to_qbe(&temp_segments, &Term::tal("100".to_string())));
}