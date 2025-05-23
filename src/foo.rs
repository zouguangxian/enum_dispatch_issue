use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Hello {
    fn hello(&self) -> String;
}

pub struct HelloA;
pub struct HelloB;

impl Hello for HelloA {
    fn hello(&self) -> String {
        "HelloA".to_string()
    }
}

impl Hello for HelloB {
    fn hello(&self) -> String {
        "HelloB".to_string()
    }
}

macro_rules! hello_set {
    ($enum_name:ident, $($alias:ident: $struct:ty),+) => {
        #[allow(non_camel_case_types)]
        #[repr(u8)]
        #[enum_dispatch(Hello)]
        pub enum $enum_name {
            $($alias($struct)),+
        }
    };
}

hello_set!(Foo, A: HelloA, B: HelloB);
