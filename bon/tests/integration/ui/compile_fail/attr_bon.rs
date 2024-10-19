use bon::bon;

struct InvalidAttrsForBonMacro;

#[bon(attrs)]
impl InvalidAttrsForBonMacro {
    #[builder]
    fn sut() {}
}

fn main() {}
