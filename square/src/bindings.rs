use wit_bindgen::generate;

generate!({
    world: "squarer",
    path: "../wit/calculator.wit",
});

pub mod exports {
    pub mod docs {
        pub mod calculator {
            pub mod square {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_square_cabi<T: Guest>(arg0: i32) -> i32 {
                    let result0 = T::square(arg0 as u32);
                    _rt::as_i32(result0)
                }
                pub trait Guest {
                    fn square(a: u32) -> u32;
                }
                #[doc(hidden)]

                macro_rules! __export_docs_calculator_square_0_1_0_cabi{
        ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

          #[export_name = "docs:calculator/square@0.1.0#square"]
          unsafe extern "C" fn export_square(arg0: i32) -> i32 {
            $($path_to_types)*::_export_square_cabi::<$ty>(arg0)
          }
        };);
      }
                #[doc(hidden)]
                pub(crate) use __export_docs_calculator_square_0_1_0_cabi;
            }
        }
    }
}
mod _rt {

    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }

    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }

    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }

    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }

    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }

    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }

    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }

    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }

    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }

    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }

    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_square_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::docs::calculator::square::__export_docs_calculator_square_0_1_0_cabi!($ty with_types_in $($path_to_types_root)*::exports::docs::calculator::square);
  )
}
#[doc(inline)]
pub(crate) use __export_square_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.21.0:square:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 213] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07Z\x01A\x02\x01A\x02\x01\
B\x02\x01@\x02\x01ay\x01by\0y\x04\0\x03square\x01\0\x04\x01\x19docs:calculator/square@\
0.1.0\x05\0\x04\x01\x1bdocs:calculator/square@0.1.0\x04\0\x0b\x0b\x01\0\x05square\x03\
\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.201.0\x10wit-\
bindgen-rust\x060.21.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
