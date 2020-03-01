The [Xmonad Dynamic Workspace](https://hackage.haskell.org/package/xmonad-contrib-0.16/docs/XMonad-Actions-DynamicWorkspaces.html) behavior for Sway.

Basically, it allows to create named workspaces and dynamically bind
them to indexes (from 1 to 10) in order to access them quickly.


### Getting started

    nix run -f https://github.com/nlewo/swayctl/archive/master.tar.gz -c swayctl --help


### The `swayctl` usage

    swayctl

    USAGE:
        swayctl <SUBCOMMAND>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    SUBCOMMANDS:
        bind      Bind a workspace to an index. The destination workspace must have a name
        help      Prints this message or the help of the given subcommand(s)
        list      List all workspaces
        move      Move a container to a workspace
        rename    Rename a workspace
        show      Show a workspace
        swap      Swap visible workspaces


### My Sway configuration

    # Switch to indexes
    bindsym $mod+ampersand workspace number 1
    bindsym $mod+eacute workspace number 2
    bindsym $mod+quotedbl workspace number 3
    bindsym $mod+apostrophe workspace number 4
    bindsym $mod+parenleft workspace number 5
    bindsym $mod+minus workspace number 6
    bindsym $mod+egrave workspace number 7
    bindsym $mod+underscore workspace number 8
    bindsym $mod+ccedilla workspace number 9
    bindsym $mod+agrave workspace number 10

    # Bind the current workspace to an index (french keyboard)
    bindsym $mod+Control+ampersand exec swayctl bind 1
    bindsym $mod+Control+eacute exec swayctl bind  2
    bindsym $mod+Control+quotedbl exec swayctl bind  3
    bindsym $mod+Control+apostrophe exec swayctl bind  4
    bindsym $mod+Control+parenleft exec swayctl bind  5
    bindsym $mod+Control+minus exec swayctl bind  6
    bindsym $mod+Control+egrave exec swayctl bind  7
    bindsym $mod+Control+underscore exec swayctl bind  8
    bindsym $mod+Control+ccedilla exec swayctl bind  9
    bindsym $mod+Control+agrave exec swayctl bind  10

    # Move a container to a named workspace
    bindsym $mod+Shift+x exec swayctl list | dmenu -p "Move container to workspace: " | xargs swayctl move

    # Switch or create a named workspace
    # Tip: use Shift+Return to ignore the completion of dmenu
    bindsym $mod+x exec swayctl list | dmenu -p "Show or create workspace: " | xargs swayctl show

    # Rename the current workspace
    bindsym $mod+Shift+Control+x exec echo "" | dmenu -p "Rename workspace: " | xargs swayctl rename
