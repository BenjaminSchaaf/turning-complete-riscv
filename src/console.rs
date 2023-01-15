use crate::peripherals::CON;

pub struct Console {
    con: CON,
    row: usize,
    column: usize,
}

const ROWS: usize = 24;
const COLUMNS: usize = 80;
const END: usize = ROWS * COLUMNS;

impl Console {
    pub fn new(con: CON) -> Console {
        Console {
            con,
            row: 0,
            column: 0,
        }
    }

    pub fn next_line(&mut self) {
        self.row += 1;
        self.column = 0;

        if self.row == ROWS {
            let mut j = 0;
            for i in COLUMNS..END {
                self.con[j] = self.con[i];
                j += 1;
            }
            for i in END - COLUMNS..END {
                self.con[i] = 0;
            }
            self.row = ROWS - 1;
        }
    }

    pub fn erase(&mut self) {
        if self.column == 0 {
            if self.row > 0 {
                self.row -= 1;
                self.column = COLUMNS - 1;
            }
        } else {
            self.column -= 1;
        }

        self.con[self.row * COLUMNS + self.column] = 0;
    }
}

impl core::fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            self.write_char(c)?;
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        match c {
            '\n' => {
                self.next_line();
            },
            _ => {
                self.con[self.row * COLUMNS + self.column] = c as u8;
                self.column += 1;

                if self.column == COLUMNS {
                    self.next_line();
                    self.column = 0;
                }
            },
        }
        Ok(())
    }
}
