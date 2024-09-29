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

| Function Name     | Arguments    | Description                                | Notes |
| ----------------- | ------------ | ------------------------------------------ | ----- |
| `delay/set`       | ms: number   | Set the delay in ms between each character |       |
| `fg/set`          | color: Color | Set the foreground color                   |       |
| `bg/set`          | color: Color | Set the background color                   |       |
| `bold`            |              | Makes the content bold                     |       |
| `italic`          |              | "                                          |       |
| `crossed`         |              | "                                          |       |
| `underline`       |              | "                                          |       |
| `reset`           |              |                                            |       |
| `content/clear`   |              | Clears the displayed content               |       |
| `content/get-raw` |              | Get the raw text of the displayed content  |       |

## Option

| Function Name   | Arguments                        | Description                                            | Notes |
| --------------- | -------------------------------- | ------------------------------------------------------ | ----- |
| `option/goto`   | content: string, room_id: symbol | Create an option that sets `current_room` to `room_id` |       |
| `option/action` | content: string, action: lambda  | Create an option that activates `action` on activation |       |
| `option/reset`  |                                  | Delete all options currently displayed                 |       |

## Basic

| Function Name | Arguments                               | Description                         | Notes |
| ------------- | --------------------------------------- | ----------------------------------- | ----- |
| `post`        | Activate the *post* section of the room |                                     |       |
| `debug`       | message: string                         | Push a message to the debug section |       |