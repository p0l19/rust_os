use volatile::Volatile;
use core::fmt;

//The VGA text-buffer has 25 rows and each row can show 80 ascii_characters
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct ColorCode(u8);

impl ColorCode {

    fn new(foreground: Color, background: Color) -> ColorCode {
        //the background color information (4 bits) is stored in the left / higher bits of the color byte
        // the information of the foreground color is stored on the right / lower side on the bit
        // the | represents the bit-wise OR operator, since the background color bits are moved to the right the OR will combine background and foreground color in this one bit
        ColorCode((background as u8)  << 4| (foreground as u8))
    }
}

/** Each character wich can be shown on the VGA-Buffer consists of the code for the character and its color code */
#[derive(Debug,Copy,Clone,PartialEq,Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode
}

/** The VGA-Buffer consists of a two-dimensional array in which ascii-characters with color code are stored
The Buffer consists of 25 lines where each has 80 characters
 */
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub static WRITER: Writer = Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer)}
};

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    //The 'static signals that the Buffer variable will be available through the entire programm runtime
    buffer: &'static mut Buffer
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {

        match byte {
            b'\n' => self.new_line(), //if the byte is the newline sign a new line will be opend
            byte => {
                if self.column_position >= BUFFER_WIDTH { //for all other bytes check if the current buffer row is alredy full
                    self.new_line() //if it is a new line will be opend
                }

                let row = BUFFER_HEIGHT -1;
                let col = self.column_position;
                let color_code = self.color_code;

                self.buffer.chars[row][col].write(ScreenChar{ //the position in the Buffer will be filled with a printable and colored ScreenChar
                    ascii_character: byte,
                    color_code
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, str: &str) {
        for byte in str.bytes() {
            match byte {
                //ascii-sign that can be printed. The VGA Buffer can not print the whole ascii-range
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                //for all signs that do not fit into the ascii range a white dot will be printed
                _ => self.write_byte(0xfe)
            }
        }
    }
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row-1][col].write(character);
            }
        }
        self.column_position = 0;
        self.clear_row(BUFFER_HEIGHT-1);
    }

    fn clear_row(&mut self, row: usize) {
        let blank_screen = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank_screen);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.write_byte(c as u8);
        Ok(())
    }
}

pub fn print_something() {
    use core::fmt::Write;
    let mut writer: Writer = Writer{
        column_position: 0,
        color_code: ColorCode::new(Color::Green, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer)}
    };

    write!(writer, "Hello paulita i love you. You are very very beautiful do you now that? I am very proud of you baby gorl").unwrap();
}
