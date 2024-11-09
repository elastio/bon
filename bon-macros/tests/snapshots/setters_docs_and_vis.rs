#[allow(unused_parens)]
#[automatically_derived]
#[allow(deprecated)]
impl<S: sut_builder::State> SutBuilder<S> {
    /**_**Required.**_

*/
    /// Docs on the required field setters.
    /// Multiline.
    pub(in overridden) fn required_field(
        mut self,
        value: u32,
    ) -> SutBuilder<sut_builder::SetRequiredField<S>>
    where
        S::RequiredField: sut_builder::IsUnset,
    {}
    /**_**Optional** ([Some](Self::optional_field()) / [Option](Self::maybe_optional_field()) setters)._

*/
    /// Docs on the optional field setters.
    /// Multiline.
    pub(in overridden) fn optional_field(
        self,
        value: u32,
    ) -> SutBuilder<sut_builder::SetOptionalField<S>>
    where
        S::OptionalField: sut_builder::IsUnset,
    {}
    /**_**Optional** ([Some](Self::optional_field()) / [Option](Self::maybe_optional_field()) setters)._

*/
    /// Docs on the optional field setters.
    /// Multiline.
    pub(in overridden) fn maybe_optional_field(
        mut self,
        value: Option<u32>,
    ) -> SutBuilder<sut_builder::SetOptionalField<S>>
    where
        S::OptionalField: sut_builder::IsUnset,
    {}
    /**_**Optional** ([Some](Self::default_field()) / [Option](Self::maybe_default_field()) setters)._ _**Default:**_ ```2 + 2 * 3```.



*/
    /// Docs on the default field setters.
    /// Multiline.
    pub(in overridden) fn default_field(
        self,
        value: u32,
    ) -> SutBuilder<sut_builder::SetDefaultField<S>>
    where
        S::DefaultField: sut_builder::IsUnset,
    {}
    /**_**Optional** ([Some](Self::default_field()) / [Option](Self::maybe_default_field()) setters)._ _**Default:**_ ```2 + 2 * 3```.



*/
    /// Docs on the default field setters.
    /// Multiline.
    pub(in overridden) fn maybe_default_field(
        mut self,
        value: Option<u32>,
    ) -> SutBuilder<sut_builder::SetDefaultField<S>>
    where
        S::DefaultField: sut_builder::IsUnset,
    {}
    /**_**Optional** ([Some](Self::optional_field_with_specific_overrides()) / [Option](Self::maybe_optional_field_with_specific_overrides()) setters)._

*/
    /// Docs on some_fn
    /// Multiline.
    pub(in some_fn_overridden) fn optional_field_with_specific_overrides(
        self,
        value: u32,
    ) -> SutBuilder<sut_builder::SetOptionalFieldWithSpecificOverrides<S>>
    where
        S::OptionalFieldWithSpecificOverrides: sut_builder::IsUnset,
    {}
    /**_**Optional** ([Some](Self::optional_field_with_specific_overrides()) / [Option](Self::maybe_optional_field_with_specific_overrides()) setters)._

*/
    /// Docs on option_fn
    /// Multiline.
    pub(in option_fn_overridden) fn maybe_optional_field_with_specific_overrides(
        mut self,
        value: Option<u32>,
    ) -> SutBuilder<sut_builder::SetOptionalFieldWithSpecificOverrides<S>>
    where
        S::OptionalFieldWithSpecificOverrides: sut_builder::IsUnset,
    {}
    /**_**Optional** ([Some](Self::default_field_with_specific_overrides()) / [Option](Self::maybe_default_field_with_specific_overrides()) setters)._ _**Default:**_ ```2 + 2 * 3```.



*/
    /// Docs on some_fn
    /// Multiline.
    pub(in some_fn_overridden) fn default_field_with_specific_overrides(
        self,
        value: u32,
    ) -> SutBuilder<sut_builder::SetDefaultFieldWithSpecificOverrides<S>>
    where
        S::DefaultFieldWithSpecificOverrides: sut_builder::IsUnset,
    {}
    /**_**Optional** ([Some](Self::default_field_with_specific_overrides()) / [Option](Self::maybe_default_field_with_specific_overrides()) setters)._ _**Default:**_ ```2 + 2 * 3```.



*/
    /// Docs on option_fn
    /// Multiline.
    pub(in option_fn_overridden) fn maybe_default_field_with_specific_overrides(
        mut self,
        value: Option<u32>,
    ) -> SutBuilder<sut_builder::SetDefaultFieldWithSpecificOverrides<S>>
    where
        S::DefaultFieldWithSpecificOverrides: sut_builder::IsUnset,
    {}
    /**_**Optional** ([Some](Self::optional_field_with_inherited_overrides()) / [Option](Self::maybe_optional_field_with_inherited_overrides()) setters)._

*/
    /// Common docs
    /// Multiline.
    pub(in overridden) fn optional_field_with_inherited_overrides(
        self,
        value: u32,
    ) -> SutBuilder<sut_builder::SetOptionalFieldWithInheritedOverrides<S>>
    where
        S::OptionalFieldWithInheritedOverrides: sut_builder::IsUnset,
    {}
    /**_**Optional** ([Some](Self::optional_field_with_inherited_overrides()) / [Option](Self::maybe_optional_field_with_inherited_overrides()) setters)._

*/
    /// Docs on option_fn
    /// Multiline.
    pub(in option_fn_overridden) fn maybe_optional_field_with_inherited_overrides(
        mut self,
        value: Option<u32>,
    ) -> SutBuilder<sut_builder::SetOptionalFieldWithInheritedOverrides<S>>
    where
        S::OptionalFieldWithInheritedOverrides: sut_builder::IsUnset,
    {}
    /**_**Optional** ([Some](Self::default_field_with_inherited_overrides()) / [Option](Self::maybe_default_field_with_inherited_overrides()) setters)._ _**Default:**_ ```2 + 2 * 3```.



*/
    /// Common docs
    /// Multiline.
    pub(in overridden) fn default_field_with_inherited_overrides(
        self,
        value: u32,
    ) -> SutBuilder<sut_builder::SetDefaultFieldWithInheritedOverrides<S>>
    where
        S::DefaultFieldWithInheritedOverrides: sut_builder::IsUnset,
    {}
    /**_**Optional** ([Some](Self::default_field_with_inherited_overrides()) / [Option](Self::maybe_default_field_with_inherited_overrides()) setters)._ _**Default:**_ ```2 + 2 * 3```.



*/
    /// Docs on option_fn
    /// Multiline.
    pub(in option_fn_overridden) fn maybe_default_field_with_inherited_overrides(
        mut self,
        value: Option<u32>,
    ) -> SutBuilder<sut_builder::SetDefaultFieldWithInheritedOverrides<S>>
    where
        S::DefaultFieldWithInheritedOverrides: sut_builder::IsUnset,
    {}
}
