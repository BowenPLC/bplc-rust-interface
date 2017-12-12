pub mod bplc {
    pub mod program_iface {
        #[repr(C)]
        pub struct RawIfaceV1 {
            pub set_digital_output: usize,
        }

        #[repr(C)]
        pub struct IfaceV1 {
            pub set_digital_output: fn (id: usize, state: bool)
        }

        impl IfaceV1 {
            pub fn new(raw: &RawIfaceV1) -> IfaceV1 {
                let set_digital_output: fn (id: usize, state: bool);
                unsafe {
                    set_digital_output = ::std::mem::transmute::<usize, fn (usize, bool)>(raw.set_digital_output);
                }

                IfaceV1 {
                    set_digital_output
                }
            }
        }
    }

    pub mod core_iface {
        pub trait CoreV1 {
            fn set_digital_output(&mut self, id: usize, state: bool);
            fn get_digital_io(&mut self, id: usize) -> bool;
        }
    }

    #[cfg(test)]
    mod test {
        use bplc::program_iface::*;
        use bplc::core_iface::*;

        struct FakeCore { }

        impl CoreV1 for FakeCore {
            fn set_digital_output(&mut self, id: usize, state: bool) {
              assert_eq!(id, 2);
                assert_eq!(state, false);
            }

            fn get_digital_io(&mut self, _: usize) -> bool {
                return true;
            }
        }

        static mut FAKE_CORE: FakeCore = FakeCore { };

        fn set_digital_output(id: usize, state: bool) {
            unsafe {
                FAKE_CORE.set_digital_output(id, state);
            }
        }

        #[test]
        fn test_constructor() {
            let raw = RawIfaceV1 {
                set_digital_output: set_digital_output as fn(usize, bool) as usize
            };

            let concrete = IfaceV1::new(&raw);
            (concrete.set_digital_output)(2, false);
        }
    }
}
