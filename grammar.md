``` BNF

<program> ::= "a)" <segment>* "V.S.V"

<segment> ::= (<expression> | <loop> | <tilldela>) "."

<expression> ::= <term> <operator> (<term> | ("(" <expression> ")"))

<loop> ::= "loop" "(" <segment>* <claim> ")"

<tilldela> ::= <variabel> "<-" <expression>

<claim> ::= ":" (<term> | <expression>) <ekvivalensrelation> (<term> | <expression>)

<term> ::= <variabel> | <tal>

<operator> ::= "-" | "+"
<ekvivalensrelation> ::= "=" | "!=" 

<variabel> ::= <bokstav>+
<tal> ::= <heltal>+

<heltal> ::= [0-9]
<bokstav> ::= "x" 
        | "z"
        | "y"
        | "q"
        | "h"
        | "j"
        | "k"

``````

``` rust
// Testspråk för fibonacci_tal.
fn fibonacci_tal(n: i32) -> i32 {
    // Init variabler, samt datatyp int~
    let mut tal_1 = 1;
    let mut tal_2 = 1;


    let mut fibonacci_tal = 0;

    // Loops
    for i in 0..(n-2) {
        fibonacci_tal = tal_1 + tal_2;
        
        tal_2 = tal_1;

        tal_1 = fibonacci_tal;
    }

    // Retunera
    return fibonacci_tal
}
`````

``` rust
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
`````