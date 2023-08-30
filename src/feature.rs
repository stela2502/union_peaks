


/// feature can parse one line of a 10x features table and compare that to other features tables.
/// It can then modifiy a features table and convert the entry back into a string

#[derive(Clone)]
pub struct Feature {
    name: String,
    name2: String,
    pub ty: String,
    chr: String,
    start: usize,
    end: usize,
    pub empty:bool,
}

impl Feature{

	pub fn init() ->Self{
		Self{
			name : "".to_string(),
			name2: "".to_string(),
			ty   : "".to_string(),
			chr  : "".to_string(),
			start: 0,
			end  : 0,
			empty: true
		}
	}

	pub fn parse(dat:&str) ->Self{
		let data:Vec<&str> = dat.split("\t").collect();
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
	pub fn overlaps(&self, other: &Feature ) -> bool{
		self.start < other.end && self.end > other.start
	}
	pub fn to_string(&self) -> String{
		format!(
            "{}\t{}\t{}\t{}\t{}\t{}",
            self.name, self.name2, self.ty, self.chr, self.start, self.end
        )
	}
}