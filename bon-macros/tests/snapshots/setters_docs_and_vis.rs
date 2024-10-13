#[allow(unused_parens)]
#[automatically_derived]
#[allow(deprecated)]
impl<S: sut_builder::State> SutBuilder<S> {
    /**| **Required** |
| -- |

*/
    /// Docs on the required field setters.
    /// Multiline.
    #[allow(clippy::inline_always, clippy::impl_trait_in_params)]
    #[inline(always)]
    pub(in overridden) fn required_field(
        mut self,
        value: u32,
    ) -> SutBuilder<sut_builder::SetRequiredField<S>>
    where
        S::RequiredField: sut_builder::IsUnset,
    {}
    /**| **Optional** |
| -- |

**See also** a companion setter that accepts an `Option`: [`maybe_optional_field()`](Self::maybe_optional_field).

*/
    /// Docs on the optional field setters.
    /// Multiline.
    #[allow(clippy::inline_always, clippy::impl_trait_in_params)]
    #[inline(always)]
    pub(in overridden) fn optional_field(
        self,
        value: u32,
    ) -> SutBuilder<sut_builder::SetOptionalField<S>>
    where
        S::OptionalField: sut_builder::IsUnset,
    {}
    /**| **Optional** |
| -- |

**See also** a companion setter that wraps the value with `Some` internally: [`optional_field()`](Self::optional_field).

*/
    /// Docs on the optional field setters.
    /// Multiline.
    #[allow(clippy::inline_always, clippy::impl_trait_in_params)]
    #[inline(always)]
    pub(in overridden) fn maybe_optional_field(
        mut self,
        value: Option<u32>,
    ) -> SutBuilder<sut_builder::SetOptionalField<S>>
    where
        S::OptionalField: sut_builder::IsUnset,
    {}
    /**| **Optional** |
| -- |

**See also** a companion setter that accepts an `Option`: [`maybe_default_field()`](Self::maybe_default_field).

**Default:** ```2 + 2 * 3```.

*/
    /// Docs on the default field setters.
    /// Multiline.
    #[allow(clippy::inline_always, clippy::impl_trait_in_params)]
    #[inline(always)]
    pub(in overridden) fn default_field(
        self,
        value: u32,
    ) -> SutBuilder<sut_builder::SetDefaultField<S>>
    where
        S::DefaultField: sut_builder::IsUnset,
    {}
    /**| **Optional** |
| -- |

**See also** a companion setter that wraps the value with `Some` internally: [`default_field()`](Self::default_field).

**Default:** ```2 + 2 * 3```.

*/
    /// Docs on the default field setters.
    /// Multiline.
    #[allow(clippy::inline_always, clippy::impl_trait_in_params)]
    #[inline(always)]
    pub(in overridden) fn maybe_default_field(
        mut self,
        value: Option<u32>,
    ) -> SutBuilder<sut_builder::SetDefaultField<S>>
    where
        S::DefaultField: sut_builder::IsUnset,
    {}
    /**| **Optional** |
| -- |

**See also** a companion setter that accepts an `Option`: [`maybe_optional_field_with_specific_overrides()`](Self::maybe_optional_field_with_specific_overrides).

*/
    /// Docs on some_fn
    /// Multiline.
    #[allow(clippy::inline_always, clippy::impl_trait_in_params)]
    #[inline(always)]
    pub(in some_fn_overridden) fn optional_field_with_specific_overrides(
        self,
        value: u32,
    ) -> SutBuilder<sut_builder::SetOptionalFieldWithSpecificOverrides<S>>
    where
        S::OptionalFieldWithSpecificOverrides: sut_builder::IsUnset,
    {}
    /**| **Optional** |
| -- |

**See also** a companion setter that wraps the value with `Some` internally: [`optional_field_with_specific_overrides()`](Self::optional_field_with_specific_overrides).

*/
    /// Docs on option_fn
    /// Multiline.
    #[allow(clippy::inline_always, clippy::impl_trait_in_params)]
    #[inline(always)]
    pub(in option_fn_overridden) fn maybe_optional_field_with_specific_overrides(
        mut self,
        value: Option<u32>,
    ) -> SutBuilder<sut_builder::SetOptionalFieldWithSpecificOverrides<S>>
    where
        S::OptionalFieldWithSpecificOverrides: sut_builder::IsUnset,
    {}
    /**| **Optional** |
| -- |

**See also** a companion setter that accepts an `Option`: [`maybe_default_field_with_specific_overrides()`](Self::maybe_default_field_with_specific_overrides).

**Default:** ```2 + 2 * 3```.

*/
    /// Docs on some_fn
    /// Multiline.
    #[allow(clippy::inline_always, clippy::impl_trait_in_params)]
    #[inline(always)]
    pub(in some_fn_overridden) fn default_field_with_specific_overrides(
        self,
        value: u32,
    ) -> SutBuilder<sut_builder::SetDefaultFieldWithSpecificOverrides<S>>
    where
        S::DefaultFieldWithSpecificOverrides: sut_builder::IsUnset,
    {}
    /**| **Optional** |
| -- |

**See also** a companion setter that wraps the value with `Some` internally: [`default_field_with_specific_overrides()`](Self::default_field_with_specific_overrides).

**Default:** ```2 + 2 * 3```.

*/
    /// Docs on option_fn
    /// Multiline.
    #[allow(clippy::inline_always, clippy::impl_trait_in_params)]
    #[inline(always)]
    pub(in option_fn_overridden) fn maybe_default_field_with_specific_overrides(
        mut self,
        value: Option<u32>,
    ) -> SutBuilder<sut_builder::SetDefaultFieldWithSpecificOverrides<S>>
    where
        S::DefaultFieldWithSpecificOverrides: sut_builder::IsUnset,
    {}
    /**| **Optional** |
| -- |

**See also** a companion setter that accepts an `Option`: [`maybe_optional_field_with_inherited_overrides()`](Self::maybe_optional_field_with_inherited_overrides).

*/
    /// Common docs
    /// Multiline.
    #[allow(clippy::inline_always, clippy::impl_trait_in_params)]
    #[inline(always)]
    pub(in overridden) fn optional_field_with_inherited_overrides(
        self,
        value: u32,
    ) -> SutBuilder<sut_builder::SetOptionalFieldWithInheritedOverrides<S>>
    where
        S::OptionalFieldWithInheritedOverrides: sut_builder::IsUnset,
    {}
    /**| **Optional** |
| -- |

**See also** a companion setter that wraps the value with `Some` internally: [`optional_field_with_inherited_overrides()`](Self::optional_field_with_inherited_overrides).

*/
    /// Docs on option_fn
    /// Multiline.
    #[allow(clippy::inline_always, clippy::impl_trait_in_params)]
    #[inline(always)]
    pub(in option_fn_overridden) fn maybe_optional_field_with_inherited_overrides(
        mut self,
        value: Option<u32>,
    ) -> SutBuilder<sut_builder::SetOptionalFieldWithInheritedOverrides<S>>
    where
        S::OptionalFieldWithInheritedOverrides: sut_builder::IsUnset,
    {}
    /**| **Optional** |
| -- |

**See also** a companion setter that accepts an `Option`: [`maybe_default_field_with_inherited_overrides()`](Self::maybe_default_field_with_inherited_overrides).

**Default:** ```2 + 2 * 3```.

*/
    /// Common docs
    /// Multiline.
    #[allow(clippy::inline_always, clippy::impl_trait_in_params)]
    #[inline(always)]
    pub(in overridden) fn default_field_with_inherited_overrides(
        self,
        value: u32,
    ) -> SutBuilder<sut_builder::SetDefaultFieldWithInheritedOverrides<S>>
    where
        S::DefaultFieldWithInheritedOverrides: sut_builder::IsUnset,
    {}
    /**| **Optional** |
| -- |

**See also** a companion setter that wraps the value with `Some` internally: [`default_field_with_inherited_overrides()`](Self::default_field_with_inherited_overrides).

**Default:** ```2 + 2 * 3```.

*/
    /// Docs on option_fn
    /// Multiline.
    #[allow(clippy::inline_always, clippy::impl_trait_in_params)]
    #[inline(always)]
    pub(in option_fn_overridden) fn maybe_default_field_with_inherited_overrides(
        mut self,
        value: Option<u32>,
    ) -> SutBuilder<sut_builder::SetDefaultFieldWithInheritedOverrides<S>>
    where
        S::DefaultFieldWithInheritedOverrides: sut_builder::IsUnset,
    {}
}
