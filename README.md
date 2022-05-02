> 🚧  code requires a timesheet CSV to run. I removed mine for privacy reasons.

# c4-stats

Rust program to create stats for [Code4rena](https://code4rena.com/leaderboard)'s `cmichel` warden.
Requires exporting a time sheet as a `csv` file (from a time tracking tool like [clockify](https://clockify.me/)) to plot the hours worked & hourly rate charts.

## Raw data

The raw C4 data are snapshots of the official C4 csv files:

- [contests.csv](https://raw.githubusercontent.com/code-423n4/code423n4.com/main/_data/contests/contests.csv)
- [findings.csv](https://raw.githubusercontent.com/code-423n4/code423n4.com/main/_data/findings/findings.csv)

### Output

Creates charts in [`plots`](./plots) directory.
