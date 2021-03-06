pub mod server;
pub use crate::server::*;

pub mod common;
pub use crate::common::*;

pub mod client;
pub use crate::client::*;

pub mod RiffClient;
pub use crate::RiffClient::*;

pub mod ext;
pub use crate::ext::*;



// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         //assert_eq!(2 + 2, 4);
//         let server = super::Server {
//             name: String::from("server1"),
//         };
//         assert_eq!(server.name, String::from("server1"));

//     }
// }
