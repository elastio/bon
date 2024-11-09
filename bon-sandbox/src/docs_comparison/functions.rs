/// Example docs generated with `bon`
pub mod bon {
    /// Doc comment on `function`
    #[bon::builder]
    pub fn function(
        /// Doc comment on `x1`
        x1: u32,

        /// Doc comment on `x2`
        #[builder(default = 2 + 2)]
        x2: u32,

        /// Doc comment on `x3`
        x3: Option<u32>,

        /// Doc comment on `x4`
        #[builder(into)]
        x4: String,
    ) {
    }
}
