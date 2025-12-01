use crate::modules::uart::uart_pl011;

use heapless::String;
use crate::modules::shell::proto_shell;

const ARG_MAX: usize = 2048;

pub unsafe fn proto_shell_exec(cmd: &str) {
    let (commd, var) = cmd.split_once(' ').unwrap_or((cmd, ""));
    if (commd == "echo") {
        proto_shell_echo(var);
    } else if (commd == "help") {
        // proto_shell_help();
        uart_pl011::puts(proto_shell_help());
    } else {
        proto_shell_echo("No command found");
    }
}

pub unsafe fn proto_shell_init() {
    let mut string_input: String<ARG_MAX> = String::new();
    // Later will have to replace uart_pl011 calls to "output" buffer so it will just have to be printed out and not called upon each time I want to print something.
    uart_pl011::puts("> ");
    loop {
        let mut input = uart_pl011::getc();
        if (input == 0x0A || input == 0x0D) {
            proto_shell_exec(&string_input);
            break;
        } else {
            string_input.push(input as char);
        }
    }
}

pub unsafe fn proto_shell_clear() {

}

pub unsafe fn proto_shell_echo(word: &str) {
    // For now it calls uart_pl011 to output to UART but later it has to be changed to something internal that returns just string to be displayed
    uart_pl011::puts(word);
    uart_pl011::puts("\n");
}

pub unsafe fn proto_shell_help() -> &'static str{
    // Not best solution but for now it has to do as I have to figure out "GLOBAL MEMORY ALLOCATOR"
    // uart_pl011::puts("This is simple help message.\nCurrently available commands:\n- echo <text> - return your text\n- help - display this message\n");
    "This is simple help message.\nCurrently available commands:\n- echo <text> - return your text\n- help - display this message\n"
}
