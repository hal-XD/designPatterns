trait Command<T> {
    fn execute(&self, c:&mut T);
}

struct Invoker<'a, Cmd, T: 'a> {
    commands: Vec<Cmd>,
    target: &'a mut T,
    current_index: usize,
}

impl<'a, Cmd, T> Invoker<'a, Cmd, T> {
    fn new(t: &'a mut T) -> Self {
        Self {
            commands: Vec::new(),
            target: t,
            current_index: 0,
        }
    }

    fn target(&self) -> &T {
        self.target
    }

    fn append_command(&mut self, c:Cmd) {
        self.commands.push(c)
    }
}

impl<'a, Cmd, T> Invoker<'a, Cmd, T>
where Cmd: Command<T>
{
    fn execute_command(&mut self) {
        if self.commands.len() <= self.current_index {
            return;
        }

        let c = &self.commands[self.current_index];
        let t = &mut *self.target;
        c.execute(t);

        self.current_index += 1;
    }
}

enum Commands {
    Command1,
}

struct Device{}
impl Device {
    fn command1(&self) {
        println!("execute command1")
    }
}

impl Command<Device> for Commands {
    fn execute(&self, r:&mut Device) {
        match *self {
            Commands::Command1 => r.command1(),
            _ => unreachable!(),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
