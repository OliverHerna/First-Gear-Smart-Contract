//No requiere de un sistema operativo
#![no_std]
use gstd::{msg, prelude::*};

#[no_mangle]
//Podemos usar esye metodo fuera de RUST
//handle - metodo que simempre esta escuchando cuando el contrato se encientra en la blockchain
unsafe extern "C" fn handle() {
    msg::reply_bytes("PONG", 0).expect("Error in sending a reply message");
}