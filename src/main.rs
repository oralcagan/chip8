mod decoder;
mod display;
mod interpreter;
mod mem;
mod util;

fn main() {
    interpreter::Interpreter::try_new("exec.ch8").unwrap().run();
}
