/// Used for providing better IDE hints (completions and syntax highlighting).
pub mod ide;

/// Used to implement the `alloc` feature.
#[cfg(feature = "alloc")]
pub extern crate alloc;

/// Marker trait to denote the state of the member that is not set yet.
#[rustversion::attr(
    since(1.78.0),
    diagnostic::on_unimplemented(
        message = "can't set the same member twice",
        label = "this member was already set"
    )
)]
pub trait IsUnset {}

/// The sole implementation of the [`IsUnset`] trait.
#[derive(Debug)]
pub struct Unset;

impl IsUnset for Unset {}

/// A trait used to transition optional members to the [`Set`] state.
///
/// It also provides a better error message when the member is not set.
/// The `Member` generic parameter isn't used by the trait implementation,
/// it's used only as a label with the name of the member to specify which one
/// was not set.
#[rustversion::attr(
    since(1.78.0),
    diagnostic::on_unimplemented(
        message = "can't finish building yet; the member `{Member}` was not set",
        label = "the member `{Member}` was not set"
    )
)]
pub trait IntoSet<T, Member> {
    fn into_set(self) -> Set<T>;
}

impl<T, Member> IntoSet<T, Member> for Set<T> {
    fn into_set(self) -> Self {
        self
    }
}

impl<T, Member> IntoSet<Option<T>, Member> for Unset {
    fn into_set(self) -> Set<Option<T>> {
        Set(None)
    }
}

/// This is all a big embarrassing workaround, please don't oversee 😳😳😳.
///
/// Anyway, if you you are curious what the hell is going on here, then here is
/// an explanation 😸. So... where to start 🤔. Ah! The problem!
///
/// ## The problem
///
/// Proc macro attributes (like `#[builder]`) see all the `#[cfg(...)]` and `#[cfg_attr(...)]`
/// attributes unexpanded. For example, if you write smth like this:
///
/// ```
/// #[bon::builder]
/// fn func(
///     #[cfg(windows)]
///     windows_only_param: u32,
/// ) {}
///
/// ```
///
/// then the `#[builder]` macro will see the full `#[cfg(...)]` attribute with
/// the `windows_only_param` it is attached to verbatim. The `#[cfg(...)]` isn't
/// removed by the time the `#[builder]`'s macro expansion is invoked.
///
/// It is a problem because the `#[builder]` macro needs to know the exact list
/// of members it has to generate setters for. It doesn't know whether the
/// the `windows` predicate evaluates to `true` or `false`, especially if this was
/// a more complex predicate. So it can't decide whether to generate a setter for
/// the `windows_only_param` or not.
///
/// ## The solution
///
/// This macro allows us to evaluate the `cfg` predicates by using a variation of
/// [the trick] shared by @recatek.
///
/// When the `#[builder]` macro finds any usage of `#[cfg(...)]` or `#[cfg_attr(...)]`
/// it generates a call to this macro with all `cfg` predicates collected from the
/// item it was placed on. The `#[builder]` macro deduplicates and sorts the `cfg`
/// predicates and passes them as `$pred` to this macro.
///
/// This macro then dispatches to `__eval_cfg_callback_true` or `__eval_cfg_callback_false`
/// by defining a conditional `use ...` statement for each predicate and collects the
/// results of the evaluation in the `$results` list.
///
/// For the last call to this macro (when no more `$pred` are left) the macro calls back
/// to the proc macro attribute that called it with the results of the evaluation and
/// the original parameters and item which are passed through via the `$rest` macro variable.
///
/// [the trick]: https://users.rust-lang.org/t/supporting-or-evaluating-cfg-in-proc-macro-parameters/93240/2
#[macro_export]
#[doc(hidden)]
macro_rules! __eval_cfg_callback {
    (
        { $($results:tt)* }
        ( $pred_id:ident: $($pred:tt)* )
        $($rest:tt)*
    ) => {
        // The `pred_id` is required to be a unique identifier for the current
        // predicate evaluation so that we can use in a `use` statement to define
        // a new unique name for the macro to call.
        #[cfg($($pred)*)]
        use $crate::__eval_cfg_callback_true as $pred_id;

        #[cfg(not($($pred)*))]
        use $crate::__eval_cfg_callback_false as $pred_id;

        // The trick here is that `$pred_id` now resolves either to
        // `__eval_cfg_callback_true` or `__eval_cfg_callback_false`
        // depending on the evaluation of the cfg predicate, so by
        // invoking it as a macro, that macro internally pushes either
        // `true` or `false` to the `$results` list.

        $pred_id! {
            { $($results)* }
            $($rest)*
        }
    };

    // The terminal case for the recursion when there are no more predicates left.
    // We have collected all the results of the cfg evaluations and now we can
    // delegate them to the proc macro attribute that called this macro.
    (
        // The results of the cfg evaluation
        { $($results:tt)* }

        // The proc macro attribute to invoke with the results
        $final_macro:path,

        // Parameters to pass to the proc macro attribute after the cfg results
        ( $($macro_params:tt)* )

        // The item to attach the proc macro attribute to
        $($item:tt)*
    ) => {
        // The special `__cfgs(...)` prefix is parsed by the proc macro attribute
        // to get the results of the cfg evaluations.
        #[$final_macro(__cfgs($($results)*) $($macro_params)*)]
        $($item)*
    };
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Set<T>(pub T);

/// The `cfg` predicated evaluated to `true`, now push that information into
/// the `$results` list.
#[macro_export]
#[doc(hidden)]
macro_rules! __eval_cfg_callback_true {
    (
        { $($results:tt)* }
        $($tt:tt)*
    ) => {
        $crate::__eval_cfg_callback! {
            { $($results)* true, }
            $($tt)*
        }
    };
}

/// The `cfg` predicated evaluated to `false`, now push that information into
/// the `$results` list.
#[macro_export]
#[doc(hidden)]
macro_rules! __eval_cfg_callback_false {
    (
        { $($results:tt)* }
        $($tt:tt)*
    ) => {
        $crate::__eval_cfg_callback! {
            { $($results)* false, }
            $($tt)*
        }
    };
}

#[doc(hidden)]
#[deprecated(note = "\
    #[bon::builder] on top of a struct is deprecated; \
    use `#[derive(bon::Builder)]` instead")]
pub mod builder_attribute_on_a_struct {}
