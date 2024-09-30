# Documentation

## Title

| Function Name     | Arguments          | Description    | Notes |
| ----------------- | ------------------ | -------------- | ----- |
| `title/name/set`  | title_name: string | Sets the title |       |
| `title/fg/set`    | color: Color       |                |       |
| `title/bg/set`    | color: Color       |                |       |
| `title/bold`      |                    |                |       |
| `title/italic`    |                    |                |       |
| `title/crossed`   |                    |                |       |
| `title/underline` |                    |                |       |
| `title/reset`     |                    |                |       |
| `title/show`      |                    |                |       |

## Color

| Function Name | Arguments                       | Description               | Notes |
| ------------- | ------------------------------- | ------------------------- | ----- |
| `color/new`   | r: number, g: number, b: number | Creates a new rgb `Color` |       |
| `color`       | color_name: symbol              |                           |       |

## Content

| Function Name         | Arguments      | Description                                | Notes |
| --------------------- | -------------- | ------------------------------------------ | ----- |
| `delay/set`           | ms: number     | Set the delay in ms between each character |       |
| `fg/set`              | color: Color   | Set the foreground color                   |       |
| `bg/set`              | color: Color   | Set the background color                   |       |
| `display/fg/set`      | color: Color   | Set the display foreground color           |       |
| `display/bg/set`      | color: Color   | Set the display background color           |       |
| `display/ac/set`      | color: Color   | Set the display accent color               |       |
| `display/fg/get`      |                | Get the display foreground color           |       |
| `display/bg/get`      |                | Get the display background color           |       |
| `display/ac/get`      |                | Get the display accent color               |       |
| `bold`                |                | Makes the content bold                     |       |
| `italic`              |                | "                                          |       |
| `crossed`             |                | "                                          |       |
| `underline`           |                | "                                          |       |
| `reset`               |                |                                            |       |
| `content/clear`       |                | Clears the displayed content               |       |
| `content/get-raw`     |                | Get the raw text of the displayed content  |       |
| `content/append`      |                | Append text to displayed content           |       |
| `content/scroll/down` |                | Scroll content down one line               |       |
| `content/scroll/up`   |                | Scroll content up one line                 |       |
| `content/scroll/set`  | scroll: number |                                            |       |
| `content/scroll/get`  |                |                                            |       |

## Option

| Function Name   | Arguments                        | Description                                            | Notes |
| --------------- | -------------------------------- | ------------------------------------------------------ | ----- |
| `option/goto`   | content: string, room_id: symbol | Create an option that sets `current_room` to `room_id` |       |
| `option/action` | content: string, action: lambda  | Create an option that activates `action` on activation |       |
| `option/reset`  |                                  | Delete all options currently displayed                 |       |

## Basic

| Function Name   | Arguments                               | Description                         | Notes                                                          |
| --------------- | --------------------------------------- | ----------------------------------- | -------------------------------------------------------------- |
| `post`          | Activate the *post* section of the room |                                     |                                                                |
| `debug`         | message: string                         | Push a message to the debug section |                                                                |
| `quit`          |                                         | Quit the program                    |                                                                |
| `import`        | library: symbol                         | Import a module from std library    |                                                                |
| `string/format` | format: string, ...args: any            | Format a string                     | Use %% to get representation of any object, use %s for strings |
| `string/escape` | char                                    | Escape a char                       | If invalid escape char it just returns the text entered        |

## Listener

| Function Name            | Arguments           | Description                              | Notes                            |
| ------------------------ | ------------------- | ---------------------------------------- | -------------------------------- |
| `listener/keyboard/char` | callback: callable  | Adds a key listener for any key from a-z | returns the listener id (number) |
| `listener/clear`         | listener_id: number | Remove listener                          |                                  |