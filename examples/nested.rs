use chumsky::zero_copy::{input::WithContext, prelude::*};

// This token is a tree: it contains within it a sub-tree of tokens
#[derive(Clone, PartialEq, Debug)]
enum Token {
    Num(i64),
    Add,
    Mul,
    Parens(Vec<Token>),
}

fn parser<'a>() -> impl Parser<'a, WithContext<(), &'a [Token]>, i64> {
    recursive(|expr| {
        let num = select! { Token::Num(x) => *x };
        let parens = expr
            // Needed to ensure that we parse *all* of the nested tokens
            .then_ignore(end())
            // Here we specify how the parser should come up with the nested tokens
            .nested_in(select! { Token::Parens(xs) => WithContext::new((), xs.as_slice()) });

        let atom = num.or(parens);

        let product = atom
            .clone()
            .then(
                just(Token::Mul)
                    .ignore_then(atom)
                    .repeated()
                    .collect::<Vec<_>>(),
            )
            .foldl(|a, b| a * b);

        let sum = product
            .clone()
            .then(
                just(Token::Add)
                    .ignore_then(product)
                    .repeated()
                    .collect::<Vec<_>>(),
            )
            .foldl(|a, b| a + b);

        sum
    })
    .then_ignore(end())
}

fn main() {
    let tokens = [
        Token::Parens(vec![Token::Num(2), Token::Add, Token::Num(3)]),
        Token::Mul,
        Token::Num(4),
    ];

    assert_eq!(
        parser().parse(WithContext::new((), &tokens)).into_result(),
        Ok(20),
    );
}
