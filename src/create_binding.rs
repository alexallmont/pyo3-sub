/// Macro to wrap creation of modules from bindings.rs
///
/// Each binding created requires a MODULE_NAME, a TYPE_NAME and a type T to
/// expose the underlying typed classes to python.
/// This approach might be considered compiler abuse as the implementation sets
/// the three required values and then pulls in private/bindings.rs which in
/// turn gets them from super::<name>. This is not pretty but it's the neatest
/// approach I could find to coerce templated types into a common suites of
/// module classes for PyO3; other approaches include strategy patterns and
/// using Any types, but this has this least duplication and strongest typing.
#[macro_export]
macro_rules! create_binding {
    (
        $binding_type:tt,
        $module_name:tt
    ) => {
        #[path="."]
        pub mod $module_name {
            // Module name, underlying type name and type
            static MODULE_NAME:&'static str = stringify!($module_name);
            static TYPE_NAME:&'static str = stringify!($binding_type);
            type T = $binding_type;

            // bindings accesses above values and types via super::
            mod bindings;
            pub use bindings::*;
        }
    };
}
