The [Xmonad Dynamic Workspace](https://hackage.haskell.org/package/xmonad-contrib-0.16/docs/XMonad-Actions-DynamicWorkspaces.html) behavior for Sway.

Basically, this allows to create named workspaces and bind them to
indexes (from 1 to 10) in order to access them quickly.

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

    # Bind the current workspace to an index
    bindsym $mod+Control+ampersand exec sway-dynamic-ws bind 1
    bindsym $mod+Control+eacute exec sway-dynamic-ws bind  2
    bindsym $mod+Control+quotedbl exec sway-dynamic-ws bind  3
    bindsym $mod+Control+apostrophe exec sway-dynamic-ws bind  4
    bindsym $mod+Control+parenleft exec sway-dynamic-ws bind  5
    bindsym $mod+Control+minus exec sway-dynamic-ws bind  6
    bindsym $mod+Control+egrave exec sway-dynamic-ws bind  7
    bindsym $mod+Control+underscore exec sway-dynamic-ws bind  8
    bindsym $mod+Control+ccedilla exec sway-dynamic-ws bind  9
    bindsym $mod+Control+agrave exec sway-dynamic-ws bind  10

    # Move a container to a named workspace
    bindsym $mod+Shift+x exec sway-dynamic-ws list | dmenu -p "Move container to workspace: " | xargs sway-dynamic-ws move 

    # Switch or create a named workspace
    bindsym $mod+x exec sway-dynamic-ws list | dmenu -p "Show or create workspace: " | xargs sway-dynamic-ws show 

### The `sway-dynamic-ws` usage:

    sway-dynamic-workspace 

    USAGE:
        sway-dynamic-ws <SUBCOMMAND>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    SUBCOMMANDS:
        bind      Bind a workspace to an index
        help      Prints this message or the help of the given subcommand(s)
        list      List all workspaces
        move      Move a container to a workspace
        rename    Rename a workspace
        show      Show a workspace
