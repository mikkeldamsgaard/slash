---
layout: single
title: Installation
permalink: /installation/
---
## Build and install
Build instuctions:
* install [rust](https://www.rust-lang.org/tools/install)
* clone it locally ``git clone https://github.com/mikkeldamsgaard/slash.git``
* cd slash
* Run cargo: ``cargo build --release``
* Optionally, install it: ``sudo cp target/release/slash /usr/local/bin/`` or
  maybe more dynamically, just link it for easy updates: ``sudo ln -s target/release/slash /usr/local/bin/``
