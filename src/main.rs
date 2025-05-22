mod reader_fastq;
use reader_fastq::fastq_readfile;
mod genome;

fn main() {
    match fastq_readfile("example_read_2.fq") {
        Ok(some_read) => {
            println!("Read the following:\n id:{} \n\t seq:{:?} \n\t qual:{:?}", 
                some_read.id,
                some_read.seq,
                some_read.qual,
            );
        },
        Err(message) => panic!("{}", message)
    }
}