macro_rules! use_mod (
    ($name: ident) => {
        mod $name; pub use $name::*;
    }
);

macro_rules! try_or (
    ($expr: expr, $fail: expr) => {
        match $expr {
            Some(x) => {x},
            None => {$fail},
        }
    }
);

//macro_rules! copyable_newtype (
//    ($name: ident : $type: ty) => {
//        #[derive(Copy,Clone,Debug,Serialize,DieselNewType)]
//        pub struct $name(pub $type);
//    };
//);
//
//macro_rules! newtype (
//    ($name: ident : $type: ty) => {
//        #[derive(Clone,Debug,Serialize,DieselNewType)]
//        pub struct $name(pub $type);
//    }
//);
