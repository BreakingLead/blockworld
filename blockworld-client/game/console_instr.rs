use anyhow::*;

use crate::render::draw::State;

#[derive(Debug)]
pub struct ConsoleInstr {
    pub command: String,
    pub args: Vec<String>,
    pub dialogue_id: Option<u64>,
}

impl ConsoleInstr {
    /// create console instr object from string
    /// such as
    /// `log:(hello_world)`
    /// or with dialogue_id
    /// `log:(hello_world)[3241]`
    pub fn new(console_string: &String) -> Result<ConsoleInstr> {
        let input = console_string.trim().to_string();
        let parts: Vec<String> = input.split(':').map(|s| s.to_string()).collect();
        if parts.len() != 2 {
            return Err(anyhow!(
                "instr should contain 2 parts (instr_name & instr_args) split by \":\" "
            ));
        }
        let command = &parts[0];
        if command.len() == 0 {
            return Err(anyhow!("instr_name should not be null"));
        }

        let args_str = &parts[1];
        if !args_str.starts_with('(') || !args_str.ends_with(')') {
            return Err(anyhow!("instr_args should be wrapped by \"()\""));
        }
        let args_clean = &args_str[1..args_str.len() - 1];
        let args: Vec<String> = args_clean.split(',').map(|s| s.to_string()).collect();

        // dialogue_id is used to distinguish the corresponding relation of request & reponse
        // It can be None if you are sure that
        let dialogue_id: Option<u64> = if input.ends_with("]") {
            let rev_chs = input.chars().rev();
            // 这是个左闭右开区间
            let event_id_start_index = input.len() - 1;
            let event_id_end_index = rev_chs
                .enumerate()
                .find(|(_, ch)| *ch == '[')
                .with_context(|| format!("failed to parse event_id"))?
                .0
                + 1;
            Some(
                input[event_id_start_index..event_id_end_index]
                    .parse::<u64>()
                    .with_context(|| format!("failed to parse event_id"))?,
            )
        } else {
            None
        };
        Ok(ConsoleInstr {
            command: command[0..].to_string(),
            args,
            dialogue_id,
        })
    }
}

macro_rules! match_command{
    ($(command $command_name:ident with_args ($($arg_n:ident:$arg_n_type:ident),*) debug $is_debug:ident)+  in $command_args:ident with_context $ctx:ident) => {
        match $command_args{
            $(ConsoleInstr{command ,args ,dialogue_id:_} if &command == stringify!($command_name) =>{
                let mut count = 0;
                match ($({count+=1;&args[count-1].parse::<$arg_n_type>()}),*){
                    $(std::result::Result::Ok($arg_n)),* =>{
                        $command_name($ctx, $($arg_n),*).await?;
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
pub async fn log(state: &mut State<'_>, s: &String) -> Result<()>{
    println!("{}", s);
    Ok(())
}
pub async fn state_dbg(state: &mut State<'_>) -> Result<()>{
    println!("state:{:?}",state);
    Ok(())
}
pub async fn exec_instr_from_string(console_string: String, state: &mut State<'_>) -> Result<()> {
    let console_instr = ConsoleInstr::new(&console_string).with_context(|| {
        format!(
            "can't recognize this string as console_string for format err:{}",
            console_string
        )
    })?;

    match_command!(
        command log with_args (s:String) debug true
        in console_instr
        with_context state
    );
    Ok(())
}
