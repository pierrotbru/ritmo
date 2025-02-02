pub struct people {
	name: String,
	nationality: Option<String>,
	birth_year: Option<String>
}

pub struct Books {
	title: String,
	orig_title: Option<String>,
	publisher: Option<String>,
	format: Option<String>,
	pub_date: Option<i64>,
	acq_date: Option<i64>,
	lastmod_date: Option<i64>,
	series: Option<String>,
	series_index: Option<u32>,
	notes: Option<String>,
	contents: Vec<Contents>,
}