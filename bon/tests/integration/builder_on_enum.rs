use bon::builder;

#[test]
fn basic() {
    #[builder]
    enum Expression {
        None,
        Binary { 
            operation : char,
            left : u32,
            right : u32
        }
    }

    let expression = Expression::binary()
        .operation('+')
        .left(6u32)
        .right(5u32)
        .build();

}