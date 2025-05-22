struct Seq {
    bases: Vec<NucleoBase>
}

enum NucleoBase {
    Cytocine, Guanine, Adenine, Thymine
}
// type C = NucleoBase::Cytocine;
// type G = NucleoBase::Guanine;
// type A = NucleoBase::Adenine;
// type T = NucleoBase::Thymine;

pub trait GenomeReading {
    fn validate(&self) -> bool;
}

pub struct Read {
    pub id: String,
    pub seq: Vec<String>,
    pub qual: Vec<String>
}

impl GenomeReading for Read {
    /// Validates a genome reading.
    fn validate(&self) -> bool {
        // The reading needs an equal amount of sequences and matching qualities
        if self.seq.len() != self.qual.len() {
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