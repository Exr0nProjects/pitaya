# Pitaya - Efficiency effeciently

[![Actively Developed](https://img.shields.io/badge/Maintenance%20Level-Actively%20Developed-brightgreen.svg)](https://gist.github.com/cheerfulstoic/d107229326a01ff0f333a1d3476e068d)

A time tracking application built for ergonomics and speed.

## Status

This project is currently in development.

### Features
- [ ] Minimum Viable Product (click a button to start and stop)
- [ ] Basic tags
- [ ] Recursive Tagging

### Time Complexities and Benchmarks
| Action | Complexity | Benchmark |
|--------|------------|-----------|
Start timer | Constant | TBD
Stop timer | Constant | TBD
Check timer duration | Constant | TBD
Add tag to timer | Constant | TBD
Remove tag from timer | Constant | TBD
Check total tag time | Constant | TBD
Sort tags by name | Linearithmic | TBD
Sort tags by duration | Linearithmic | TBD
Sort tags by dependancies | Linear | TBD

## Project Plan

- [ ] Basic time framework
- [ ] Tags
	- [ ] Recursive tagging
	- [ ] Tag coloring by DAG
	- [ ] Fuzzy search over tag names
	- [ ] RNN or transformer net? for tag suggestions
- [ ] Data Visualization
	- [ ] View by tags
		- [ ] Basic
		- [ ] Set logic with tags
- [ ] Interface
	- Platforms
		- [ ] CLI, probably with [tui-rs](https://github.com/fdehau/tui-rs)
		- [ ] Web API with Rocket
		- [ ] Electron/Ionic app?
	- Features
		- [ ] Create/start/stop timers
		- [ ] Create tags
		- [ ] Add tag dependencies
		- [ ] Visualize tag dependencies

