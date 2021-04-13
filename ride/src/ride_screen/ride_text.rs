use std::fs;
use std::fs::File;
use std::io::Read;

use regex::Regex;

pub enum SearchDirection {
Backward,
Forward,
}

pub enum CommentType {
LineComment,
BlockComment,
}

pub struct RideText {
	current_line_number: usize,
	current_character_offset: usize,
	current_indentation_level: usize,
	lines: Vec<Line>,
	file_path: Option<String>,
	selection_mark: Option<usize>,
	}

impl RideText {
	
	pub fn new() -> RideText {
		RideText {current_line_number: 0, current_character_offset: 0, current_indentation_level: 0, lines: vec![Line::new(0, vec!['\n'])], file_path: None, selection_mark: None}
		}
	pub fn load(&mut self, text: String) -> Result<(), String>
		{
		let mut lines: Vec<Line>=text.lines().map(|i| Line::new(0, i.chars().collect::<Vec<char>>())).collect();
		
		if lines.len()==0 {
			lines.push(Line::new(0, vec![]));
			}
		
		for l in lines.iter_mut() {
			l.text.push('\n');
			}
		
		self.file_path=None;
		self.lines=lines;
		self.lines=RideText::parse_indentation(&self.lines)?;
		
		Ok(())
		}
	
	pub fn load_from_file(&mut self, file_path: &str) -> Result<(), String> {
	let mut text=String::new();
	let mut f=match File::open(file_path) {
	Ok(file) => file,
	Err(message) => return Err(message.to_string()),
	};
	
	if let Err(message) = f.read_to_string(&mut text) {
	return Err(message.to_string());
	}
	
	let result=self.load(text);
	self.file_path=Some(file_path.to_string());
	
	result
	}
	
	pub fn save(&self) -> Result<(), std::io::Error> {
	if let Some(file_path) = &self.file_path {
	fs::write(file_path, self.render_text(0, self.lines.len()))?;
	}
	
	Ok(())
	}
	
	pub fn render_text(&self, beginning_line: usize, ending_line: usize, ) -> String {
	let mut result="".to_string();
	
	for l in self.lines[beginning_line..ending_line].iter() {
	let mut prefix=String::new();
	if l.text.len()!=1 {
	for _ in 0..l.indentation_level {
	prefix.push_str("    ");
	}
	}
	
	let line=prefix+&l.text.iter().collect::<String>().trim()+"\n";
	result+=&line;
	}
	
	result
	}
	
	pub fn navigate_to_previous_line(&mut self) -> bool
		{
		
		if self.lines[self.current_line_number].indentation_level < self.current_indentation_level {
			return false;
			}
		
		for i in (0..self.current_line_number).rev()
			{
			if self.lines[i].indentation_level <= self.current_indentation_level
				{
				//Suitable line found, move the cared and return the result
				
				self.current_line_number=i;
				self.current_character_offset=0;
				
				return true;
				}
			}
		
		false
		}
	pub fn navigate_to_next_line(&mut self) -> bool
		{
		for i in self.current_line_number+1..self.lines.len()
			{
			if self.lines[i].indentation_level == self.current_indentation_level
				{
				//Suitable line found, move the caret and return the result
				
				self.current_line_number=i;
				self.current_character_offset=0;
				
				return true;
				}
			else if self.lines[i].indentation_level < self.current_indentation_level {
				return false;
				}
			}
		
		false
		}
	pub fn navigate_to_previous_character(&mut self) -> bool
		{
		if self.current_character_offset>0 {
			self.current_character_offset-=1;
			return true;
			}
		
		if self.navigate_to_previous_line() {
			self.current_character_offset=self.lines[self.current_line_number].text.len()-1;
			return true;
			}
		
		false
		}
	pub fn navigate_to_next_character(&mut self) -> bool
		{
		if self.current_character_offset<self.lines[self.current_line_number].text.len()-1 {
			self.current_character_offset+=1;
			return true;
			}
		
		self.navigate_to_next_line()
		}
	pub fn navigate_to_area_beginning(&mut self)
		{
		while self.navigate_to_previous_line() {}
		}
	pub fn navigate_to_area_ending(&mut self)
		{
		while self.navigate_to_next_line() {}
		}
	
	pub fn navigate_to_line_beginning(&mut self) {
	self.current_character_offset=0;
	}
	
	pub fn navigate_to_line_ending(&mut self) {
	self.current_character_offset=self.lines[self.current_line_number].text.len()-1;
	}
	
	pub fn increase_indentation_level(&mut self) -> bool
		{
		if self.current_indentation_level!=self.lines[self.current_line_number].indentation_level {
			return false;
			}
		
		if self.line_has_subblock(self.current_line_number) {
			self.current_indentation_level+=1;
			return true;
			}
		
		false
		}
	pub fn decrease_indentation_level(&mut self) -> bool
		{
		if self.current_indentation_level==0 {
			return false;
			}
		
		self.navigate_to_area_beginning();
		
		if self.lines[self.current_line_number].indentation_level<self.current_indentation_level {
			self.current_indentation_level-=1;
			return true;
			}
		
		false
		}

pub fn jump_to_line(&mut self, line_number: usize) -> Result<bool, String> {
if line_number==0 {
return Err("Line numbering starts from 1.".to_string());
}

if line_number>self.lines.len() {
return Err(format!("Line number {} is invalid, there are just {} lines.", line_number, self.lines.len()));
}

let current_indentation_level=self.current_indentation_level;
self.current_line_number=line_number-1;
self.current_character_offset=0;
self.current_indentation_level=self.lines[self.current_line_number].indentation_level;

return Ok(current_indentation_level!=self.current_indentation_level);
}

	pub fn insert(&mut self, character: char)
		{
		self.lines[self.current_line_number].text.insert(self.current_character_offset, character);
		self.current_character_offset+=1;
		}
	
	pub fn get_current_line(&self) -> String {
		self.lines[self.current_line_number].text.iter().collect()
		}
	
	pub fn get_current_character(&self) -> char
		{
		self.lines[self.current_line_number].text[self.current_character_offset]
		}
	
	pub fn create_new_line(&mut self)
		{
		if self.current_character_offset==self.lines[self.current_line_number].text.len()-1 {
			let new_line_number=if self.current_indentation_level==self.lines[self.current_line_number].indentation_level {
			self.get_subblock_finishing_line_number(self.current_line_number)+1
			} else {
			self.current_line_number+1
			};
			self.lines.insert(new_line_number, Line::new(self.current_indentation_level, vec!['\n']));
			self.current_line_number=new_line_number;
			self.current_character_offset=0;
			}
		else
			{
			let mut new_line=Line::new(self.lines[self.current_line_number].indentation_level, self.lines[self.current_line_number].text.drain(0..self.current_character_offset).collect::<Vec<char>>());
			new_line.text.push('\n');
			self.lines.insert(self.current_line_number, new_line);
			
			self.current_line_number+=1;
			self.current_character_offset=0;
			}
		}
	pub fn create_new_block(&mut self)
		{
		let new_line=Line::new(self.lines[self.current_line_number].indentation_level+1, self.lines[self.current_line_number].text.drain(self.current_character_offset..).collect::<Vec<char>>());
		self.lines[self.current_line_number].text.push('\n');
		self.lines.insert(self.current_line_number+1, new_line);
		self.current_line_number+=1;
		self.current_character_offset=0;
		self.current_indentation_level=self.lines[self.current_line_number].indentation_level;
		}
	pub fn delete_character(&mut self) -> Option<char>
		{
		if self.current_character_offset>0
			{
			let result=self.lines[self.current_line_number].text[self.current_character_offset-1];
			self.lines[self.current_line_number].text.remove(self.current_character_offset-1);
			self.current_character_offset-=1;
			
			return Some(result);
			}
		else {
			let original_line_number=self.current_line_number;
			
			if self.line_has_subblock(self.current_line_number) || !self.navigate_to_previous_line()
				{
				return None;
				}
			
			self.current_character_offset=self.lines[self.current_line_number].text.len()-1;
			self.lines[self.current_line_number].text.remove(self.current_character_offset);
			let mut original_line=self.lines.remove(original_line_number).text;
			self.lines[self.current_line_number].text.append(&mut original_line);
			
			return Some('\n');
			}
		}
	pub fn delete(&mut self)
		{
		let starting_line_number=self.current_line_number;
		let finishing_line_number=self.get_subblock_finishing_line_number(self.current_line_number);
		self.navigate_to_previous_line();
		self.lines.drain(starting_line_number..finishing_line_number+1);
		
		if self.lines.len()==0 {
			self.lines.push(Line::new(0, vec!['\n']));
			}
		}
	
	pub fn get_selected_text(&mut self, cut: bool) -> Result<String, String> {
		let selection_mark=match self.selection_mark {
		Some(line) => line,
		None => self.current_line_number,
		};
		
		if self.lines[selection_mark].indentation_level<self.current_indentation_level && cut {
		return Err("Invalid selection for cutting".to_string());
		}
		
		let (selection_beginning, selection_ending) = if selection_mark<=self.current_line_number {
		(selection_mark, self.get_subblock_finishing_line_number(self.current_line_number))
		} else {
		(self.current_line_number, self.get_subblock_finishing_line_number(selection_mark))
		};
		
		let result=self.render_text(selection_beginning, selection_ending+1).trim_end().to_string();
		if cut {
		self.current_line_number=selection_beginning;
		self.navigate_to_previous_line();
		self.lines.drain(selection_beginning..selection_ending+1);
		if self.lines.len()==0 {
		self.lines.push(Line::new(0, vec!['\n']));
		}
		}
		
		Ok(result)
		}
	
	pub fn paste(&mut self, text: &str) -> Result<(), String> {
		let text=if !text.trim_end().contains("\n") {
		text.trim_start()
		}
		else {
		text
		};
		let chars: Vec<char>=text.chars().collect();
		if !text.contains('\n') {
		for i in 0..chars.len() {
		self.lines[self.current_line_number].text.insert(self.current_character_offset+i, chars[i]);
		}
		self.current_character_offset+=chars.len();
		return Ok(());
		}
		
		let mut lines: Vec<Line>=text.replace("\r", "").split('\n').into_iter().map(|i| Line::new(0, i.chars().collect::<Vec<char>>())).collect();
		lines.iter_mut().for_each(|i| i.text.push('\n'));
		 lines=RideText::parse_indentation(&lines)?;

		let insertion_index=if self.current_indentation_level<self.lines[self.current_line_number].indentation_level {
		self.current_line_number+1
		} else {
		self.get_subblock_finishing_line_number(self.current_line_number)+1
		};
		
		for (i, mut line) in lines.into_iter().enumerate() {
		line.indentation_level+=self.current_indentation_level;
		self.lines.insert(insertion_index+i, line);
		}
		
		Ok(())
		}
	
	pub fn start_selection(&mut self) {
		if self.selection_mark==None {
		self.selection_mark=Some(self.current_line_number);
		}
		}
	
	pub fn cancel_selection(&mut self) {
		if self.selection_mark!=None {
		self.selection_mark=None;
		}
		}
	fn parse_indentation(lines: &Vec<Line>) -> Result<Vec<Line>, String>
		{
		let mut lines: Vec<Line>=Vec::clone(lines);
		if lines.len()==0 {
			return Ok(lines);
			}
		
		let mut indentation_steps: Vec<usize>=vec![RideText::get_indentation_level(&lines[0].text)];
		let mut current_universal_indentation_level=0;
		let mut previous_indentation_level=indentation_steps[0];
		let comment_level=0; // Will be mutable in future, the mark is removed for now to shut up the compiler.
		let mut lines_adjustment_data: Vec<bool>=vec![false; lines.len()];
		
		for (line_number, line) in lines.iter_mut().enumerate() {
			let current_indentation_level=RideText::get_indentation_level(&line.text);
			if current_indentation_level<indentation_steps[0] && !(current_indentation_level==0 && line.text.len()==1) {
				return Err(format!("Indentation corrupted, code must start with the smallest indentation. Problem occurred on line {}.", line_number).to_string());
				}
				
				let line_text: String=line.text.iter().collect();
				if (current_indentation_level==0 && line.text.len()==1) || comment_level>0 || line_text.trim().starts_with("//") || line_text.trim().starts_with("#") || line_text.trim().starts_with(";") || line_text.trim().starts_with("<<<<<<<") || line_text.trim().starts_with(">>>>>>>") || line_text.starts_with("=======") || line_text.trim().starts_with("/*") || line_text.trim().starts_with("\"\"\"") {
				lines_adjustment_data[line_number]=true;
				continue;
				}
				
			line.text.drain(0..current_indentation_level);
			
			if current_indentation_level>previous_indentation_level {
				indentation_steps.push(current_indentation_level);
				current_universal_indentation_level+=1;
				}
			else if current_indentation_level<previous_indentation_level {
				let mut found=false;
				for i in (0..indentation_steps.len()).rev() {
					if current_indentation_level==indentation_steps[i] {
						current_universal_indentation_level=i;
						indentation_steps.drain(i+1..indentation_steps.len());
						found=true;
						}
					}
				
				if !found {
					return Err(format!("Invalid indentation on line {}.", line_number));
					}
				}
			
			line.indentation_level=current_universal_indentation_level;
			previous_indentation_level=current_indentation_level;
			}
		
		//Deal with the empty lines by filling them from backward
		
		previous_indentation_level=lines[lines.len()-1].indentation_level;
		for (line_number, line) in lines.iter_mut().enumerate().rev() {
			if lines_adjustment_data[line_number] && previous_indentation_level>0 {
				line.indentation_level=previous_indentation_level;
				}
			
			previous_indentation_level=line.indentation_level;
			}
		
		Ok(lines)
		}
	
	fn get_indentation_level(line: &Vec<char>) -> usize
		{
		for i in 0..line.len() {
			if line[i]!=' ' && line[i]!='\t' {
				return i;
				}
			}
		
		0
		}
	fn get_subblock_finishing_line_number(&self, starting_line_number: usize) -> usize
		{
		let starting_line_indentation_level=self.lines[starting_line_number].indentation_level;
		
		for l in starting_line_number+1..self.lines.len() {
			if self.lines[l].indentation_level<=starting_line_indentation_level {
				return l-1;
				}
			}
		
		self.lines.len()-1
		}
	fn line_has_subblock(&self, line_number: usize) -> bool
		{
		if line_number>=self.lines.len()-1 {
			return false;
			}
		
		self.lines[line_number+1].indentation_level>self.lines[line_number].indentation_level
		}
	
	fn search_on_line(&self, line_number: usize, starting_position: usize, search_term: &Vec<char>, search_direction: SearchDirection) -> Option<usize> {
		
		if search_term.len()==0 || self.lines[line_number].text.len()==0 {
		return None;
		}
		
		let mut match_streak=0; //The number of matched characters from the search term.
		let desired_match_streak=search_term.len(); //To make things somewhat nicer.
		let current_line_text=&self.lines[line_number].text;
		let mut position=starting_position;
		
		match search_direction {
		SearchDirection::Backward => {
		
		loop {
		if current_line_text[position]==search_term[search_term.len()-1-match_streak] {
		match_streak+=1;
		}
		else if match_streak>0 {
		position+=match_streak-1;
		match_streak=0;
		continue;
		}
		
		if match_streak==desired_match_streak {
		return Some(position);
		}
		
		if position==0 {
		break;
		}
		
		position-=1;
		}
		},
		SearchDirection::Forward => {
		while position<current_line_text.len() {
		if current_line_text[position]==search_term[match_streak] {
		match_streak+=1;
		}
		else if match_streak>0 {
		position-=match_streak-1;
		match_streak=0;
		continue;
		}
		
		if match_streak==desired_match_streak {
		return Some((position-(match_streak-1)) as usize);
		}
		
		position+=1;
		}
		},
		};
		
		None
		
		}
	
	pub fn find(&mut self, search_term: &str, search_direction: SearchDirection) -> bool {
		
		let search_term: Vec<char>=search_term.chars().collect();
		
		match search_direction {
		SearchDirection::Backward => {
		if let Some(position) = self.search_on_line(self.current_line_number, self.current_character_offset, &search_term, SearchDirection::Backward) {
		self.current_character_offset=position;
		return true;
		}
		
		for line_number in (0..self.current_line_number).rev() {
		let character_offset: usize=if self.lines[line_number].text.len()>0 {
		self.lines[line_number].text.len()-1
		}
		else {
		0
		};
		if let Some(position) = self.search_on_line(line_number, character_offset, &search_term, SearchDirection::Backward) {
		self.current_line_number=line_number;
		self.current_character_offset=position;
		return true;
		}
		}
		
		},
		SearchDirection::Forward => {
		if let Some(position) = self.search_on_line(self.current_line_number, self.current_character_offset+1, &search_term, SearchDirection::Forward) {
		self.current_character_offset=position;
		return true;
		}
		
		for line_number in (self.current_line_number+1)..self.lines.len() {
		let character_offset: usize=0;
		
		if let Some(position) = self.search_on_line(line_number, character_offset, &search_term, SearchDirection::Forward) {
		self.current_line_number=line_number;
		self.current_character_offset=position;
		return true;
		}
		}
		
		},
		};
		
		false
		}
	
	pub fn reformat(&mut self, beginning_mark: &str, ending_mark: &str) -> Result<(), String> {
		if beginning_mark=="" || ending_mark=="" {
		return Err("Error: marks can't be empty".to_string());
		}
		
		let search_regex=Regex::new(&format!("(\\{})|(\\{})|(//)|(#)|(/\\*)|(\\*/)|(\")|(')", beginning_mark, ending_mark)).unwrap();
		let mut indentation_level: i32=0;
		let mut in_quotes: Option<String>=None;
		let mut in_comment: Option<CommentType>=None;
		
		for line in &mut self.lines {
		let line_text=line.text.iter().collect::<String>();
		
		let mut indentation_delta: i32=0;
		
		for m in search_regex.find_iter(&line_text) {
		match m.as_str() {
		"\"" | "'" => {
		if in_comment.is_none() {
		if let Some(quote)=&in_quotes {
		if quote==m.as_str() {
		in_quotes=None;
		}
		}
		else {
		in_quotes=Some(m.as_str().to_string());
		}
		}
		},
		"//" | "#" | "/*" | "*/" => {
		if in_quotes.is_none() {
		if let Some(comment_type)=&in_comment {
		if let CommentType::BlockComment=comment_type {
		if m.as_str()=="*/" {
		in_comment=None;
		}
		}
		}
		else {
		in_comment=match m.as_str() {
		"/*" | "*/" => Some(CommentType::BlockComment),
		_ => Some(CommentType::LineComment),
		};
		}
		}
		},
		b_mark if b_mark==beginning_mark => {
		indentation_delta+=1;
		},
		e_mark if e_mark==ending_mark => {
		indentation_delta-=1;
		},
		_ => {},
		}
		
		}
		
		if indentation_delta>0 && line_text.starts_with(beginning_mark) {
		indentation_level+=indentation_delta;
		line.indentation_level=if indentation_level>=0 {
		indentation_level as usize
		}
		else {
		0
		};
		}
		else {
		line.indentation_level=if indentation_level>=0 {
		indentation_level as usize
		}
		else {
		0
		};
		
		indentation_level+=indentation_delta;
		
		}
		}
		
		Ok(())
		
		}
	
	pub fn file_path(&self) -> &Option<String> {
	&self.file_path
	}
	
	}
#[derive(Clone, Debug)]
struct Line {
	indentation_level: usize,
	text: Vec<char>,
	}
impl Line {
	
	pub fn new(indentation_level: usize, text: Vec<char>) -> Line
		{
		Line {indentation_level, text}
		}
	}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
		}
	}
