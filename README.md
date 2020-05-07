# Pitaya - Efficiency effeciently

[![Actively Developed](https://img.shields.io/badge/Maintenance%20Level-Actively%20Developed-brightgreen.svg)](https://gist.github.com/cheerfulstoic/d107229326a01ff0f333a1d3476e068d)

A time tracking application built for ergonomics and speed.

## Status

This project is currently in development, kinda. I need to figure out how to serde `DateTime::Duration`s, which might involve working on [this issue](https://github.com/chronotope/chrono/issues/117).

### Features
- [ ] Minimum Viable Product (click a button to start and stop)
- [ ] Basic tags
- [ ] Recursive Tagging

### Performance
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

#### Optimizations
Here are the ideas I'm working with:
- Lazily evaluate tag duration: traverse new timers since previous "sync" when data is dumped
	- This reduces the overhead needed to start/stop timers
- Optimize graph representation for cache size, perhaps employ some of the techniques seen
[here](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.11.3433&rep=rep1&type=pdf)
	- How to store tag DAG? Contiguous vector, separated into chunks and a hashmap of iterators?

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

