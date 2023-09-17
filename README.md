# gm_rant
[Rant](https://github.com/rant-lang/rant) template language right in Garry's Mod

Currently this module is built with Rant 4 (4.0.0-alpha.33)

## API
```lua
-- Compiles rant code into program
-- Returns RantProgram on success
-- On failure returns nil and error string
rant.compile(code: string) -> RantProgram/nil, nil/string

-- Compiles and runs rant code. Returns program output or error
-- Arguments must consist of table with string key and any value
rant.run(code: string/RantProgram, args?: table) -> RantOutput/nil, nil/string
```