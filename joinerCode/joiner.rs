use std::rand::random;
use std::os;
use std::io::File;

fn main() {
    let args: ~[~str] = os::args();
    if args.len() != 3 {
        println!("Usage: {:s} <splitFile1> <splitFile2>", args[0]); 
    } else {
        let f1name = args[1];
        let f2name = args[2];
        let path1 = Path::new(f1name.clone());
        let path2 = Path::new(f2name.clone());
        let msg_file1 = File::open(&path1);
        let msg_file2 = File::open(&path2);

        match (msg_file1, msg_file2) {
            Some(mut msg1, mut msg2) => {
                let msg1_bytes: ~[u8] = msg1.read_to_end();
                let msg2_bytes: ~[u8] = msg2.read_to_end();
                let output_file
                       = File::create(&Path::new("originalMsg"));
                
                match (output_file) {
                    (Some(output)) => { 
                        split(msg1_bytes, msg2_bytes, output_file); 
                        } ,
                    (_, _) => fail!("Error opening output files!"),
                }
            } ,
            None => fail!("Error opening message file: {:s}", fname)
        }
    }
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
    let mut ret = ~[];
    for i in range(0, a.len()) {
	ret.push(a[i] ^ b[i]);
    }
    ret
}

fn split(msg1_bytes: &[u8], mut msg2_bytes: &[u8], mut output_file: File) {
    let mut original_bytes: ~[u8] = xor(msg1_bytes, msg2_bytes);
    output_file.write(original_bytes);
}
