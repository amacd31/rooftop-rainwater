# Rooftop Rainfall Calculator

Simple CLI tool to calculate the number of litres of water collected from a
roof for a given amount of rainfall.

## Building

    git clone git@github.com:amacd31/rooftop-rainwater.git
    # or
    git clone https://github.com/amacd31/rooftop-rainwater.git

    cd rooftop-rainwater
    cargo build --release

## Running

Look at options with `--help`:

    $ target/release/rooftop-rainwater --help
    Usage: rooftop-rainwater [OPTIONS] [rainfall]

    Arguments:
      [rainfall]

    Options:
      -a, --area <rooftop-area>          [default: 232.3]
      -C, --coefficient <coefficient>    [default: 0.95]
      -l, --initial-loss <initial-loss>  [default: 0.3]
      -h, --help                         Print help

Calculate litres of water produced from a 195.5 m^2 roof, with an initial loss
of 0.2mm and a general loss coefficient of 0.98. The input rainfall is 15mm:

    $ target/release/rooftop-rainwater -a 195.5 -l 0.2 -C 0.98 15
    2835.532

## Configuration

Alternate defaults can be set in a TOML file located in one of the following
locations (first one found in this list will be used):

  * Directory where the executable is located (e.g. `target/debug/rooftop-rainfall-parameters.toml`)
  * `~/.config/rooftop-rainfall-parameters.toml`

Example configuration (alternately see `rooftop-rainfall-parameters.toml.example`):

    [parameters]
    roof_area = 195.5
    coefficient = 0.98
    initial_loss = 0.2
