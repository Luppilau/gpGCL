// #[macro_use]
// extern crate paste;

// #[macro_export]
// macro_rules! unsupported {
//     ($($struct:ident),*) => {
//         struct SupportChecker {
//             errors: Vec<String>,
//         }

//         impl SupportChecker {
//             fn new() -> Self {
//                 Self { errors: vec![] }
//             }
//         }

//         impl Visit for SupportChecker {
//             paste! {
//                 $(
//                     fn [<visit_ $struct:snake>](&mut self, i: &$struct) {
//                         self.errors.push(concat!(
//                             stringify!($struct),
//                             " operation not supported"
//                         ).to_string());
//                     }
//                 )*
//             }
//         }
//     };
// }
