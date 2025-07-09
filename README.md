# Dotfiles Templater
Useful tool for replacing things like colour across different Linux dotfiles, intended for use with window managers, but can be used across any file. PLEASE BACKUP ANY FILES BEFORE USING THIS! It should be safe, but it doesn't hurt to be careful.

Currently any replacement must be of the same length as the text it replaces, this is to make sure that the correct parts of the config are modified only.

## Configuration
Put a TOML file at ```$XDG_CONFIG_HOME/dotfile-templater/config.toml``` to configure the templater.

Optionally set ```marker_repetition_num``` to dictate the number of marker characters which signify the divide between the template code and your config, the default is 3.

Set ```files = [{file = "test/test.conf", marker_char = "%"]``` to dictate which files should be modified by the templater. The file can be an absolute path, or relative to the ```.config/``` folder (Like here).

Set ```theme = "my_theme"``` to choose which of your themes the colours should be chosen from.

### Themes
In the ```[[themes]]``` section, the only necessary definition is ```name = "my_name"```, all other variables can be used, as typed in the config, within the template code.

#### TOML Config
```toml
theme = "purple-night"
marker_repetition_num = 3
files = [
  {file = "test/test.conf", marker_char = "%"},
  {file = "another/another.rs", marker_char = "//"},
]

[[themes]]
name = "purple-night"

primary_col = "#9549FF"
secondary_col = "#FF4958"
tertiary_col = "#B3FF49"
quaternary_col = "#49FFF0"

bg_col = "#1A1B26"
bg_col_light = "#24283B"
fg_col = "#A9B1D6"

[[themes]]
name = "blue-bannana"

primary_col = "#48FFD1"
secondary_col = "#4876FF"
tertiary_col = "#FF4876"
quaternary_col = "#FFD148"

bg_col = "#0A0A40"
bg_col_light = "#11116C"
fg_col = "#9999F8"
```

#### test.conf
```css
test = "This wont be configured" % Even if there are comments

$primary: #9549FF; %%% @replace-col(primary_col)
$secondary: #FF4958; %%% @replace-col(secondary_col)
$tertiary: #B3FF49; %%% @replace-col(tertiary_col)
$quaternary: #49FFF0; %%% @replace-col(quaternary_col)

$background: #1A1B26; %%% @replace('#[\w\d]{6}', bg_col) <-- Equivalent to @replace-col(bg_col)
$background-lighter: #24283B; %%% @replace-col(bg_col_light)
$foreground: #A9B1D6; %%% @replace-col(fg_col)
```

### Running
After downloading this folder, and installing ``rustc`` or ```rustup```, simply running ```cargo r``` will be enough to modify your files, so long as you have configured the TOML file correctly. Errors will be printed to the terminal.
