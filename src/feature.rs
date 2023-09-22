
use std::fmt;
/// feature can parse one line of a 10x features table and compare that to other features tables.
/// It can then modifiy a features table and convert the entry back into a string

#[derive(Clone)]
pub struct Feature {
    pub name: String,
    pub name2: String,
    pub ty: String,
    pub chr: String,
    pub start: usize,
    pub end: usize,
    pub empty:bool,
}


impl Feature{

	// pub fn init() ->Self{
	// 	Self{
	// 		name : "".to_string(),
	// 		name2: "".to_string(),
	// 		ty   : "".to_string(),
	// 		chr  : "".to_string(),
	// 		start: 0,
	// 		end  : 0,
	// 		empty: true
	// 	}
	// }

	pub fn parse(dat:&str) ->Self{
		let data:Vec<&str> = dat.split('\t').collect();
		Self{
			name : data[0].to_string(),
			name2: data[1].to_string(),
			ty   : data[2].to_string(),
			chr  : data[3].to_string(),
			start: data[4].parse::<usize>().unwrap_or_default(),
			end  : data[5].parse::<usize>().unwrap_or_default(),
			empty: false
		}
	}
	
	fn overlaps(&self, other: &Feature ) -> bool{
		self.chr == other.chr && self.start < other.end && self.end > other.start
	}
	/// Use overlaps and if that is true adjusts the own position to cover the total area.
	pub fn overlapps_adjust( &mut self, other: &Feature ) -> bool{
		if self.overlaps (other){
			//println!("I {} am a match to\n  {}\n------------", self, other );
			
			if self.start > other.start{
				self.start = other.start;
			}
			if self.end < other.end{
				self.end = other.end
			}
			self.name = format!("{}:{}-{}", self.chr, self.start, self.end);
			self.name2 = self.name.clone();
			//println!("  {} <- should have changed?!\n--------------", self.name);
			return true
		}
		
		false
	}
	/// Checks if the self poition is located before the other object
	pub fn before(&self, other: &Self ) -> bool{
		if self.chr == other.chr{
			self.end < other.start 
		} else {
			self.chr.cmp(&other.chr) == std::cmp::Ordering::Less
		}
	}
	// pub fn before_all(&self, others: Vec<&Self> ) -> bool{
	// 	for other in others{
	// 		if ! self.before (other){
	// 			return false
	// 		}
	// 	}
	// 	true
	// }
}


impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}\t{}\t{}\t{}\t{}",
            self.name, self.name2, self.ty, self.chr, self.start, self.end
        )
    }
}


#[cfg(test)]
mod tests {

 	use crate::Feature;

    #[test]
    fn check_parse() {
        let line = "ENSG00000157933\tSKI\tGene Expression\tchr1\t2228318\t2228319";
        let data = Feature::parse(line);
        assert_eq!(data.name, "ENSG00000157933".to_string());
        assert_eq!(data.name2, "SKI".to_string());
        assert_eq!(data.ty, "Gene Expression".to_string());
        assert_eq!(data.chr, "chr1".to_string());
        assert_eq!(data.start, 2228318);
        assert_eq!(data.end, 2228319);
      	assert_eq!(format!("{}",data), "ENSG00000157933\tSKI\tGene Expression\tchr1\t2228318\t2228319");
    }


}
