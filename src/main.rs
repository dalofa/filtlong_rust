mod reader_fastq;
use reader_fastq::fastq_readfile;
mod genome;

fn main() {
    match fastq_readfile("example_read_2.fq") {
        Ok(some_read) => {
            println!("Read the following:\n");
            for reading in some_read.readings() {
                println!(" id:{} \n\t seq:{:?} \n\t qual:{:?}", 
                    reading.id,
                    reading.seq,
                    reading.qual,
                );
            }
        },
        Err(message) => panic!("{}", message)
    }
}