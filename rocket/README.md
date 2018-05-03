# Rocket with r2d2 mysql


## Installation

```bash
$ rustup default nightly

$ rustup override set nightly

$ rustup update && cargo update
```

## Issue

Unable to complete setup because of error:

```bash
A feature attribute named a feature that has been removed.

Erroneous code example:

```
#![feature(managed_boxes)] // error: feature has been removed
```

Delete the offending feature attribute.
(END)
```