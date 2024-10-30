# `#[bon]`

This is a companion macro for [`builder`]. You should place it on top of the `impl` block
where you want to define methods with the [`builder`] macro. It provides
the necessary context to the [`builder`] macros on top of the functions
inside of the `impl` block. You'll get compile errors without that context.

For the examples of the usage of this macro and the reason why it's needed, see this paragraph in [the overview](../guide/overview#builder-for-an-associated-method).

[`builder`]: ./builder
