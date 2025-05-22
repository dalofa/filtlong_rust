use std::fs::read_to_string;
use super::genome::{GenomeReading, Read};

enum GenomeReaderState {
    Id, Sequence, Quality
}
impl GenomeReaderState {
    fn next_state(&self) -> GenomeReaderState {
        match &self {
            GenomeReaderState::Id => GenomeReaderState::Sequence{},
            GenomeReaderState::Sequence => GenomeReaderState::Quality {},
            GenomeReaderState::Quality => GenomeReaderState::Id {},
        }
    }
}

// Assumes:
//  1 read per file
//  doesn't validate anything at this point ...
pub fn fastq_parse(fastq_content: String) -> Result<Read,String> {
    let mut read: Read = Read {
        id: String::from(""),
        seq: Vec::<String>::new(),
        qual: Vec::<String>::new(),
    };

    let mut state = GenomeReaderState::Id {};
    let mut sequences_read: u32 = 0;

    // Read file line for line
    for line in fastq_content.lines() {
        match &state {
            GenomeReaderState::Id => {
                read.id = line.to_owned();
                state = state.next_state();
            },
            GenomeReaderState::Sequence => {
                if line != "+" {
                    read.seq.push(line.to_owned());
                    sequences_read += 1;
                } else {
                    state = state.next_state();
                }
            },
            GenomeReaderState::Quality => {
                read.qual.push(line.to_owned());
                sequences_read -= 1;
                if sequences_read == 0 {
                    state = state.next_state();
                }
            }
        }
    }
    match read.validate() {
        true => Ok(read),
        false => Err(String::from("Could not validate the genome :(")),
    }
}


pub fn fastq_readfile(file_path: &str) -> Result<Read,String> {
    fastq_parse(read_to_string(file_path).unwrap())
}

#[cfg(test)]
mod tests {
    use super::{fastq_parse, fastq_readfile};
    use test_case::test_case;

    #[test_case("example_read.fq")]
    #[test_case("example_read_2.fq")]
    fn reading_fastq_file(file_path: &str) {
        let read = fastq_readfile(file_path);
        assert!(read.is_ok());
    }

    #[test]
    fn incorrect_fastq_file_fails() {
        let invalid_fastq = String::from("
            @071112_SLXA-EAS1_s_7:5:1:817:345
            GGGTGATGGCCGCTGCCGATGGCGTC
            AAATCCCACC
            +
            IIIIIIIIIIIIIIIIIIIIIIIIII
            IIII9IG9ICA
        ");
        let read = fastq_parse(invalid_fastq);
        assert!(read.is_err());
    }
}