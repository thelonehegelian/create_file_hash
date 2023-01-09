// use data_encoding::HEXUPPER;
use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read, Write};
fn main() {
    // create sha256 hex
    fn create_hash<R: Read>(mut file: R) -> Result<Digest, std::io::Error> {
        // create a context for sha256 computation
        let mut context = Context::new(&SHA256);
        // create a buffer -> !note: don't need the bufReader here as file type is a bufReader
        // let mut reader = BufReader::new(file);
        let mut buffer = [0; 1024];
        loop {
            let bytes_read = file.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                break;
            }
            let slice = &buffer[..bytes_read];
            // uses an array of slices file the file is read and computes the SHA
            context.update(slice);
        }
        Ok(context.finish())
    }
    // create a new file and write data to it
    let path = "file.txt";
    let input = File::open(path).unwrap();
    let mut file = File::create("file.txt").unwrap();

    let _ = write!(
        &mut file,
        "I’m Nobody! Who are you?
    Are you – Nobody – too?
    Then there’s a pair of us!
    Don't tell! they'd advertise – you know!
    
    How dreary – to be – Somebody!
    How public – like a Frog –
    To tell one’s name – the livelong June –
    To an admiring Bog!"
    );
    // create hash function takes in a Reader as an argument
    let reader = BufReader::new(input);
    let hash = create_hash(reader).unwrap();
    println!(
        "THIS IS THE SHA256 OF THE GIVEN TEXT: {:?}",
        HEXUPPER.encode(hash.as_ref())
    );
}
