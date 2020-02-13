extern crate i3ipc;
use i3ipc::I3Connection;
use i3ipc::MessageError;
use i3ipc::reply;

use clap::{App, Arg, SubCommand, AppSettings};

fn main() {
    let app = App::new("sway-dynamic-workspace")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("bind")
                .about("Bind a workspace to an index")
                .arg(
                    Arg::with_name("to")
                        .required(true)
                        .help("The destination index"),
                )
        )
        .subcommand(
            SubCommand::with_name("rename")
                .about("Rename a workspace")
                .arg(
                    Arg::with_name("name")
                        .required(true)
                        .help("The new name"),
                )
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("Show a workspace")
                .arg(
                    Arg::with_name("name")
                        .required(true)
                        .help("A workspace name"),
                )
        )
        .subcommand(
            SubCommand::with_name("move")
                .about("Move a container to a workspace")
                .arg(
                    Arg::with_name("name")
                        .required(true)
                        .help("The destination workspace name"),
                )
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("List all workspaces")
        );
    
    let matches = app.get_matches();
    
    let mut connection = I3Connection::connect().unwrap();
    let ws = connection.get_workspaces().unwrap();
    
    let ret = match matches.subcommand() {
        ("bind", Some(args)) =>
            bind(ws, args.value_of("to").unwrap().parse().unwrap()),
        ("rename", Some(args)) =>
             rename(ws, args.value_of("name").unwrap().to_string()),
        ("show", Some(args)) =>
            show(ws, args.value_of("name").unwrap().to_string()),
        ("move", Some(args)) =>
            moveTo(ws, args.value_of("name").unwrap().to_string()),
        ("list", Some(_args)) =>
            list(ws),
        _ => None
    };

    if let Some(c) = ret {
        if let Err(e) = connection.run_command(&c) {
            println!("{:?}", e)
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Workspace {
    pub num: Option<i32>,
    pub name: Option<String>,
}

fn workspace_id(ws: &Workspace) -> String {
    let mut id = Vec::new();
    if let Some(num) = ws.num {
        id.push(num.to_string())
    };
    if let Some(name) = &ws.name {
        id.push(name.to_string())
    };
    id.join(": ")
}

fn workspace_move(src: &Workspace, dest: &Workspace) -> String {
    format!("rename workspace {} to {}",
            workspace_id(&src),
            workspace_id(&dest))
}

fn new_workspace(ws: &reply::Workspace) -> Workspace {
    let v: Vec<&str> = ws.name.split(": ").collect();
    let num =
        if v.len() == 1 && ws.name == ws.num.to_string() && ws.num != -1 {
            Some(ws.num)
        } else if v.len() == 2 && ws.num != -1 {
            Some(ws.num)
        } else {
            None 
        };
    let name =
        if v.len() == 1 && ws.name != ws.num.to_string() {
            Some(v[0].to_string())
        } else if v.len() == 2 {
            Some(v[1].to_string())
        } else {
            None
        };
    return Workspace {
        num: num,
        name: name,
    }
}

fn find_or_create(ws: reply::Workspaces, name: String) -> Workspace {
    ws.workspaces
        .iter()
        .map(|w| new_workspace(w))
        .find(|w| w.name.as_ref().map(|x| x == &name).unwrap_or(false))
        .unwrap_or(Workspace {num: None, name: Some(name)})
}

fn moveTo(ws: reply::Workspaces, name: String) -> Option<String> {
    let w = find_or_create(ws, name);
    Some(format!("move container to workspace {}", workspace_id(&w)))
}

fn show(ws: reply::Workspaces, name: String) -> Option<String> {
    let w = find_or_create(ws, name);
    Some(format!("workspace {}", workspace_id(&w)))
}

fn list(ws: reply::Workspaces)  -> Option<String> {
    let names: Vec<String> = ws.workspaces
        .iter()
        .map(|w| new_workspace(w))
        .filter(|w| w.name.is_some())
        .map(|w| w.name.unwrap())
        .collect();

    println!("{}", names.join("\n"));
    None
}

fn rename(ws: reply::Workspaces, name: String) -> Option<String> {
    let current = ws.workspaces
        .iter()
        .find(|&w| w.focused)
        .map(|w| new_workspace(w))
        .unwrap();

    if let Some(n) = current.name {
        if n == name {
            return None
        }
    };

    let renamed = Workspace { num: current.num, name: Some(name) };
    let cmd = format!("rename workspace to \"{}\"", workspace_id(&renamed));
    Some(cmd)
}

fn bind(ws: reply::Workspaces, to: i32) -> Option<String> {
    let mut cmds = Vec::new();
    
    let current = ws.workspaces
        .iter()
        .find(|&w| w.focused)
        .map(|w| new_workspace(w))
        .unwrap();

    // If the destination is the current position, do nothing
    if let Some(num) = current.num {
        if num == to {
            return None
        }
    }

    let dest = ws.workspaces.iter().find(|&w| w.num == to).map(|w| new_workspace(w));

    let new = Workspace { num: Some(to), name: current.name.clone() };

    // If the destination workspace already exist,
    // swap current and destination workspaces
    if let Some(d) = dest {
        let tmp = Workspace { num: None, name: Some("internal-tmp-swapping".to_string()) };
        cmds.push(workspace_move(&d, &tmp));
    
        cmds.push(workspace_move(&current, &new));

        let swap = Workspace { num: current.num, name: d.name };
        cmds.push(workspace_move(&tmp, &swap));
    }
    // Otherwise, just move the current workspace to the destination
    else {
        cmds.push(workspace_move(&current, &new));
    }
    Some(cmds.join("; "))
}

#[test]
fn new_workspaces () {
    fn dummy_ws(num: i32, name: &str) -> reply::Workspace {
        reply::Workspace {
            num: num,
            name: name.to_string(),
            visible: true,
            focused: true,
            urgent: false,
            rect: (0, 0, 0, 0),
            output: "".to_string(),
        }
    }
    assert_eq!(
        new_workspace(&dummy_ws(1, "1")),
        Workspace{num: Some(1), name: None}
    );
    assert_eq!(
        new_workspace(&dummy_ws(1, "1: mail")),
        Workspace{num: Some(1), name: Some("mail".to_string())}
    );
    assert_eq!(
        new_workspace(&dummy_ws(1, "mail")),
        Workspace{num: None, name: Some("mail".to_string())}
    );
    assert_eq!(
        new_workspace(&dummy_ws(-1, "-1: mail")),
        Workspace{num: None, name: Some("mail".to_string())}
    )
}
