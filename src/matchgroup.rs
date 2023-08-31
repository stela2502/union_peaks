use crate::Feature;


use std::fmt;


pub struct MatchGroup {
    pub name: String,
    pub name2: String,
    pub ty: String,
    pub chr: String,
    pub start: usize,
    pub end: usize,
    pub empty:bool,
    pub targets: Vec<usize>,
    
}


impl MatchGroup{

	pub fn new( from: &Feature, id:usize ) -> Self{
		let mut targets = Vec::<usize>::with_capacity(100);
		targets.push(id);
		Self{
			name   : from.name.clone(),
			name2  : from.name2.clone(),
			ty     : from.ty.clone(),
			chr    : from.chr.clone(),
			end    : from.end,
			start  : from.start,
			empty: false,
			targets
		}
	}
	pub fn register_write_to( &mut self, id:usize ){
		self.targets.push( id );
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
			//println!("  {}\n--------------", self.name);
			return true
		}
		
		false
	}
	/// Checks if the self poition has is before the other object
	fn before(&self, other: &Self ) -> bool{
		if self.chr == other.chr{
			self.end < other.start 
		} else {
			self.chr.cmp(&other.chr) == std::cmp::Ordering::Less
		}
	}
	pub fn before_all(&self, others: Vec<&Self> ) -> bool{
		for other in others{
			if ! self.before (other){
				return false
			}
		}
		true
	}
}

impl fmt::Display for MatchGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}\t{}\t{}\t{}\t{}",
            self.name, self.name2, self.ty, self.chr, self.start, self.end
        )
    }
}