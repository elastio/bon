use bon::builder;

#[test]
fn basic() {
    #[builder]
    enum Expression {
        None,
        Binary { 
            operation : char,
            left : Box<Expression>,
            right : Box<Expression>
        }
    }


    let expression = Expression::binary()
        .operation('+')
        .left(Box::new(Expression::None))
        .right(Box::new(Expression::None))
        .build();

}