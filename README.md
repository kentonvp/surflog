```
 ________       ___  ___      ________      ________      ___           ________      ________      ________
 |\   ____\     |\  \|\  \    |\   __  \    |\  _____\    |\  \         |\   __  \    |\   ____\    |\   ____\
 \ \  \___|_    \ \  \\\  \   \ \  \|\  \   \ \  \__/     \ \  \        \ \  \|\  \   \ \  \___|    \ \  \___|_
 \ \_____  \    \ \  \\\  \   \ \   _  _\   \ \   __\     \ \  \        \ \  \\\  \   \ \  \  ___   \ \_____  \
  \|____|\  \    \ \  \\\  \   \ \  \\  \|   \ \  \_|      \ \  \____    \ \  \\\  \   \ \  \|\  \   \|____|\  \
    ____\_\  \    \ \_______\   \ \__\\ _\    \ \__\        \ \_______\   \ \_______\   \ \_______\    ____\_\  \
   |\_________\    \|_______|    \|__|\|__|    \|__|         \|_______|    \|_______|    \|_______|   |\_________\
   \|_________|                                                                                       \|_________|

```

# Feature Plans:
## Data Collection
* [ ] Collect surf data from NOAA

## Admin UI
* [ ] UI for buoy statuses

## ML
* [ ] Create models on previous surf sessions. Targets: WaveHeight & Rating
* [ ] Use model to predict future surf sessions with current buoy data.

# CLI Tool
## Help
```bash
❯ surflog
CLI tool to track surf sessions

Usage: surflog <COMMAND>

Commands
  add     Add a new surf session
  delete  Delete a surf session
  list    List surf sessions
  config  Display configuration elements
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
