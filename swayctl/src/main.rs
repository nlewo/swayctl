extern crate i3ipc;
use clap::{App, AppSettings, Arg, SubCommand};
use i3ipc::reply;
use i3ipc::I3Connection;
use itertools::Itertools;

fn main() {
    let app = App::new("swayctl")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(Arg::with_name("debug").short("d").help("Debug commands"))
        .subcommand(
            SubCommand::with_name("bind")
                .about("Bind a workspace to an index. The destination workspace must have a name")
                .arg(
                    Arg::with_name("to")
                        .required(true)
                        .help("The destination index"),
                ),
        )
        .subcommand(
            SubCommand::with_name("rename")
                .about("Rename a workspace")
                .arg(Arg::with_name("name").required(true).help("The new name")),
        )
        .subcommand(
            SubCommand::with_name("show-by-name")
                .about("Show a workspace by it's name")
                .arg(
                    Arg::with_name("name")
                        .required(true)
                        .help("A workspace name"),
                ),
        )
        .subcommand(
            SubCommand::with_name("show-by-num")
                .about("Show a workspace by it's number")
                .arg(
                    Arg::with_name("num")
                        .required(true)
                        .help("A workspace number"),
                ),
        )
        .subcommand(
            SubCommand::with_name("move")
                .about("Move a container to a workspace")
                .arg(
                    Arg::with_name("name")
                        .required(true)
                        .help("The destination workspace name"),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("List all workspaces"))
        .subcommand(SubCommand::with_name("swap").about("Swap visible workspaces"));

    let matches = app.get_matches();

    let mut connection = I3Connection::connect().unwrap();
    let ws = connection.get_workspaces().unwrap();

    let debug = matches.is_present("debug");

    let ret = match matches.subcommand() {
        ("bind", Some(args)) => bind(ws, args.value_of("to").unwrap().parse().unwrap()),
        ("rename", Some(args)) => rename(ws, args.value_of("name").unwrap().to_string()),
        ("show-by-name", Some(args)) => show(
            &ws,
            find_or_create_by_name(&ws, args.value_of("name").unwrap().to_string()),
        ),
        ("show-by-num", Some(args)) => show(
            &ws,
            find_or_create_by_number(&ws, args.value_of("num").unwrap().parse().unwrap()),
        ),
        ("move", Some(args)) => move_to(ws, args.value_of("name").unwrap().to_string()),
        ("list", Some(_args)) => list(ws),
        ("swap", Some(_args)) => swap(ws),
        _ => Err("".to_string()),
    };

    match ret {
        Ok(Some(c)) => {
            if debug {
                println!("I would have run: {}", c)
            } else if let Err(e) = connection.run_command(&c) {
                println!("Run command error {:?}", e)
            }
        }
        Err(e) => println!("Command error: {}", e),
        Ok(None) => (),
    }
}

// An i3 compatible command. Can contain several commands separated
// with ';'
type Command = String;

// Store workspace attributes used to identify a workspace
#[derive(PartialEq, Debug)]
pub struct Workspace {
    pub num: Option<i32>,
    pub name: Option<String>,
    pub output: Option<String>,
    pub visible: bool,
    pub focused: bool,
}

impl Workspace {
    fn new(num: Option<i32>, name: Option<String>) -> Workspace {
        Workspace {
            num,
            name,
            output: None,
            visible: false,
            focused: false,
        }
    }

    fn from_i3ws(ws: &reply::Workspace) -> Workspace {
        let mut parts = ws.name.split(": ");
        let (num, name) = match (parts.next(), parts.next()) {
            (Some(_), None) => {
                if ws.name == ws.num.to_string() {
                    (Some(ws.num), None)
                } else {
                    (None, Some(ws.name.to_string()))
                }
            }
            (Some("-1"), Some(name)) => (None, Some(name.to_string())),
            (Some(_), Some(name)) => (Some(ws.num), Some(name.to_string())),
            // Should not be reached
            (None, _) => (None, None),
        };

        Workspace {
            num,
            name,
            output: Some(ws.output.clone()),
            visible: ws.visible,
            focused: ws.focused,
        }
    }
    /// id returns an id to uniquely identify a workspace based on its attributes
    fn id(&self) -> String {
        let mut id = Vec::new();
        if let Some(num) = self.num {
            id.push(num.to_string())
        };
        if let Some(name) = &self.name {
            id.push(name.to_string())
        };
        id.join(": ")
    }
    /// show focuses workspace and move it to output if provided
    fn show(&self, output: Option<String>) -> Vec<String> {
        let mut cmds = Vec::new();
        if let Some(num) = self.num {
            cmds.push(format!("workspace number {}", num.to_string()));
        } else {
            cmds.push(format!("workspace {}", self.id()));
        }
        match (self.output.as_ref(), output.as_ref()) {
            (Some(o1), Some(o2)) => {
                if o1 != o2 {
                    cmds.push(format!("move workspace to output {}", o2));
                }
            }
            _ => {}
        }
        cmds
    }
    fn move_to(&self, dest: &Workspace) -> String {
        format!("rename workspace {} to {}", self.id(), dest.id())
    }
    /// swap_with swap outputs with other workspace
    fn swap_with(&self, other: &Workspace) -> Vec<String> {
        let mut cmds = Vec::new();
        cmds.push(format!(
            "move workspace to output {}",
            other.output.as_ref().unwrap()
        ));
        cmds.append(&mut other.show(None));
        cmds.push(format!(
            "move workspace to output {}",
            self.output.as_ref().unwrap()
        ));
        cmds
    }
}

fn find_current(ws: &reply::Workspaces) -> Workspace {
    ws.workspaces
        .iter()
        .find(|&w| w.focused)
        .map(|w| Workspace::from_i3ws(w))
        .unwrap()
}

fn find_or_create_by_number(ws: &reply::Workspaces, number: i32) -> Workspace {
    ws.workspaces
        .iter()
        .map(|w| Workspace::from_i3ws(w))
        .find(|w| w.num.as_ref().map(|x| x == &number).unwrap_or(false))
        .unwrap_or(Workspace::new(Some(number), None))
}

fn find_or_create_by_name(ws: &reply::Workspaces, name: String) -> Workspace {
    ws.workspaces
        .iter()
        .map(|w| Workspace::from_i3ws(w))
        .find(|w| w.name.as_ref().map(|x| x == &name).unwrap_or(false))
        .unwrap_or(Workspace::new(None, Some(name.clone())))
}

fn move_to(ws: reply::Workspaces, name: String) -> Result<Option<Command>, String> {
    let w = find_or_create_by_name(&ws, name);
    Ok(Some(format!("move container to workspace {}", w.id())))
}

fn show(ws: &reply::Workspaces, target: Workspace) -> Result<Option<Command>, String> {
    let cmds: Vec<String>;
    let current = find_current(ws);

    if target == current {
        return Ok(None);
    }

    if target.visible {
        cmds = current.swap_with(&target);
    } else {
        cmds = target.show(current.output);
    }

    Ok(Some(cmds.join("; ")))
}

fn list(ws: reply::Workspaces) -> Result<Option<Command>, String> {
    let names: Vec<String> = ws
        .workspaces
        .iter()
        .filter_map(|w| Workspace::from_i3ws(w).name)
        .sorted()
        .collect();

    println!("{}", names.join("\n"));
    Ok(None)
}

fn rename(ws: reply::Workspaces, name: String) -> Result<Option<Command>, String> {
    let current = find_current(&ws);

    let already_exist = ws
        .workspaces
        .iter()
        .map(|w| Workspace::from_i3ws(w))
        .find(|w| w.name == Some(name.to_string()));

    if already_exist.is_some() {
        return Err(format!("a workspace named {} already exists", name));
    };

    let renamed = Workspace::new(current.num, Some(name));
    let cmd = format!("rename workspace to \"{}\"", renamed.id());
    Ok(Some(cmd))
}

fn bind(ws: reply::Workspaces, to: i32) -> Result<Option<Command>, String> {
    let mut cmds = Vec::new();

    let current = find_current(&ws);

    // If the destination is the current position, do nothing
    if let Some(num) = current.num {
        if num == to {
            return Ok(None);
        }
    }

    let dest = ws
        .workspaces
        .iter()
        .find(|&w| w.num == to)
        .map(|w| Workspace::from_i3ws(w));

    let new = Workspace::new(Some(to), current.name.clone());

    // If the destination workspace already exists, we first rename
    // the destination workspace with a temporary name to free its
    // index. We can then move the current workspace to the
    // destination index. Finally, we move the temporary named
    // workspace to the current index.
    if let Some(d) = dest {
        // If the destination index is bound to a not named workspace,
        // we just skip this binding. If we don't, we could loose the
        // destination workspace (no bound anymore and no name).
        if let None = d.name {
            return Err("the destination index is bound to a not named workspace".to_string());
        }

        let tmp = Workspace::new(None, Some("internal-tmp-swapping".to_string()));
        cmds.push(d.move_to(&tmp));

        cmds.push(current.move_to(&new));

        let swap = Workspace::new(current.num, d.name);
        cmds.push(tmp.move_to(&swap));
    }
    // Otherwise, just move the current workspace to the destination
    else {
        cmds.push(current.move_to(&new));
    }
    Ok(Some(cmds.join("; ")))
}

fn swap(ws: reply::Workspaces) -> Result<Option<Command>, String> {
    let visible = ws
        .workspaces
        .iter()
        .map(|w| Workspace::from_i3ws(w))
        .filter(|w| w.visible)
        .collect::<Vec<Workspace>>();

    if visible.len() != 2 {
        return Ok(None);
    }

    let current = visible.iter().find(|&w| w.focused).unwrap();
    let other = visible.iter().find(|&w| !w.focused).unwrap();

    Ok(Some(current.swap_with(other).join("; ")))
}

#[test]
fn new_workspaces() {
    fn dummy_ws(num: i32, name: &str) -> reply::Workspace {
        reply::Workspace {
            num,
            name: name.to_string(),
            visible: true,
            focused: true,
            urgent: false,
            rect: (0, 0, 0, 0),
            output: "".to_string(),
        }
    }
    assert_eq!(
        Workspace::from_i3ws(&dummy_ws(1, "1")),
        Workspace {
            num: Some(1),
            name: None,
            visible: true,
            focused: true,
            output: Some("".to_string()),
        }
    );
    assert_eq!(
        Workspace::from_i3ws(&dummy_ws(1, "1: mail")),
        Workspace {
            num: Some(1),
            name: Some("mail".to_string()),
            visible: true,
            focused: true,
            output: Some("".to_string()),
        }
    );
    assert_eq!(
        Workspace::from_i3ws(&dummy_ws(1, "mail")),
        Workspace {
            num: None,
            name: Some("mail".to_string()),
            visible: true,
            focused: true,
            output: Some("".to_string()),
        }
    );
    assert_eq!(
        Workspace::from_i3ws(&dummy_ws(-1, "-1: mail")),
        Workspace {
            num: None,
            name: Some("mail".to_string()),
            visible: true,
            focused: true,
            output: Some("".to_string()),
        }
    )
}
