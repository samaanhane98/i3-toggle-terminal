# i3-toggle-terminal

Simple program to toggle a terminal on any screen at a fixed size, which is normally not supported by i3.

## Install

```bash
git clone git@github.com:samaanhane98/i3-toggle-terminal.git
cd i3-toggle-terminal

cargo build --release

 mv ./target/release/i3-toggle-terminal /usr/local/bin/i3-toggle-terminal
```

## Usage

Add the following line to your i3 config:

```md
\# toggle terminal
bindsym $mod+Shift+t exec i3-toggle-terminal

exec --no-startup-id kitty --title=dropdown
for_window [title="^dropdown"] move scratchpad
```
