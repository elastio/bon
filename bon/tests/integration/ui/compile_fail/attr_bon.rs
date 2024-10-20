use bon::bon;

struct InvalidAttrsForBonMacro;

#[bon(attrs)]
impl InvalidAttrsForBonMacro {
    #[builder]
    fn sut() {}
}

struct BuilderAttrOnReceiver;

#[bon]
impl BuilderAttrOnReceiver {
    #[builder]
    fn sut(#[builder] &self) {}
}


fn main() {}
