////////////////////////////////////////////////////////
/// DNA, in very basic terms, consists of 'sequences' of 'nucleo bases'.
/// It's possible to read these with certain technology, not relevant here.
/// Commonly called a 'reading', is a collection of nucleo base sequence readings.
/// Each base in the sequence are accompanied by a value determining the 'quality' of the reading.
/// The 'quality score' is proportional with the confidence of a correct reading.
/// 


enum NucleoBase {
    Cytocine,   //= C
    Guanine,    //= G
    Adenine,    //= A
    Thymine,    //= T
}
pub struct Sequence { bases: Vec<NucleoBase> }
impl Sequence {
    pub fn new(nucleo_base_sequence: &str) -> Result<Sequence,String> {

        let mut bases = Vec::<NucleoBase>::new();
        for char in nucleo_base_sequence.chars() {
            bases.push(
                match char {
                    'C' => NucleoBase::Cytocine { },
                    'G' => NucleoBase::Guanine { },
                    'A' => NucleoBase::Adenine { },
                    'T' => NucleoBase::Thymine { },
                    _ => return Err(format!("Error parsing sequence: {char} is not a valid nucleo base.")),
                }
            );
        }
        Ok(Sequence { bases: bases })
    }
}

fn try_convert_sequence_vector(sequences: Vec<String>) -> Result<Vec<Sequence>,String> {
    let mut parsed = Vec::<Sequence>::new();
    for seq in sequences {
        match Sequence::new(&seq) {
            Ok(s) => parsed.push(s),
            Err(e) => return Err(e),
        }
    };
    Ok(parsed)
}

/// a Genome Reading can be either valid or not. As this is a property of any reading, we define it as a trait, in that way, any structure resembling a genome reading, can implement an expression for this.
pub trait GenomeReading {
    fn validate(&self) -> bool;
}

pub struct GenomeRead {
    pub id: String,
    pub seq: Vec<Sequence>,
    pub qual: Vec<String>
}

impl TryFrom<Read> for GenomeRead {
    fn try_from(read:Read) -> Result<Self, String> {
        Ok(GenomeRead { 
            id: read.id, 
            seq: try_convert_sequence_vector(read.seq).unwrap(),
            qual: read.qual,
        })
    }
    
    type Error = String;
}

#[derive(Clone,PartialEq,Debug)]
pub struct Read {
    pub id: String,
    pub seq: Vec<String>,
    pub qual: Vec<String>
}
impl Read {
    fn valid_read_lengths(&self) -> bool {
        self.seq.len() == self.qual.len() 
        && (0..self.seq.len()).all(|index| {
            self.seq[index].chars().count() == self.qual[index].chars().count()
        })
    }
    pub fn new(id: String) -> Self {
        Self { 
            id: id, 
            seq: Vec::<String>::new(), 
            qual: Vec::<String>::new()
        }
    }
    pub fn add_sequence(&mut self, seq: String) {
        self.seq.push(seq);
    }
    pub fn add_quality(&mut self, qual: String) {
        self.qual.push(qual);
    }
}

impl GenomeReading for Read {
    /// Validates a genome reading.
    fn validate(&self) -> bool {
        // The reading needs an equal amount of sequences and matching qualities
        if !self.valid_read_lengths() {
            return false;
        }
        // ToDo: validate other requirements
        //  - Sequence consists of nucleobases
        //  - The length of each sequence matches its corresponding quality
        for sequence in 0..self.seq.len() {
            if self.seq[sequence].chars().count() != self.qual[sequence].chars().count() {
                return false;
            }
        }

        return true;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    // use test_case::test_case;
 
    #[test]
    fn nucleo_base_from_string() {
        assert!(Sequence::new("GGGTGATGGCCGCTGCCGATGGCGTC").is_ok());
        assert!(Sequence::new("GGGTGXXGGCCGCTGCCGATGGCGTC").is_err());
    }

    #[test]
    fn vec_nucleo_bases_from_strings() {
        // this is supposed to succeed
        assert!(try_convert_sequence_vector(vec![
            String::from("GGGTGATGGCCGCTGCCGATGGCGTC"),
            String::from("AAATCCCACC"),
            String::from("AAATGATGGCCGCTGCCGATGGCGTC"),
            String::from("AAATCCCACC"),
        ]).is_ok());
        // this is supposed to fail
        assert!(try_convert_sequence_vector(vec![
            String::from("GGGTGATGGCCGCTGCCGATGGCGTC"),
            String::from("AAATCCCACC"),
            String::from("AAATCXCACC"), // fails because of the X in this line!
        ]).is_err());
    }
}