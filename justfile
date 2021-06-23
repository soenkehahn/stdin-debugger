install:
  cargo install --path .

test: install
  echo foo | stdin-debugger
