use crate::error::CahierError;
use crate::constants::VERSION;


pub fn execute(args: Vec<String>) -> Result<(), CahierError> {
    match args.len() {
        1 => Ok(help()),
        2 => match args[1].as_str() {
            "add" => Ok(help_add()),
            "bring" => Ok(help_bring()),
            "clear" => Ok(help_clear()),
            "config" => Ok(help_config()),
            "connect" => Ok(help_connect()),
            "delete" => Ok(help_delete()),
            "help" => Ok(help_help()),
            "list" => Ok(help_list()),
            "ping" => Ok(help_ping()),
            "send" => Ok(help_send()),
            "setup" => Ok(help_setup()),
            "version" => Ok(help_version()),
            _ => Err(CahierError::InvalidCommand("Invalid help command. Use 'cahier help' for more information.".into())),
        },
        _ => Err(CahierError::InvalidCommand("Too many arguments provided. Use 'cahier help' for more information".into())),
    }
}

fn help() {
    let message = "
\x1b[32musage\x1b[0m:
    cahier <command>

\x1b[32mcommands\x1b[0m:
    \x1b[35madd\x1b[0m      Add a new host to the Cahier configuration file.
    \x1b[35mbring\x1b[0m    Bring a file or directory from a host to the local machine.
    \x1b[35mclear\x1b[0m    Clear the Cahier configuration file.
    \x1b[35mconfig\x1b[0m   Open the Cahier configuration file in the default editor.
    \x1b[35mconnect\x1b[0m  Connect to a host in the Cahier configuration file.
    \x1b[35mdelete\x1b[0m   Delete a host from the Cahier configuration file.
    \x1b[35mhelp\x1b[0m     Display this help message.
    \x1b[35mlist\x1b[0m     List all hosts in the Cahier configuration file.
    \x1b[35mping\x1b[0m     Ping a host in the Cahier configuration file.
    \x1b[35msend\x1b[0m     Send a file or directory from the local machine to a host.
    \x1b[35msetup\x1b[0m    Setup Cahier for the first time.
    \x1b[35mversion\x1b[0m  Display the current version of Cahier.

For more information on a specific command, use 'cahier help <command>'.
";

    println!("\n\x1b[35mcahier\x1b[0m {}\n{}", VERSION, message);
}

fn help_add() {
    let message = "
\x1b[35mcahier\x1b[0m add

\x1b[32musage\x1b[0m:
    cahier add

\x1b[32mdescription\x1b[0m:
    Add a new host to the Cahier configuration file. It will prompt you for the following information:
        - Host name: The name of the host.
        - IP address: The IP address of the host.
        - Nickname: A nickname for the host.
        - Wether to send the ssh key to the host.
";

    println!("{}", message);
}

fn help_bring() {
    let message = "
\x1b[35mcahier\x1b[0m bring

\x1b[32musage\x1b[0m:
    cahier bring <host> <file>

\x1b[32mdescription\x1b[0m:
    Bring a file or directory from a host to the current directory of the local machine.

\x1b[32marguments\x1b[0m:
    \x1b[35mhost\x1b[0m    The nickname of the host from which to bring the file.
    \x1b[35mfile\x1b[0m    The file or directory to bring from the host.
";
    
    println!("{}", message);
}

fn help_clear() {
    let message = "
\x1b[35mcahier\x1b[0m clear

\x1b[32musage\x1b[0m:
    cahier clear

\x1b[32mdescription\x1b[0m:
    Clear the Cahier configuration file.
";
    
    println!("{}", message);
}

fn help_config() {
    let message = "
\x1b[35mcahier\x1b[0m config

\x1b[32musage\x1b[0m:
    cahier config

\x1b[32mdescription\x1b[0m:
    Open the Cahier configuration file in the default editor.
";

    println!("{}", message);
}

fn help_connect() {
    let message = "
\x1b[35mcahier\x1b[0m connect

\x1b[32musage\x1b[0m:
    cahier connect <host>

\x1b[32mdescription\x1b[0m:
    Connect to a host in the Cahier configuration file.

\x1b[32marguments\x1b[0m:
    \x1b[35mhost\x1b[0m    The nickname of the host to connect to.
";

    println!("{}", message);
}

fn help_delete() {
    let message = "
\x1b[35mcahier\x1b[0m delete

\x1b[32musage\x1b[0m:
    cahier delete <host>

\x1b[32mdescription\x1b[0m:
    Delete a host from the Cahier configuration file.

\x1b[32marguments\x1b[0m:
    \x1b[35mhost\x1b[0m    The nickname of the host to delete.
";
    
    println!("{}", message);
}

fn help_help() {
    let message = "
\x1b[35mcahier\x1b[0m help

\x1b[32musage\x1b[0m:
    cahier help [command]

\x1b[32mdescription\x1b[0m:
    Display help information for Cahier or a specific command.

\x1b[32marguments\x1b[0m:
    \x1b[35mcommand\x1b[0m    The command for which to display help information.
";
    
    println!("{}", message);
}

fn help_list() {
    let message = "
\x1b[35mcahier\x1b[0m list

\x1b[32musage\x1b[0m:
    cahier list

\x1b[32mdescription\x1b[0m:
    List all hosts in the Cahier configuration file.
";

    println!("{}", message);
}

fn help_ping() {
    let message = "
\x1b[35mcahier\x1b[0m ping

\x1b[32musage\x1b[0m:
    cahier ping <host>

\x1b[32mdescription\x1b[0m:
    Ping a host in the Cahier configuration file.

\x1b[32marguments\x1b[0m:
    \x1b[35mhost\x1b[0m    The nickname of the host to ping.
";
    
    println!("{}", message);
}

fn help_send() {
    let message = "
\x1b[35mcahier\x1b[0m send

\x1b[32musage\x1b[0m:
    cahier send <host> <file> <destination>

\x1b[32mdescription\x1b[0m:
    Send a file or directory from the local machine to a host.

\x1b[32marguments\x1b[0m:
    \x1b[35mhost\x1b[0m           The nickname of the host to which to send the file.
    \x1b[35mfile\x1b[0m           The file or directory to send to the host.
    \x1b[35mdestination\x1b[0m    The destination directory on the host.
";

    println!("{}", message);
}

fn help_setup() {
    let message = "
\x1b[35mcahier\x1b[0m setup

\x1b[32musage\x1b[0m:
    cahier setup

\x1b[32mdescription\x1b[0m:
    Setup Cahier for the first time. It will prompt you for the following information:
        - Configuration file path: The path to the Cahier configuration file.
        - Wether to generate a new ssh key in your machine.
";  

    println!("{}", message);
}


fn help_version() {
    let message = "
\x1b[35mcahier\x1b[0m version

\x1b[32mdescription\x1b[0m:
    Display the current version of Cahier";

    println!("{} (which is {} btw).\n", message, VERSION);
}
