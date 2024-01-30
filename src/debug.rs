use crate::{compiler::chunk::Chunk, flamebytecode::{FBOpCode, debug}};


pub fn debug_chunk(chunk: &Chunk) {
    println!("\n\n");

    println!("constant_pool:");
    for (i, constant) in chunk.consts.as_vm().iter().enumerate() {
        println!("\t[{i:#08X}]   {constant:?},")
    }
    println!("\n_________________________________________________\n");

    chunk.code.chunks(8).enumerate().for_each(|x| {
        print!("\t{:#010X}  |  ", x.0 * 8);
        x.1.into_iter().for_each(|op| print!("{op:08}  "));
        println!();
    });
    println!("\n");

    debug_code(&chunk.code);
    println!("\n\n");
}

pub fn debug_code(code: &Vec<u8>) {
    let mut i = 0;
    while i < code.len() {
        let by = code[i];
        let size = FBOpCode::size()[by as usize];
        debug(i as u64, &code[i as usize..i as usize + size as usize]);
        i += size as usize;
    }
}
