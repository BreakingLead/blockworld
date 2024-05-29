use anyhow::*;

use crate::render::draw::State;

#[derive(Debug)]
pub struct ConsoleInstr {
    pub command: String,
    pub args: Vec<String>,
    pub dialogue_id: Option<u64>,
}

impl ConsoleInstr {
    // create console instr object from string
    pub fn new(console_string: String) -> Option<ConsoleInstr> {
        let input = console_string.trim().to_string();
        let parts: Vec<String> = input.split(':').map(|s| s.to_string()).collect();
        if parts.len() != 2 {
            return None;
        }
        let command = &parts[0];
        if command.len() == 0 {
            println!("message has no command");
            return None;
        }

        let args_str = &parts[1];
        if !args_str.starts_with('(') || !args_str.ends_with(')') {
            return None;
        }
        let args_clean = &args_str[1..args_str.len() - 1];
        let args: Vec<String> = args_clean.split(',').map(|s| s.to_string()).collect();

        let dialogue_id: Option<u64> = if input.ends_with("]") {
            let rev_chs = input.chars().rev();
            // 这是个左闭右开区间
            let event_id_start_index = input.len() - 1;
            let event_id_end_index = rev_chs
                .enumerate()
                .find(|(index, ch)| *ch == '[')
                .unwrap()
                .0
                + 1;
            Some(
                input[event_id_start_index..event_id_end_index]
                    .parse::<u64>()
                    .unwrap(),
            )
        } else {
            None
        };
        Some(ConsoleInstr {
            command: command[0..].to_string(),
            args,
            dialogue_id,
        })
    }
}

macro_rules! _match_command{
    ($(command $command_name:ident with_args ($($arg_n:ident:$arg_n_type:ident),*) debug $is_debug:ident)+  in $command_args:ident with_context $ctx:ident) => {
        match $command_args{
            $(ConsoleInstr{command ,args ,dialogue_id} if &command == stringify!($command_name) =>{
                let mut count = 0;
                match ($({count+=1;&args[count-1].parse::<$arg_n_type>()}),*){
                    ($(std::result::Result::Ok($arg_n)),*) =>{
                        $command_name($ctx, $($arg_n),*);
                        if $is_debug {
                            println!("successfullly run \x1B[32m {} \x1B[0m with args \x1B[32m{:?}\x1B[0m",command,args);
                        }
                    },
                    _ => {println!("conversion failed \x1B[31m{}\x1B[0m with args \x1B[31m{:?}\x1B[0m ",stringify!($command_name), args)}
                }
            }),*
            _ => { return Err(anyhow!("instr not found"))}
        }
    } ;
}
pub fn log(state: &mut State, s: &String) {
    println!("{}", s);
}
pub fn match_command(console_string: String, state: &mut State) -> Result<()> {
    let console_instr = ConsoleInstr::new(console_string.clone()).with_context(|| {
        format!(
            "can't recognize this string as console_string for format err:{}",
            console_string
        )
    })?;
    _match_command!(
        command log with_args (s:String) debug true
        in console_instr
        with_context state
    );
    Ok(())
}
