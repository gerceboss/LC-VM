use std::{fs::File, io::Read, path::Path};

pub fn read_image(image: &str, memory: &mut Vec<u16>) -> bool {
    //println!("read {}", image);

    let path = Path::new(image);
    let mut file = File::open(path).expect("No such file exists.");

    //data is a Vec<u8> where the data is read and stored in the
    //form of bytes using `read_to_end()` method.
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("buffer offerflow");

    //chunks(2) method combines the elements as [[val0, val1], ...]
    //chunks returns an iterator over data vector.
    let mut iter = data.chunks(2);

    //The first element specifies the address in memory where program
    //should start. It is the general value 0x3000 or 12288 in rogue.obj
    let pc = iter.next().unwrap();

    //We are combining two bytes into a u16 word as that is how our memory
    //stores data. That is, word size of our memory is 16 bits.
    let mut pc: usize = ((pc[0] as u16) << 8 | pc[1] as u16) as usize;

    //We now store the rest of the program data into memory
    for elem in iter {
        memory[pc] = (elem[0] as u16) << 8 | elem[1] as u16;
        pc = pc + 1;
    }

    true
}
