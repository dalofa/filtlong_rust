use std::fs::read_to_string;
use super::genome::{GenomeReading, Read};

enum GenomeReaderState {
    Id, Sequence, Quality
}
impl GenomeReaderState {
    fn next_state(&self) -> GenomeReaderState {
        match &self {
            GenomeReaderState::Id => GenomeReaderState::Sequence,
            GenomeReaderState::Sequence => GenomeReaderState::Quality,
            GenomeReaderState::Quality => GenomeReaderState::Id,
        }
    }
}

#[derive(Debug)]
pub struct Readings {
    pub reads: Vec<Read>,
}
impl Readings {
    pub fn add_record(&mut self, reading: &Read) -> Result<(),String> {
        match reading.validate() {
            true => {
                // Clone does hurt performance, don't know when it'll hurt tho ...
                self.reads.push(reading.clone());
                return Ok(());
            },
            false => Err(String::from("Could not validate the genome reading :(")),
        }
    }
    pub fn readings(&self) -> &Vec<Read> { &self.reads }
    
    pub fn validate_records(&self) -> Result<(),String> {
        match (self.reads.iter().all(|r| { r.validate() }), self.reads.is_empty()) {
            (_, true) => Err(String::from("No records")),
            (false, false) => Err(String::from("One or more records invalid")),
            (true, false) => Ok(()),
        }
    }
}

static WRONG_STATE_ENCOUNTERED: &str = "Expected current read to be valid! But found 'None'";

// Assumes:
//  1 read per file
//  doesn't validate anything at this point ...
pub fn fastq_parse(fastq_content: String) -> Result<Readings,String> {

    let mut reads = Readings { reads: Vec::new() };
    let mut read: Option<Read> = None;

    let mut reader_state = GenomeReaderState::Id;
    let mut sequences_read: u32 = 0;

    // Read file line for line
    for line in fastq_content.lines() {
        match &reader_state {
            GenomeReaderState::Id => {
                // Begin a new reading, with the line as the id.
                read = Some(Read::new(line.to_owned()));
                // Cycle reader to next "finite state machine"-state 
                reader_state = reader_state.next_state();
            },
            GenomeReaderState::Sequence => {
                if line != "+" {                    
                    match read {
                        Some(ref mut reading) => {
                            reading.add_sequence(line.to_owned());
                        },
                        None => return Err(String::from(WRONG_STATE_ENCOUNTERED)),
                    };
                    sequences_read += 1;
                } else {
                    reader_state = reader_state.next_state();
                }
            },
            GenomeReaderState::Quality => {
                match read {
                    Some(ref mut reading) => reading.add_quality(line.to_owned()),
                    None => return Err(String::from(WRONG_STATE_ENCOUNTERED)),
                };
                sequences_read -= 1;
                if sequences_read == 0 {
                    // We reached the end of the quality lines, thus saving the record
                    if let Some(reading) = &read {
                        if let Err(e) = reads.add_record(reading) {
                            return Err(e);
                        }
                    }
                    reader_state = reader_state.next_state();
                };
            }
        }
    }
    match reads.validate_records() {
        Ok(()) => Ok(reads),
        Err(error) => Err(format!("Failed to verify records ... {}", error)),
    }
}


pub fn fastq_readfile(file_path: &str) -> Result<Readings,String> {
    match read_to_string(file_path) {
        Ok(file_content) => fastq_parse(file_content),
        Err(_) => Err(String::from("Could not read the file.")),
    }   
}

#[cfg(test)]
mod tests {
    use super::{fastq_parse, fastq_readfile};
    use test_case::test_case;

    #[test_case("example_read.fq")]
    /// note that: a fastq file, can contain several records.
    /// Each 'record' consists of a sequence ID and 1 or more sequence readings and a matching quality score.
    #[test_case("example_read_2.fq")]
    fn reading_fastq_file(file_path: &str) {
        let read = fastq_readfile(file_path);
        assert!(
            read.is_ok(),
            "test failed error: {:?}",
            read
        );
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
        // N.B. the sequence reading is invalid because the:
        //      length of the reading: "AAATCCCACC"
        //      does not match the quality reading: "IIII9IG9ICA"
        let read = fastq_parse(invalid_fastq);
        assert!(
            read.is_err(), 
            "Test didn't fail, but was expected to ... {:?}",
            read
        );
    }
}