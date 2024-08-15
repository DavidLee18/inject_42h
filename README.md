# inject_42h

This is a simple cli which prepends 42 header (from 42 school) to your desired file.
Required arguments are as follows:

- name: your 42 identifier
- email: your 42 email
- path: path to your desired file

Generated output is like this:


This program is actually built to be used as a [Zed](https://zed.dev) Task. Let me tell you how.

1. download cli from release page.
2. edit a Zed Task file (such as `~/.config/zed/tasks.json`) like this:
```json
  [
    {
      "label": "Inject 42 Header",
      "command": "your/path/to/inject_42h",
      "args": ["--name", "<name>", "--email", "<email>", "--path", "$ZED_FILE"]
    }
  ]
```
3. Press Alt + Shift + T to spawn task. your task should be right there.
