use bon::Builder;

#[derive(Builder)]
struct IncorrectOrder1 {
    #[builder(start_fn)]
    _a: (),
    _b: (),
    #[builder(start_fn)]
    _c: (),
}

#[derive(Builder)]
struct IncorrectOrder2 {
    #[builder(finish_fn)]
    _a: (),
    _b: (),
    #[builder(start_fn)]
    _c: (),
}

#[derive(Builder)]
struct IncorrectOrder3 {
    _a: (),
    #[builder(start_fn)]
    _b: (),
}

#[derive(Builder)]
struct IncorrectOrder4 {
    _a: (),
    #[builder(finish_fn)]
    _b: (),
}

#[derive(Builder)]
struct IncorrectOrder5 {
    #[builder(skip)]
    _a: (),
    #[builder(start_fn)]
    _b: (),
}

#[derive(Builder)]
struct IncorrectOrder6 {
    #[builder(skip)]
    _a: (),
    #[builder(finish_fn)]
    _b: (),
}

#[derive(Builder)]
struct IncorrectOrder7 {
    #[builder(finish_fn)]
    _a: (),
    #[builder(start_fn)]
    _b: (),
}

#[derive(Builder)]
struct IncorrectOrder7 {
    #[builder(start_fn)]
    _a: (),
    #[builder(finish_fn)]
    _b: (),
    #[builder(start_fn)]
    _c: (),
}


fn main() {}
