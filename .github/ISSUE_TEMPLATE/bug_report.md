---
name: Bug report
about: The daemon isn't working properly :'(
title: ''
labels: ''
assignees: ''

---

**Describe the bug**

_A clear and concise description of what the bug is._

**Desktop (please complete the following information):**
- Distro: (e.g: Ubuntu 20.04)
- Desktop Environment: (e.g: GNOME)
- output of `uname -a`: 

**Service Logs**

_Please copy the output of `journalctl -u surface-dial.service` / the daemon's `stdout/stderr`._

When possible, please reproduce the bug with the `RUST_BACKTRACE=1` environment variable set. Having a detailed backtrace will make it easier to diagnose (and hopefully fix) the bug.
