Taurus - Agent Launcher
=======================

What is this?
-------------
Taurus runs and manages several coding agents (Claude Code and others) as
terminal tabs in one window. Every agent starts in the folder you pick, so it
starts with exactly the context that folder carries.

Requirements
------------
  - An agent CLI on your PATH, for example the Claude Code CLI (`claude`).
  - Windows: the WebView2 runtime (preinstalled on Windows 11).

First start
-----------
Pick or create a specialist or work process in the sidebar, choose a folder and
hit Start. The agent opens as a tab.

Where is the configuration?
---------------------------
  - Windows: %APPDATA%\Taurus
  - macOS:   ~/Library/Application Support/Taurus
  Projects, hosts and sessions are stored there as JSON, per user.

More
----
Documentation and releases: https://github.com/astetic-dev/taurus
