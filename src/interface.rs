use ncurses::{*, ll::curs_set};
use std::cmp;

use crate::{grid::TextGrid, model::{CellAddress, Primitive}};

const CELL_WIDTH: i32 = 7;
const CELL_HEIGHT: i32 = 1;
const CELL_HORIZ_OFFSET: i32 = 3;
const CELL_VERT_OFFSET: i32 = 1;

pub struct Interface {
    grid_dimensions: (i32, i32),
    grid_window: WINDOW,
    editor_window: WINDOW,
    result_window: WINDOW,
    mode: Mode,
    text: String,
    grid_cursor: (i32, i32),
}

impl Interface {
    pub fn new(grid_dimensions: (i32, i32)) -> Self {
        initscr();
        refresh();
        noecho();
        // curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        keypad(stdscr(), true);

        let mut width: i32 = 0;
        let mut height: i32 = 0;
        getmaxyx(stdscr(), &mut height, &mut width);

        let grid_window = newwin(height - 4, width, 0, 0);
        let editor_window = newwin(2, width, height - 4, 0);
        let result_window = newwin(2, width, height - 2, 0);
        wrefresh(grid_window);
        wrefresh(editor_window);
        wrefresh(result_window);
        
        Self {
            grid_dimensions,
            grid_window,
            editor_window,
            result_window,
            mode: Mode::Grid,
            text: String::new(),
            grid_cursor: (0, 0),
        }
    }

    pub fn setup(&mut self) {
        let mut height = 0;
        let mut width = 0;
        getmaxyx(self.grid_window, &mut height, &mut width);

        self.draw_grid();

        wmove(self.editor_window, 0, 0);
        whline(self.editor_window, ACS_HLINE(), width);
        wrefresh(self.editor_window);
        
        wmove(self.result_window, 0, 0);
        whline(self.result_window, ACS_HLINE(), width);
        wrefresh(self.result_window);
        mv(self.grid_cursor.0 * (CELL_HEIGHT + 1) + CELL_VERT_OFFSET + 1, self.grid_cursor.1 * (CELL_WIDTH + 1) + CELL_HORIZ_OFFSET + 1);
    }

    fn draw_grid(&self) {
        let mut grid_window_height = 0;
        let mut grid_window_width = 0;
        getmaxyx(self.grid_window, &mut grid_window_height, &mut grid_window_width);

        let (num_grid_rows, num_grid_cols) = self.grid_dimensions;
        
        let grid_char_width = 1 + (1 + CELL_WIDTH) * num_grid_cols as i32;
        let grid_char_height = 1 + (1 + CELL_HEIGHT) * num_grid_rows as i32;

        wmove(self.grid_window, CELL_VERT_OFFSET, CELL_HORIZ_OFFSET);
        wvline(self.grid_window, ACS_VLINE(), grid_char_height);
        for col in 0..num_grid_cols as i32 {
            mvwaddstr(self.grid_window, 0, col as i32 * (CELL_WIDTH + 1) + 1 + CELL_HORIZ_OFFSET, &col.to_string()); 
            wmove(self.grid_window, CELL_VERT_OFFSET, (1 + CELL_WIDTH) * (col + 1) + CELL_HORIZ_OFFSET);
            wvline(self.grid_window, ACS_VLINE(), grid_char_height);
        }

        wmove(self.grid_window, CELL_VERT_OFFSET, CELL_HORIZ_OFFSET);
        whline(self.grid_window, ACS_HLINE(), grid_char_width);
        for row in 0..num_grid_rows as i32 {
            mvwaddstr(self.grid_window, row as i32 * (CELL_HEIGHT + 1) + 1 + CELL_VERT_OFFSET, 0, &row.to_string()); 
            wmove(self.grid_window, (1 + CELL_HEIGHT) * (row + 1) + CELL_VERT_OFFSET, CELL_HORIZ_OFFSET);
            whline(self.grid_window, ACS_HLINE(), grid_char_width);
        }

        wrefresh(self.grid_window);
    }

    pub fn update(&mut self, grid: &mut TextGrid) -> bool {
        let mut result = true;
        wclear(self.grid_window);
        self.update_grid(grid.get_all_cell_values());
        match self.mode {
            Mode::Grid => {
                wmove(self.grid_window, self.grid_cursor.0 * (CELL_HEIGHT + 1) + CELL_VERT_OFFSET + 1, self.grid_cursor.1 * (CELL_WIDTH + 1) + CELL_HORIZ_OFFSET + 1);
                let key = getch();
                if key == 'q' as i32 {
                    result = false;
                } else if key == '\n' as i32 {
                    wclear(self.grid_window);
                    wmove(self.editor_window, 1, 0);
                    waddstr(self.editor_window, &self.text);
                    wrefresh(self.editor_window);
                    self.mode = Mode::Editor;
                } else if key == KEY_UP {
                    if self.grid_cursor.0 == 0 {
                        self.grid_cursor.0 = self.grid_dimensions.0 - 1;
                    } else {
                        self.grid_cursor.0 = (self.grid_cursor.0 - 1) % self.grid_dimensions.0;
                    }
                    mv(self.grid_cursor.0 * (CELL_HEIGHT + 1) + CELL_VERT_OFFSET + 1, self.grid_cursor.1 * (CELL_WIDTH + 1) + CELL_HORIZ_OFFSET + 1);
                    self.text = match grid.get_cell_text(cursor_pos_to_cell_address(self.grid_cursor)) {
                        Some(val) => val.to_owned(),
                        None => "".to_string(),
                    };


                    wmove(self.editor_window, 1, 0);
                    wclrtoeol(self.editor_window);
                    waddstr(self.editor_window, &self.text);
                    wrefresh(self.editor_window);
                    self.set_result("");

                    if let Some(result) = grid.get_cell_value(cursor_pos_to_cell_address(self.grid_cursor)) {
                        match result {
                            Ok(val) => self.set_result(&val.to_string()),
                            Err(err) => self.set_result(&err),
                        }
                    }
               } else if key == KEY_DOWN {
                    self.grid_cursor.0 = (self.grid_cursor.0 + 1) % self.grid_dimensions.0;
                    mv(self.grid_cursor.0 * (CELL_HEIGHT + 1) + CELL_VERT_OFFSET + 1, self.grid_cursor.1 * (CELL_WIDTH + 1) + CELL_HORIZ_OFFSET + 1);
                    self.text = match grid.get_cell_text(cursor_pos_to_cell_address(self.grid_cursor)) {
                        Some(val) => val.to_owned(),
                        None => "".to_string(),
                    };
                    wmove(self.editor_window, 1, 0);
                    wclrtoeol(self.editor_window);
                    waddstr(self.editor_window, &self.text);
                    wrefresh(self.editor_window);
                    self.set_result("");
                    
                     if let Some(result) = grid.get_cell_value(cursor_pos_to_cell_address(self.grid_cursor)) {
                        match result {
                            Ok(val) => self.set_result(&val.to_string()),
                            Err(err) => self.set_result(&err),
                        }
                    }
                } else if key == KEY_LEFT {
                    if self.grid_cursor.1 == 0 {
                        self.grid_cursor.1 = self.grid_dimensions.1 - 1;
                    } else {
                        self.grid_cursor.1 = (self.grid_cursor.1 - 1) % self.grid_dimensions.1;
                    }
                    mv(self.grid_cursor.0 * (CELL_HEIGHT + 1) + CELL_VERT_OFFSET + 1, self.grid_cursor.1 * (CELL_WIDTH + 1) + CELL_HORIZ_OFFSET + 1);
                    self.text = match grid.get_cell_text(cursor_pos_to_cell_address(self.grid_cursor)) {
                        Some(val) => val.to_owned(),
                        None => "".to_string(),
                    };
                    wmove(self.editor_window, 1, 0);
                    wclrtoeol(self.editor_window);
                    waddstr(self.editor_window, &self.text);
                    wrefresh(self.editor_window);
                    self.set_result("");

                    if let Some(result) = grid.get_cell_value(cursor_pos_to_cell_address(self.grid_cursor)) {
                        match result {
                            Ok(val) => self.set_result(&val.to_string()),
                            Err(err) => self.set_result(&err),
                        }
                    }
 
                } else if key == KEY_RIGHT {
                    self.grid_cursor.1 = (self.grid_cursor.1 + 1) % self.grid_dimensions.1;
                    mv(self.grid_cursor.0 * (CELL_HEIGHT + 1) + CELL_VERT_OFFSET + 1, self.grid_cursor.1 * (CELL_WIDTH + 1) + CELL_HORIZ_OFFSET + 1);
                    self.text = match grid.get_cell_text(cursor_pos_to_cell_address(self.grid_cursor)) {
                        Some(val) => val.to_owned(),
                        None => "".to_string(),
                    };
                    wmove(self.editor_window, 1, 0);
                    wclrtoeol(self.editor_window);
                    waddstr(self.editor_window, &self.text);
                    wrefresh(self.editor_window);
                    self.set_result("");

                    if let Some(result) = grid.get_cell_value(cursor_pos_to_cell_address(self.grid_cursor)) {
                        match result {
                            Ok(val) => self.set_result(&val.to_string()),
                            Err(err) => self.set_result(&err),
                        }
                    }
                    
                } else {
                    mv(self.grid_cursor.0 * (CELL_HEIGHT + 1) + CELL_VERT_OFFSET + 1, self.grid_cursor.1 * (CELL_WIDTH + 1) + CELL_HORIZ_OFFSET + 1);
                }
            }
            Mode::Editor => {
                let mut curs_y = 0;
                let mut curs_x = 0;
                getyx(self.editor_window, &mut curs_y, &mut curs_x);
                
                wrefresh(self.editor_window);

                let key = getch();
                if key == KEY_ENTER || key == '\n' as i32 {
                    grid.set_cell_text(cursor_pos_to_cell_address(self.grid_cursor), self.text.to_owned());

                    self.set_result("");

                    if let Some(result) = grid.get_cell_value(cursor_pos_to_cell_address(self.grid_cursor)) {
                        match result {
                            Ok(val) => self.set_result(&val.to_string()),
                            Err(err) => self.set_result(&err),
                        }
                    }

                    wmove(self.grid_window, self.grid_cursor.0 * (CELL_HEIGHT + 1) + CELL_VERT_OFFSET + 1, self.grid_cursor.1 * (CELL_WIDTH + 1) + CELL_HORIZ_OFFSET + 1);
                    self.mode = Mode::Grid;
                } else if key == KEY_BACKSPACE {
                    wmove(self.editor_window, 1, cmp::max(curs_x - 1, 0));
                    wdelch(self.editor_window);
                    if self.text.len() > 0 {
                        self.text = remove_nth_char(&self.text, curs_x as usize - 1);
                    }
                } else if key == KEY_LEFT {
                    wmove(self.editor_window, 1, cmp::max(curs_x - 1, 0));
                } else if key == KEY_RIGHT {
                    wmove(self.editor_window, 1, cmp::min(curs_x + 1, self.text.len() as i32));
                } else {
                    wmove(self.editor_window, 1, 0);
                    self.text = replace_nth_char(&self.text, curs_x as usize, char::from_u32(key as u32).unwrap());
                    waddstr(self.editor_window, &self.text);
                    wmove(self.editor_window, 1, curs_x + 1);
                    wrefresh(self.editor_window);
                }
            }
        } 

        result
    }

    fn set_result(&self, text: &str) {
        wmove(self.result_window, 1, 0);
        wclrtoeol(self.result_window);
        waddstr(self.result_window, text);
        wrefresh(self.result_window);
    }

    fn update_grid(&self, cell_values: Vec<(&CellAddress, &Result<Primitive, String>)>) {
        for cell in cell_values {
            if (cell.0.1, cell.0.0) == self.grid_cursor {
                wattron(self.grid_window, A_BOLD());
            }
            wmove(self.grid_window, cell.0.1 * (CELL_HEIGHT + 1) + CELL_VERT_OFFSET + 1, cell.0.0 * (CELL_WIDTH + 1) + CELL_HORIZ_OFFSET + 1);
            let _ = match cell.1 {
                Ok(val) => waddstr(self.grid_window, &format!("{0:.1$}", val.to_string(), CELL_WIDTH as usize)),
                Err(_) => waddstr(self.grid_window, "ERROR"),
            };
            if (cell.0.1, cell.0.0) == self.grid_cursor {
                wattroff(self.grid_window, A_BOLD());
            }
        }

        self.draw_grid();
        if let Mode::Grid = self.mode {
            wmove(self.grid_window, self.grid_cursor.0 * (CELL_HEIGHT + 1) + CELL_VERT_OFFSET + 1, self.grid_cursor.1 * (CELL_WIDTH + 1) + CELL_HORIZ_OFFSET + 1);
        }
        wrefresh(self.grid_window);
    }
}

/// Taken from Stack Overflow
fn replace_nth_char(s: &str, idx: usize, newchar: char) -> String {
    if idx == s.len() {
        let mut str = String::from(s);
        str.push(newchar);
        str
    } else {
        s.chars().enumerate().fold("".to_string(), |mut s,(i,c)| { if i == idx { s.push(newchar); s.push(c) } else { s.push(c) }; s } )
    }
}

fn remove_nth_char(s: &str, idx: usize) -> String {
    s.chars().enumerate().filter(|(i,_)| *i != idx ).map(|(_,c)| c ).collect()
}

fn cursor_pos_to_cell_address(cursor_pos: (i32, i32)) -> CellAddress {
    CellAddress(cursor_pos.1, cursor_pos.0)
}


enum Mode {
    Grid, Editor
}

pub enum InterfaceRequest {
    None, Quit, LoadFromCell((usize, usize)), SaveToCell(String, (usize, usize))
}

pub enum Response {
    None, UpdatedCells(Vec<(CellAddress, Result<Primitive, String>)>), CellData(String, Primitive),
}

 
