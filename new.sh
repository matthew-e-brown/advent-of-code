#!/usr/bin/env bash

set -e

# We want to CD relative to this script, not from PWD; lets us call
# `../new.sh` from inside the 2025 directory, for example.
cd "$(dirname "${BASH_SOURCE[0]:-$0}")"

# Allow year to be passed on command line.
YEAR="$1"
if [[ -z "$YEAR" ]]; then
    # If not passed, get "the most recent December": if we're currently
    # in December, use this year; otherwise, use last year.
    YEAR="$(date +%Y)"
    if [[ "$(date +%m)" -lt 12 ]]; then
        YEAR="$((YEAR - 1))"
    fi

    # Confirm first:
    read -rp "Year $YEAR? [Yn] " choice
    if [[ ! "$choice" =~ ^[Yy](es)?$ && ! -z "$choice" ]]; then
        >&2 echo "Please provide a year for the package."
        exit 1
    fi
fi

if [[ ! -e "./$YEAR" ]]; then
    >&2 echo "$YEAR folder does not exist yet. Please create."
    >&2 echo "Don't forget to add it to workspace Cargo.toml."
    exit 1
fi

# Now check for the day: either use the one they gave, or look in the
# year's folder and find the earliest missing day.
DAY="$2"
if [[ -z "$DAY" ]]; then
    for i in {01..25}; do
        if [[ ! -d "$YEAR/day-$i" ]]; then
            DAY="$i"
            break
        fi
    done

    # Confirm first:
    read -rp "Day $DAY? [Yn] " choice
    if [[ ! "$choice" =~ ^[Yy](es)?$ && ! -z "$choice" ]]; then
        >&2 echo "Please provide a day for the package."
        exit 1
    fi
fi

# Zero-pad just in case it's not already:
DAY="$(printf '%02d' "$DAY")"

if [[ ! "$DAY" =~ ^[0-9]+$ ]]; then
    >&2 echo "Day $DAY is not a number."
    exit 1
elif [[ "$DAY" -lt 1 || $DAY -gt 25 ]]; then
    >&2 echo "Day $DAY is out of range."
    exit 1
fi

# Now finally make the package:
cd "./$YEAR"
cargo new --bin --name "aoc${YEAR}_${DAY}" "day-${DAY}"

# Append aoc_utils as dependency to Cargo.toml:
echo 'aoc_utils = { version = "*", path = "../../aoc_utils" }' >> "day-${DAY}/Cargo.toml"
