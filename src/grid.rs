use crate::{model::{Evaluatable, CellAddress, Primitive}, lexer::lex, parser::parse, environment::Environment};
use std::collections::HashMap;

pub struct Grid {
    map: HashMap<CellAddress, Result<Primitive, String>>,
} 

impl Grid {
    pub fn new() -> Grid {
        Grid { map: HashMap::new(), }
    }
    
    pub fn set_cell(&mut self, adr: &CellAddress, val: Result<Primitive, String>) {
       self.map.insert(*adr, val);
    }

    pub fn get_cell(&self, adr: &CellAddress) -> Option<&Result<Primitive, String>> {
        self.map.get(&adr)
    }
}

pub struct TextGrid {
    grid: Grid,
    map: HashMap<CellAddress, String>,
    dimensions: (usize, usize),
}

impl TextGrid {
    pub fn new(dimensions: (usize, usize)) -> TextGrid {
        TextGrid {
            grid: Grid::new(),
            map: HashMap::new(),
            dimensions,
        }
    }

    pub fn get_cell_text(&self, adr: CellAddress) -> Option<&String> {
        self.map.get(&adr)
    }
    
    pub fn set_cell_text(&mut self, adr: CellAddress, str: String) {
        self.map.insert(adr, str);
        self.update_cells();
    }

    fn update_cells(&mut self) {
        for i in 0..self.dimensions.0 {
            for j in 0..self.dimensions.1 {
                self.evaluate_cell(CellAddress(i as i32, j as i32));
            }
        }
    }

    pub fn get_cell_value(&self, adr: CellAddress) -> Option<&Result<Primitive, String>> {
        self.grid.get_cell(&adr)
    }

    fn evaluate_cell(&mut self, adr: CellAddress) {
        let result = self.map.get(&adr);
        match result {
            Some(text) => {
                if text.len() != 0 && text.chars().next().unwrap() == '=' {
                    let new_text = &text[1..text.len()].chars().fold("".to_string(), |mut acc, character| { acc.push(character); acc });
                    self.grid.set_cell(&adr, evaluate_from_string(&new_text, &self.grid));
                } else if text.parse::<i32>().is_ok() {
                    self.grid.set_cell(&adr, Ok(Primitive::Integer(text.parse::<i32>().unwrap())));
                } else if text.parse::<bool>().is_ok() {
                    self.grid.set_cell(&adr, Ok(Primitive::Boolean(text.parse::<bool>().unwrap())));
                } else if text.parse::<f64>().is_ok() {
                    self.grid.set_cell(&adr, Ok(Primitive::Float(text.parse::<f32>().unwrap())));
                } else if !text.is_empty() {
                    self.grid.set_cell(&adr, Ok(Primitive::String(text.to_string())));
                } else {
                    self.grid.map.remove(&adr);
                }
            }
            None => (),
        }
    }

    // fn get_grid(&self) -> &Grid {
    //     &self.grid
    // }

    pub fn get_all_cell_values(&self) -> Vec<(&CellAddress, &Result<Primitive, String>)> {
        self.grid.map.iter().collect()
    }
}

pub fn evaluate_from_string(str: &String, grid: &Grid) -> Result<Primitive, String> {
    match lex(str) {
        Ok(tokens) => {
            match parse(tokens) {
                Ok(expression) => {
                    expression.evaluate(&Environment::new(grid))
                }
                Err(string) => Err(string),
            }
        }
        Err(string) => Err(string),
    }
}
