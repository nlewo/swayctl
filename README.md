The [Xmonad Dynamic Workspace](https://hackage.haskell.org/package/xmonad-contrib-0.16/docs/XMonad-Actions-DynamicWorkspaces.html) behavior for Sway.

Basically, it allows to create named workspaces and dynamically bind
them to indexes (from 1 to 10) in order to access them quickly.


### Getting started

Without flakes (before nix 2.4):

```sh
nix run -f https://github.com/nlewo/swayctl/archive/master.tar.gz -c swayctl --help
```

With flakes (nix 2.4):

```sh
nix run github:nlewo/swayctl -- --help
```

### The `swayctl` usage

    swayctl

    USAGE:
        swayctl [FLAGS] <SUBCOMMAND>

    FLAGS:
        -d               Debug commands
        -h, --help       Prints help information
        -V, --version    Prints version information

    SUBCOMMANDS:
        bind            Bind a workspace to an index. The destination workspace must have a name
        help            Prints this message or the help of the given subcommand(s)
        list            List all workspaces
        move            Move a container to a workspace
        rename          Rename a workspace
        show-by-name    Show a workspace by it's name
        show-by-num     Show a workspace by it's number
        swap            Swap visible workspaces


### My Sway configuration

    bindsym --to-code {
      $mod+1 exec swayctl show-by-num 1
      $mod+2 exec swayctl show-by-num 2
      $mod+3 exec swayctl show-by-num 3
      $mod+4 exec swayctl show-by-num 4
      $mod+5 exec swayctl show-by-num 5
      $mod+6 exec swayctl show-by-num 6
      $mod+7 exec swayctl show-by-num 7
      $mod+8 exec swayctl show-by-num 8
      $mod+9 exec swayctl show-by-num 9
      $mod+0 exec swayctl show-by-num 10

      $mod+Control+1 exec swayctl bind 1
      $mod+Control+2 exec swayctl bind 2
      $mod+Control+3 exec swayctl bind 3
      $mod+Control+4 exec swayctl bind 4
      $mod+Control+5 exec swayctl bind 5
      $mod+Control+6 exec swayctl bind 6
      $mod+Control+7 exec swayctl bind 7
      $mod+Control+8 exec swayctl bind 8
      $mod+Control+9 exec swayctl bind 9
      $mod+Control+0 exec swayctl bind 10
    }

    # Move a container to a named workspace
    bindsym $mod+Shift+x exec swayctl list | dmenu -p "Move container to workspace: " | xargs swayctl move

    # Switch or create a named workspace
    # Tip: use Shift+Return to ignore the completion of dmenu
    bindsym $mod+x exec swayctl list | dmenu -p "Show or create workspace: " | xargs -I{} swayctl show-by-name "{}"

    # Rename the current workspace
    bindsym $mod+Shift+Control+x exec echo "" | dmenu -p "Rename workspace: " | xargs swayctl rename
