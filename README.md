# Pitaya - Efficiency effeciently

<!--[![Actively Developed](https://img.shields.io/badge/Maintenance%20Level-Actively%20Developed-brightgreen.svg)](https://gist.github.com/cheerfulstoic/d107229326a01ff0f333a1d3476e068d)-->
[![Not Maintained](https://img.shields.io/badge/Maintenance%20Level-Not%20Maintained-yellow.svg)](https://gist.github.com/cheerfulstoic/d107229326a01ff0f333a1d3476e068d)

A time tracking application built for ergonomics and speed.

## Status

This project is currently in development, kinda.

~~I need to figure out how to serde `DateTime::Duration`s, which might involve working on [this issue](https://github.com/chronotope/chrono/issues/117).~~

I need to figure out how to allow everybody to mutate things that the user object owns, possibly through a load of `Arc`s and `RwLock`s.

I'm currently taking a break from Pitaya to work on other projects. I'll be back next time I get annoyed enough at Toggl.

### Why Pitaya

This project was inspired by [Toggl](https://toggl.com/).
However, I've found Toggl to be a pain to use, because it is absurdly laggy and slow to start and stop timers, and because tagging is so manual that it's a pain.
I track my time because I want insights, but it's so difficult to get good enough resolution on tags to look at your data well that the tagging system becomes obsolete--you either spend too much time creating and managing tags to timers, or your tags aren't specific enough to really offer much insight.

This project attempts to bring two features to the Toggl concept: recursive tagging and arbitrary collections.

Recursive tagging is a way to group tags to make them easier to manage. One of the biggest problems with Toggl is that when you create a new timer, you have to add each relevant tag by hand.
When you have tens or hundreds of tags, this quickly gets unwieldy.
However, many timers share a similar set of tags--`meeting` which means meeting with another human and discussing things might often be paired with `synchronous`, which means real-time communication over voice or in real life.
Recursive tagging allows you to create a link between these tags; if all of your meetings are synchronous, then you can add `synchronous` as a subtag of `meeting`, so whenever you add `meeting` to a timer then `synchronous` is added automatically.
Additionally, creating this DAG of tags opens a whole other world of possibility. We can use the dependency structure to color timers based on their tag makeup, do more advanced set logic, and so on.

Arbitrary collections ask: "Why should we only track time?" They are more of a shower-thought kind of feature but could prove to be invaluable.
The premise is to add other "collectibles" to each time, which could be subjective like intensity ("How hard did I work during this period of time? How productive was I?") or alignment ("How important was this to my long term goals?")
or objective like time, strenuosity ("What was my median/max heart rate?"), or focus ("How many times did I change the focused application? How much typing/clicking/mouse movement/network transfer occurred?").
As you can imagine, this would bring in whole new dimensions of analysis, which could lead to surprising insights such as "I type the fastest at midnight" or "I tend to eat more calories on Fridays" or "I get more REM sleep after eating more and working out".
Collections have some kinks that need to be worked out: how do you let the user define a default or accumulator? how do stats that require multiple types of data work? (median, stddev, curve fits would have to hold an array instead of just a value?) and so on.

Finally, Pitaya aims to be fast and extensible. The end product expects to be thoroughly benchmarked and have a fully featured web API. And of course, you can always build from source to tweak something that you don't like.
And please submit an issue to request a feature or a PR after implementing something cool!

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

