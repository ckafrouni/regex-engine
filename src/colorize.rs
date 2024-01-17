pub trait Colorize {
    fn black(self) -> String;
    fn red(self) -> String;
    fn green(self) -> String;
    fn yellow(self) -> String;
    fn blue(self) -> String;
    fn magenta(self) -> String;
    fn cyan(self) -> String;
    fn white(self) -> String;

    fn on_black(self) -> String;
    fn on_red(self) -> String;
    fn on_green(self) -> String;
    fn on_yellow(self) -> String;
    fn on_blue(self) -> String;
    fn on_magenta(self) -> String;
    fn on_cyan(self) -> String;
    fn on_white(self) -> String;
}

impl<T: AsRef<str>> Colorize for T {
    fn black(self) -> String {
        format!("\x1b[30m{}\x1b[0m", self.as_ref())
    }
    fn red(self) -> String {
        format!("\x1b[31m{}\x1b[0m", self.as_ref())
    }
    fn green(self) -> String {
        format!("\x1b[32m{}\x1b[0m", self.as_ref())
    }
    fn yellow(self) -> String {
        format!("\x1b[33m{}\x1b[0m", self.as_ref())
    }
    fn blue(self) -> String {
        format!("\x1b[34m{}\x1b[0m", self.as_ref())
    }
    fn magenta(self) -> String {
        format!("\x1b[35m{}\x1b[0m", self.as_ref())
    }
    fn cyan(self) -> String {
        format!("\x1b[36m{}\x1b[0m", self.as_ref())
    }
    fn white(self) -> String {
        format!("\x1b[37m{}\x1b[0m", self.as_ref())
    }

    fn on_black(self) -> String {
        format!("\x1b[40m{}\x1b[0m", self.as_ref())
    }
    fn on_red(self) -> String {
        format!("\x1b[41m{}\x1b[0m", self.as_ref())
    }
    fn on_green(self) -> String {
        format!("\x1b[42m{}\x1b[0m", self.as_ref())
    }
    fn on_yellow(self) -> String {
        format!("\x1b[43m{}\x1b[0m", self.as_ref())
    }
    fn on_blue(self) -> String {
        format!("\x1b[44m{}\x1b[0m", self.as_ref())
    }
    fn on_magenta(self) -> String {
        format!("\x1b[45m{}\x1b[0m", self.as_ref())
    }
    fn on_cyan(self) -> String {
        format!("\x1b[46m{}\x1b[0m", self.as_ref())
    }
    fn on_white(self) -> String {
        format!("\x1b[47m{}\x1b[0m", self.as_ref())
    }
}
