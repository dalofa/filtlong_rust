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
    let mut sequences_read: u32 = 0;
    let mut new_reading: bool = true;

    for (index, line) in read_to_string(file_path).unwrap().lines().enumerate() {
        if sequences_read == 0 && new_reading {
            read.id = line.to_owned();
            new_reading = false;
            reading_sequences= true;
        } else {
            println!("{sequences_read}");
            if line == "+" {
                reading_sequences = false;
                continue;
            }
            if reading_sequences {
                read.seq.push(line.to_owned());
                sequences_read += 1;
            } else {
                read.qual.push(line.to_owned());
                sequences_read-=1;
            }

            if sequences_read == 0 {
                new_reading = true;
            }
        }
    }
    return read;
}


fn main() {
    let some_read = read_fastq("example_read_2.fq");
    println!("Read the following:\n id:{} \n\t seq:{:?} \n\t qual:{:?}", 
        some_read.id,
        some_read.seq,
        some_read.qual,
    );
}
