use anyhow::{Result, anyhow};

pub mod sample_files;


pub struct Arguments {
    pub command: Command
}

#[derive(Debug, Clone)]
pub enum RawCommand {
    Version,
    Help,
    Create,
    Add,
    Bundle,
    Extract
}

impl RawCommand {
    fn new(args: &mut std::env::Args) -> Result<RawCommand> {

        let cmd = args.next().unwrap_or("version".to_string());

        match cmd.as_str() {
            "create" => Ok(RawCommand::Create),
            "add" => Ok(RawCommand::Add),
            "bundle" => Ok(RawCommand::Bundle),
            "extract" => Ok(RawCommand::Extract),
            "help" => Ok(RawCommand::Help),
            "version" => Ok(RawCommand::Version),
            _ => Err(anyhow!("invalid command"))
        }
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    Version,
    Help(RawCommand),
    Create(Create),
    AddNode(AddNode),
    AddWidget(AddWidget),
    Bundle,
    Extract(Extract),
    Manual
}

impl Command {
    pub fn new(mut args: std::env::Args) -> Result<Command> {

        let cmd: RawCommand = RawCommand::new(&mut args)?;

        let out: Result<Command>;

        'base: {
            out = Ok(match cmd {
                RawCommand::Create => {
                    let c = Command::Create(Create {
                        name: args.next().unwrap_or("".to_string()),
                        blank: {
                            if let Some(option) = args.next() {
                                if option == "-blank" {true}
                                else {out = Err(anyhow!("invalid option")); break 'base;}
                            } else {false}
                        }
                    });
                    if let Command::Create(b) = c.clone() {
                        if b.name == "" {out = Err(anyhow!("missing plugin name")); break 'base;}
                    }
                    c
                }
                RawCommand::Add => {
                    if let Some(elem_type) = args.next() {
                        if let Some(name) = args.next() {
                            match elem_type.as_str() {
                                "widget" => {
                                    Command::AddWidget(AddWidget { name })
                                }
                                "node" => {
                                    Command::AddNode(AddNode { name })
                                }
                                _ => {out = Err(anyhow!("invalid element type")); break 'base}
                            }
                        } else {
                            out = Err(anyhow!("missing element name"));
                            break 'base;
                        }
                    } else {
                        out = Err(anyhow!("missing element type"));
                        break 'base;
                    }
                }
                RawCommand::Bundle => {
                    Command::Bundle
                }
                RawCommand::Extract => {
                    if let Some(path) = args.next() {
                        Command::Extract(Extract { origin_path: path })
                    } else {out = Err(anyhow!("missing path of .opb")); break 'base}
                }
                RawCommand::Version => {
                    Command::Version
                }
                RawCommand::Help => {
                    Command::Help(RawCommand::new(&mut args)?)
                }
            })
        };
        out
    }
}

#[derive(Debug, Clone)]
pub struct Create {
    pub name: String,
    pub blank: bool
}

#[derive(Debug, Clone)]
pub struct AddNode {
    pub name: String
}

#[derive(Debug, Clone)]
pub struct AddWidget {
    pub name: String
}

#[derive(Debug, Clone)]
pub struct Extract {
    pub origin_path: String
}