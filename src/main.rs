use tamandua_rs::lexer::Lexer;

fn main() {
    let msg = "1 2 3 4 ; 5 asvcs err(2.234444) 89. .2222222 -.999 2.2.2.2  2e10 1.234E123
        /* COMMENT AREA */
        3 = 1e10 + -23 . . . + - += -= ++ == / /= *=
        +.x +.4
    /*"
    .to_string();

    let lexemes = Lexer::new(msg).lex_input();

    for (l, s) in lexemes {
        println!("{:?} : {:?}", l, s);
    }
}
