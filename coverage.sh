#!/bin/bash

cargo llvm-cov --ignore-filename-regex '(callbacks.rs|views.rs|events.rs|storage.rs)' --open
