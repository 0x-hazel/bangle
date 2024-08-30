# Bangle!

Bangle! is a source-available (for now) and (ideally) lightweight tool for applying and customizing
DuckDuckGo style [bangs](https://duckduckgo.com/bangs) for whatever search engine or
site you want to use. It can act as a bookmarking tool/link shortener as well
(e.g. set !gh to navigate to your github dashboard)

> [!WARNING]
> Bangle! is alpha software and is not expected to see use during this phase, other than for development.
>
> While you are welcome to do so, please note that there may be breaking changes in future.


## Hosting

Bangle! has been designed to run on the [Shuttle](https://www.shuttle.rs/) hosting platform,
and can be run locally using shuttle tooling with `cargo shuttle run`.
Some modification will be required to host independently of shuttle, but it shouldn't be much.

## Using

To use, navigate to a running Bangle! instance and create a new bangle list.
Once you do, you will be provided an interface to modify your bangle list, including changing the default search engine and adding or removing bangs.

### Keys

When you enter your bangle list, you are provided with one or two keys, depending on your access level. The most important key, the one you are given when you first create your list, is the edit key. Accessing your list using this key will allow editing of the list, so it shouldn't be given out to just anyone.

Which is what the read key is for, sharing your bangle list with others or using on shared devices. Accessing your list with the read key doesn't allow any editing of the contents of your list.

The intention of the key system is to prevent other people accessing your bangs
(which would be a violation of privacy, however much trial-and-error would be required to extract information in this manner),
and to allow you to share your bangle list with others, without them also being able to modify it on your behalf.

To make a search with a Bangle! instance, access `http[s]://<bangle instance>/!?list=<list>&key=<key>&query=<query>`, either manually or by setting it as a search engine in your browser (this is simple to do on firefox mobile but not on browser, automatic settings tbd). You can also do this using the search bar on the list page.

All Bangle! lists have a hardcoded `!bangs` bang, that brings you to the list page, to view or edit, depending on which key you use.

## Roadmap

- [ ] Refactoring
    - [X] Split sections out into their own modules
    - [ ] Commenting
    - [ ] General refactoring to make the code nicer
- [ ] Further Features
    - [ ] Common default search engine options
    - [ ] Showing page name/favicon for bangs
    - [ ] Edit bang urls
- [ ] Polish
    - [ ] Make it look nice
    - [ ] Make it nice to use
    - [ ] Some sort of messaging system for errors
- [ ] Bugfixing
    - [X] Adding a new bang doesn't validate the key
    - [X] Bangs that don't exist are silently elided from the query
    - [ ] There are probably many undiscovered bugs