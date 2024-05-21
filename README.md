# qbfrt (qBittorrent fastresume tool)
Command line tool for working with qBittorrent's fastresume data. Supports the experimental SQLite database and (soon) traditional .fastresume files.

`queue-bee-fart`

<br>

## Why?
The qBittorrent SQLite database is still experimental. While it drastically improves start up times when you have many torrents, most tools that manipulate qBittorrent fastresume data do not support the new database. This tool prevents you from having to convert back to the .fastresume files to use those other fastresume tools.

<br>

## Features
With this tool you can:
- Mass update the save paths for torrents in the SQLite database
    - Change files to a new drive or directory without having to move torrents in qBittorrent or recheck all of the torrent data
    - Migrate from qBittorrent on Windows to Linux without having to recheck the torrent data


**More functionality to come!**

The application will look for the default qBittorrent data directory
containing the torrents.db file. This behavior can be changed by passing
`--data_dir /some/path/to/db`

<br>

## Arguments
- `-p, --config_dir` - Path to the qB local config directory (where torrents.db lives)
    - uses default qBittorrent data directory if not specified
- `-d, --disable_backup` - Disables the automatic torrents.db backup
- `--existing-path` - Existing save path or path fragment
    - requires `--new-path` to be provided
- `--new-path` - New save path or path fragment to replace existing path
    - requires `--existing-path` to be provided
- `-v, --verbose` - Enables more verbose output
- `--old-path` - The old save path or partial path to replace
- `--new-path` - The new save path or partial path to replace with
- `--use-unix-sep` - Force using path slash "/" for updated paths
- `--use-win-sep` - Force using Windows separators "\" for updated paths

<br>

## Examples and Usage
### Updating save path on Unix
Here the torrent is saved at `~/torrents/some/old/path/here`. Running the following command
will result in the save path becoming `~/torrents/new/thing/here`.
```bash
qbfrt -v --old-path /some/old/path --new-path /new/thing
```
### Updating save path on Windows
Here the torrent is saved at `D:\Downloads\torrents\some\old\path\here`. Running the following
command will result in the save path becoming `C:\torrents\some\old\path\here`.  
```powershell
.\qbfrt -v --old-path "D:\Downloads" --new-path "C:\"
```
### Force using specific path separator
You can force updated paths to use a specific separator by passing `--use-unix-sep` or `--use-win-sep`.
This is useful if you want to update save paths for a different machine. Here the torrent is saved at
`D:\Downloads\some\folder` on a Windows machine and we are running the command on Linux. The new path
will still use Windows "\" path separators. Note you would have to escape the back slashes in bash.
```bash
qbfrt -v --old-path "D:\\Downloads" --new-path "C:\\" --use-win-sep
```

<br>

## Notes
- By default, a timestamped backup of the torrents.db file will be created before processing changes. Currently,
a simple file copy is used to do the backup, not a proper SQL dump. **qBittorrent should be completely shut down
before running this tool.**
- The save path replacement uses a lazy find and replace. It will replace all instances of the old string. Be careful
if you are updating partial paths that may share segments with others. e.g. `--existing-path /torrents/movie` will
match both `/torrents/movies` and `/torrents/movie-folder`. Avoid using a single word, it will replace all instances
of it.
- If you end a string with a slash, use care to include a slash at the end of the new string, otherwise it will remove it.
- You have to run the command once for each path you want to change, currently you can not batch different path replacements.
- Use something like [Beekeeper Studio](https://www.beekeeperstudio.io/) to confirm the appropriate changes
were made. Check the `target_save_path` column. You can check the libtorrent_resume_data save path, but first
you will have to convert the hex blob to text.
- Git bash/MINGW64 on Windows: mingw messes up partial paths starting with "/" and makes them relative to the local git
program directory. See [here](https://github.com/moby/moby/issues/24029#issuecomment-250412919). Run the command with command
prompt or powershell instead.

<br>

## Building From Source
Until I figure out how to compile cross-platform, you're going to have to do it yourself.


### 1. [Install Rust](https://www.rust-lang.org/tools/install)
### 2. Clone the git repo:
```bash
git clone https://github.com/strangeepoxy/qbfrt.git
```
### 3. Compile the app:
```bash
cargo build --release
```
### 4. Move the compiled executable where you want it:
#### Unix
```bash
mv /target/release/qbfrt /place/you/want/qbfrt
```
#### Windows
```powershell
move ./target/release/qbfrt.exe C:/place/you/want/qbfrt.exe
```
### 5. (Optional) Add qbfrt to your path

<br>

## Updating From Source

### 1. Pull updates from repo:
```bash
git pull
```
### 2. Follow "Building From Source" starting at step #3.
