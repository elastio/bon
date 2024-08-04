use bon::builder;
use std::hint::black_box;

pub fn regular_bench() -> u32 {
    let (arg1, arg2, arg3, arg4, arg5) = black_box(("4", 24, true, Some("5".to_string()), Some(6)));

    regular(arg1, arg2, arg3, arg4, arg5)
}

pub fn builder_bench() -> u32 {
    let (arg1, arg2, arg3, arg4, arg5) = black_box(("4", 24, true, Some("5".to_string()), Some(6)));

    let this = builder()
        .arg1(arg1)
        .arg2(arg2)
        .arg3(arg3)
        .maybe_arg4(arg4)
        .maybe_arg5(arg5);

    this.call()
}

// #[builder(expose_positional_fn = regular)]
// fn builder(
//     arg1: &str,
//     arg2: u32,
//     arg3: bool,
//     arg4: Option<String>,
//     arg5: Option<u32>,
// ) -> u32 {
//     let x = arg1.parse::<u32>().unwrap() + arg2;
//     let x = x + u32::from(arg3);
//     let x = x + arg4.map(|x| x.parse::<u32>().unwrap()).unwrap_or(0);
//     let x = x + arg5.unwrap_or(0);
//     x
// }

// Recursive expansion of builder macro
// =====================================

#[inline(always)]
fn builder<'__f0>() -> BuilderBuilder<'__f0> {
    BuilderBuilder {
        __private_impl: __BuilderBuilderPrivateImpl {
            _phantom: ::std::marker::PhantomData,
            arg1: bon::private::Required(::std::marker::PhantomData),
            arg2: bon::private::Required(::std::marker::PhantomData),
            arg3: bon::private::Required(::std::marker::PhantomData),
            arg4: bon::private::Optional(::std::marker::PhantomData),
            arg5: bon::private::Optional(::std::marker::PhantomData),
        },
    }
}
#[doc(hidden)]
trait __BuilderBuilderState {
    type Arg1;
    type Arg2;
    type Arg3;
    type Arg4;
    type Arg5;
}
impl<Arg1, Arg2, Arg3, Arg4, Arg5> __BuilderBuilderState for (Arg1, Arg2, Arg3, Arg4, Arg5) {
    type Arg1 = Arg1;
    type Arg2 = Arg2;
    type Arg3 = Arg3;
    type Arg4 = Arg4;
    type Arg5 = Arg5;
}
#[must_use = "the builder does nothing until you call `call()` on it to finish building"]
struct BuilderBuilder<
    '__f0,
    __State: __BuilderBuilderState = (
        bon::private::Required<&'__f0 str>,
        bon::private::Required<u32>,
        bon::private::Required<bool>,
        bon::private::Optional<String>,
        bon::private::Optional<u32>,
    ),
> {
    #[doc = r" Please don't touch this field. It's an implementation"]
    #[doc = r" detail that is exempt from the API stability guarantees."]
    #[doc = r" It's visible to you only because of the limitations of"]
    #[doc = r" the Rust language."]
    #[doc = r""]
    #[doc = r" The limitation is that we can't make the fields of the"]
    #[doc = r" generated struct private other than by placing its"]
    #[doc = r" declaration inside of a nested submodule. However, we"]
    #[doc = r" can't do that because this breaks support for fn items"]
    #[doc = r" declared inside of other fn items like this:"]
    #[doc = r""]
    #[doc = r" ```rustdoc_hidden"]
    #[doc = r" use bon::builder;"]
    #[doc = r""]
    #[doc = r" fn foo() {"]
    #[doc = r"     struct Foo;"]
    #[doc = r""]
    #[doc = r"     #[builder]"]
    #[doc = r"     fn nested(foo: Foo) {}"]
    #[doc = r""]
    #[doc = r"     nested().foo(Foo).call();"]
    #[doc = r" }"]
    #[doc = r" ```"]
    #[doc = r""]
    #[doc = r" If we were to generate a child module like this then code"]
    #[doc = r" in that child module would lose access to the symbol `Foo`"]
    #[doc = r" in the parent module. The following code doesn't compile."]
    #[doc = r""]
    #[doc = r" ```rustdoc_hidden"]
    #[doc = r" fn foo() {"]
    #[doc = r"     struct Foo;"]
    #[doc = r""]
    #[doc = r"     mod __private_child_module {"]
    #[doc = r"         use super::*;"]
    #[doc = r""]
    #[doc = r"         pub(super) struct Builder {"]
    #[doc = r"             foo: Foo,"]
    #[doc = r"         }"]
    #[doc = r"     }"]
    #[doc = r" }"]
    #[doc = r" ```"]
    #[doc = r""]
    #[doc = r" `Foo` symbol is inaccessible inside of `__private_child_module`"]
    #[doc = r" because it is defined inside of the function `foo()` and not"]
    #[doc = r" inside of the parent module."]
    #[doc = r""]
    #[doc = r#" Child modules are kinda implicitly "hoisted" to the top-level of"#]
    #[doc = r" the module and they can't see the local symbols defined inside"]
    #[doc = r" of the same function scope."]
    __private_impl: __BuilderBuilderPrivateImpl<'__f0, __State>,
}
#[doc = r" This struct exists only to reduce the number of private fields"]
#[doc = r" that pop up in IDE completions for developers. It groups all"]
#[doc = r" the private fields in it leaving the builder type higher with"]
#[doc = r" just a single field of this type that documents the fact that"]
#[doc = r" the developers shouldn't touch it."]
struct __BuilderBuilderPrivateImpl<'__f0, __State: __BuilderBuilderState> {
    _phantom:
        ::std::marker::PhantomData<(&'__f0 str, u32, bool, Option<String>, Option<u32>, __State)>,
    arg1: __State::Arg1,
    arg2: __State::Arg2,
    arg3: __State::Arg3,
    arg4: __State::Arg4,
    arg5: __State::Arg5,
}
impl<'__f0, __State: __BuilderBuilderState> BuilderBuilder<'__f0, __State>
where
    __State::Arg1: ::std::convert::Into<bon::private::Set<&'__f0 str>>,
    __State::Arg2: ::std::convert::Into<bon::private::Set<u32>>,
    __State::Arg3: ::std::convert::Into<bon::private::Set<bool>>,
    __State::Arg4: ::std::convert::Into<bon::private::Set<Option<String>>>,
    __State::Arg5: ::std::convert::Into<bon::private::Set<Option<u32>>>,
{
    #[doc = r" Finishes building and performs the requested action."]
    #[inline(always)]
    fn call(self) -> u32 {
        regular(
            ::std::convert::Into::<bon::private::Set<_>>::into(self.__private_impl.arg1).0,
            ::std::convert::Into::<bon::private::Set<_>>::into(self.__private_impl.arg2).0,
            ::std::convert::Into::<bon::private::Set<_>>::into(self.__private_impl.arg3).0,
            ::std::convert::Into::<bon::private::Set<_>>::into(self.__private_impl.arg4).0,
            ::std::convert::Into::<bon::private::Set<_>>::into(self.__private_impl.arg5).0,
        )
    }
}
#[allow(type_alias_bounds)]
#[doc(hidden)]
type __BuilderBuilderSetArg1<'__f0, __State: __BuilderBuilderState> = BuilderBuilder<
    '__f0,
    (
        bon::private::Set<&'__f0 str>,
        __State::Arg2,
        __State::Arg3,
        __State::Arg4,
        __State::Arg5,
    ),
>;
impl<'__f0, __State: __BuilderBuilderState<Arg1 = bon::private::Required<&'__f0 str>>>
    BuilderBuilder<'__f0, __State>
{
    #[inline(always)]
    fn arg1(self, value: &'__f0 str) -> __BuilderBuilderSetArg1<'__f0, __State> {
        BuilderBuilder {
            __private_impl: __BuilderBuilderPrivateImpl {
                _phantom: ::std::marker::PhantomData,
                arg1: bon::private::Set(value),
                arg2: self.__private_impl.arg2,
                arg3: self.__private_impl.arg3,
                arg4: self.__private_impl.arg4,
                arg5: self.__private_impl.arg5,
            },
        }
    }
}
#[allow(type_alias_bounds)]
#[doc(hidden)]
type __BuilderBuilderSetArg2<'__f0, __State: __BuilderBuilderState> = BuilderBuilder<
    '__f0,
    (
        __State::Arg1,
        bon::private::Set<u32>,
        __State::Arg3,
        __State::Arg4,
        __State::Arg5,
    ),
>;
impl<'__f0, __State: __BuilderBuilderState<Arg2 = bon::private::Required<u32>>>
    BuilderBuilder<'__f0, __State>
{
    #[inline(always)]
    fn arg2(self, value: u32) -> __BuilderBuilderSetArg2<'__f0, __State> {
        BuilderBuilder {
            __private_impl: __BuilderBuilderPrivateImpl {
                _phantom: ::std::marker::PhantomData,
                arg1: self.__private_impl.arg1,
                arg2: bon::private::Set(value),
                arg3: self.__private_impl.arg3,
                arg4: self.__private_impl.arg4,
                arg5: self.__private_impl.arg5,
            },
        }
    }
}
#[allow(type_alias_bounds)]
#[doc(hidden)]
type __BuilderBuilderSetArg3<'__f0, __State: __BuilderBuilderState> = BuilderBuilder<
    '__f0,
    (
        __State::Arg1,
        __State::Arg2,
        bon::private::Set<bool>,
        __State::Arg4,
        __State::Arg5,
    ),
>;
impl<'__f0, __State: __BuilderBuilderState<Arg3 = bon::private::Required<bool>>>
    BuilderBuilder<'__f0, __State>
{
    #[inline(always)]
    fn arg3(self, value: bool) -> __BuilderBuilderSetArg3<'__f0, __State> {
        BuilderBuilder {
            __private_impl: __BuilderBuilderPrivateImpl {
                _phantom: ::std::marker::PhantomData,
                arg1: self.__private_impl.arg1,
                arg2: self.__private_impl.arg2,
                arg3: bon::private::Set(value),
                arg4: self.__private_impl.arg4,
                arg5: self.__private_impl.arg5,
            },
        }
    }
}
#[allow(type_alias_bounds)]
#[doc(hidden)]
type __BuilderBuilderSetArg4<'__f0, __State: __BuilderBuilderState> = BuilderBuilder<
    '__f0,
    (
        __State::Arg1,
        __State::Arg2,
        __State::Arg3,
        bon::private::Set<Option<String>>,
        __State::Arg5,
    ),
>;
impl<'__f0, __State: __BuilderBuilderState<Arg4 = bon::private::Optional<String>>>
    BuilderBuilder<'__f0, __State>
{
    #[doc = "Same as [`Self::arg4`], but accepts an `Option` as input. See that method's documentation for more details."]
    #[inline(always)]
    fn maybe_arg4(
        self,
        value: Option<impl Into<String>>,
    ) -> __BuilderBuilderSetArg4<'__f0, __State> {
        BuilderBuilder {
            __private_impl: __BuilderBuilderPrivateImpl {
                _phantom: ::std::marker::PhantomData,
                arg1: self.__private_impl.arg1,
                arg2: self.__private_impl.arg2,
                arg3: self.__private_impl.arg3,
                arg4: bon::private::Set(value.map(Into::into)),
                arg5: self.__private_impl.arg5,
            },
        }
    }
    #[inline(always)]
    fn arg4(self, value: impl Into<String>) -> __BuilderBuilderSetArg4<'__f0, __State> {
        BuilderBuilder {
            __private_impl: __BuilderBuilderPrivateImpl {
                _phantom: ::std::marker::PhantomData,
                arg1: self.__private_impl.arg1,
                arg2: self.__private_impl.arg2,
                arg3: self.__private_impl.arg3,
                arg4: bon::private::Set(Some(value.into())),
                arg5: self.__private_impl.arg5,
            },
        }
    }
}
#[allow(type_alias_bounds)]
#[doc(hidden)]
type __BuilderBuilderSetArg5<'__f0, __State: __BuilderBuilderState> = BuilderBuilder<
    '__f0,
    (
        __State::Arg1,
        __State::Arg2,
        __State::Arg3,
        __State::Arg4,
        bon::private::Set<Option<u32>>,
    ),
>;
impl<'__f0, __State: __BuilderBuilderState<Arg5 = bon::private::Optional<u32>>>
    BuilderBuilder<'__f0, __State>
{
    #[doc = "Same as [`Self::arg5`], but accepts an `Option` as input. See that method's documentation for more details."]
    #[inline(always)]
    fn maybe_arg5(self, value: Option<u32>) -> __BuilderBuilderSetArg5<'__f0, __State> {
        BuilderBuilder {
            __private_impl: __BuilderBuilderPrivateImpl {
                _phantom: ::std::marker::PhantomData,
                arg1: self.__private_impl.arg1,
                arg2: self.__private_impl.arg2,
                arg3: self.__private_impl.arg3,
                arg4: self.__private_impl.arg4,
                arg5: bon::private::Set(value),
            },
        }
    }
    #[inline(always)]
    fn arg5(self, value: u32) -> __BuilderBuilderSetArg5<'__f0, __State> {
        BuilderBuilder {
            __private_impl: __BuilderBuilderPrivateImpl {
                _phantom: ::std::marker::PhantomData,
                arg1: self.__private_impl.arg1,
                arg2: self.__private_impl.arg2,
                arg3: self.__private_impl.arg3,
                arg4: self.__private_impl.arg4,
                arg5: bon::private::Set(Some(value)),
            },
        }
    }
}
#[doc = "Positional function equivalent of [`builder()`].\nSee its docs for details."]
#[allow(clippy::too_many_arguments)]
fn regular(arg1: &str, arg2: u32, arg3: bool, arg4: Option<String>, arg5: Option<u32>) -> u32 {
    let x = arg1.parse::<u32>().unwrap() + arg2;
    let x = x + u32::from(arg3);
    let x = x + arg4.map(|x| x.parse::<u32>().unwrap()).unwrap_or(0);
    let x = x + arg5.unwrap_or(0);
    x
}
