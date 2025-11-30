use core::char::from_u32;
use crate::modules::uart::uart_pl011;

pub unsafe fn proto_shell_exec(cmd: &str) {
    let (commd, var) = cmd.split_once(' ').unwrap_or((cmd, ""));
    if (commd == "echo") {
        proto_shell_echo(var);
    } else if (commd == "help") {
        proto_shell_help();
    } else {
        proto_shell_echo("No command found");
    }
}

pub unsafe fn proto_shell_init() {
    // let mut input_str = String::from("");
    // loop {
    //     input_str += &*uart_pl011::getc();
    // }
}

pub unsafe fn proto_shell_clear() {

}

pub unsafe fn proto_shell_echo(word: &str) {
    uart_pl011::puts(word);
}

pub unsafe fn proto_shell_help() {
    // Not best solution but for now it has to do as I have to figure out "GLOBAL MEMORY ALLOCATOR"
    uart_pl011::puts("This is simple help message.\nCurrently available commands:\n- echo <text> - return your text\n- help - display this message\n");
    // "This is simple help message.\nCurrently available commands:\n- echo <text> - return your text\n- help - display this message\n"
}
