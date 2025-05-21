use std::fs::read_to_string;

struct Read {
    pub id: String,
    pub seq: Vec<String>,
    pub qual: Vec<String>
}

// Assumes:
//  1 read per file
//  doesn't validate anything at this point ...
fn read_fastq(file_path: &str) -> Read {
    let mut read: Read = Read {
        id: String::from(""),
        seq: Vec::<String>::new(),
        qual: Vec::<String>::new(),
    };
    let mut reading_sequences = true;
    for (index, line) in read_to_string(file_path).unwrap().lines().enumerate() {
        match index {
            0 => read.id = line.to_owned(),
            _ => {
                if line == "+" {
                    reading_sequences = false;
                    continue;
                }
                if reading_sequences {
                    read.seq.push(line.to_owned());
                } else {
                    read.qual.push(line.to_owned());
                }
            }
        }
    }
    return read;
}


fn main() {
    let some_read = read_fastq("src/example_read.fq");
    println!("Read the following:\n id:{} \n\t seq:{:?} \n\t qual:{:?}", 
        some_read.id,
        some_read.seq,
        some_read.qual,
    );
}
