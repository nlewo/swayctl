The [Xmonad Dynamic Workspace](https://hackage.haskell.org/package/xmonad-contrib-0.16/docs/XMonad-Actions-DynamicWorkspaces.html) behavior for Sway.

Basically, it allows to create named workspaces and dynamically bind
them to indexes (from 1 to 10) in order to access them quickly.


### Getting started

```sh
nix run -f https://github.com/nlewo/swayctl/archive/master.tar.gz -c swayctl --help
```


### The `swayctl` usage

```
USAGE:
    swayctl [FLAGS] <SUBCOMMAND>

FLAGS:
    -d               Debug commands
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    bind         Bind a workspace to an index. The destination workspace must have a name
    help         Prints this message or the help of the given subcommand(s)
    list         List all workspaces
    move         Move a container to a workspace
    rename       Rename a workspace
    show-name    Show a workspace by it's name
    show-num     Show a workspace by it's number
    swap         Swap visible workspaces
```


### My Sway configuration

```
set $show exec swayctl show-num
set $bind exec swayctl bind

# Switch to indexes
bindsym $mod+ampersand $show 1
bindsym $mod+eacute $show 2
bindsym $mod+quotedbl $show 3
bindsym $mod+apostrophe $show 4
bindsym $mod+parenleft $show 5
bindsym $mod+minus $show 6
bindsym $mod+egrave $show 7
bindsym $mod+underscore $show 8
bindsym $mod+ccedilla $show 9
bindsym $mod+agrave $show 10

# Bind the current workspace to an index (french keyboard)
bindsym $mod+Control+ampersand $bind1
bindsym $mod+Control+eacute $bind 2
bindsym $mod+Control+quotedbl $bind 3
bindsym $mod+Control+apostrophe $bind 4
bindsym $mod+Control+parenleft $bind 5
bindsym $mod+Control+minus $bind 6
bindsym $mod+Control+egrave $bind 7
bindsym $mod+Control+underscore $bind 8
bindsym $mod+Control+ccedilla $bind 9
bindsym $mod+Control+agrave $bind 10

# Move a container to a named workspace
bindsym $mod+Shift+x exec swayctl list | dmenu -p "Move container to workspace: " | xargs swayctl move

# Switch or create a named workspace
# Tip: use Shift+Return to ignore the completion of dmenu
bindsym $mod+x exec swayctl list | dmenu -p "Show or create workspace: " | xargs -I{} swayctl show-name "{}"

# Rename the current workspace
bindsym $mod+Shift+Control+x exec echo "" | dmenu -p "Rename workspace: " | xargs swayctl rename
```
