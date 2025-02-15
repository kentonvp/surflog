/*
 ________       ___  ___      ________      ________      ___           ________      ________      ________
 |\   ____\     |\  \|\  \    |\   __  \    |\  _____\    |\  \         |\   __  \    |\   ____\    |\   ____\
 \ \  \___|_    \ \  \\\  \   \ \  \|\  \   \ \  \__/     \ \  \        \ \  \|\  \   \ \  \___|    \ \  \___|_
 \ \_____  \    \ \  \\\  \   \ \   _  _\   \ \   __\     \ \  \        \ \  \\\  \   \ \  \  ___   \ \_____  \
  \|____|\  \    \ \  \\\  \   \ \  \\  \|   \ \  \_|      \ \  \____    \ \  \\\  \   \ \  \|\  \   \|____|\  \
    ____\_\  \    \ \_______\   \ \__\\ _\    \ \__\        \ \_______\   \ \_______\   \ \_______\    ____\_\  \
   |\_________\    \|_______|    \|__|\|__|    \|__|         \|_______|    \|_______|    \|_______|   |\_________\
   \|_________|                                                                                       \|_________|
*/

mod cli;
mod database;

use anyhow::Result;

fn main() -> Result<()> {
    cli::run()
}
