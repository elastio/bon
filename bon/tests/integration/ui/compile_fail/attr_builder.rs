use bon::{bon, builder, Builder};

#[derive(Builder)]
#[builder]
struct EmptyTopLevelBuilderAttr {}

#[derive(Builder)]
#[builder()]
struct EmptyTopLevelBuilderAttrWithParens {}

#[derive(Builder)]
struct EmptyMemberLevelBuilderAttr {
    #[builder]
    x: u32,
}

#[derive(Builder)]
struct EmptyMemberLevelBuilderAttrWithParens {
    #[builder()]
    x: u32,
}

#[builder]
fn fn_empty_member_level_builder_attr(#[builder] _x: u32) {}

#[builder]
fn fn_empty_member_level_builder_attr_with_parens(#[builder()] _x: u32) {}

struct EmptyBuilderAttr;

#[bon]
impl EmptyBuilderAttr {
    #[builder]
    fn empty_member_level_builder_attr(#[builder] _x: u32) {}
}

#[bon]
impl EmptyBuilderAttr {
    #[builder]
    fn empty_member_level_builder_attr_with_parens(#[builder()] _x: u32) {}
}

#[bon]
impl EmptyBuilderAttr {
    #[builder()]
    fn empty_top_level_builder_attr_with_parens() {}
}

#[builder]
struct LegacyBuilderProcMacroAttrOnStruct {}

fn main() {}
