Redtime: Simple tool to time track activities in Redmine
========================================================

- Florian Dupeyron <florian.dupeyron@mugcat.fr>
- August 2023

Redtime is a simple tool that is intended to add time tracking
entries in [Redmine](https://www.redmine.org). It is also a learning
project around the Rust language.


Test this project
=====================

Dependencies
------------

- Stable rust toolchain with `cargo`;
- Docker if you want to use the try the test server;
- [`just`](https://just.systems). Can be installed using `cargo install just`.

Procedure
---------

This repo comes with a preconfigured redmine database. Initialize it with:

```
just redmine-init
```

This should create a test redmine server, and initialize the necessary private configuration file, `.env`. This file contains
the following environment variables:

- `REDMINE_API_KEY`: API key to access Redmine through the REST API from testing user, `test-user`;
- `REDMINE_URL`: URL to the launched test redmine server.

The test datase contains a test project named `test-project`, with some simple configuration. Two users are initialized:

- `test-user`, with password `test-user123`, which represents a test user;
- `admin`, with password `admin123`, which is the server's administrator.

The used API key for testing is the one from `test-user`.

The `redtime.toml` Contains the configuration for `test-project`:

```toml
[general]
project_identifier="test-project" # Identifier of the project
add_work_hours=3.75               # Number of added work hours
logfile="time_entries.csv"        # Path to output CSV log file

[status]
new="New"                         # Status name for detecting new tasks
working="Ongoing"                 # Status name set when starting a new task
```

Comments are self explanatory. Please note that the status names are fetched from the server, and are case sensitive.


Next, you can run the app:

```bash
cargo run
```

Please note that you may wait some time for the server to be init. to be able to run this command.

If it goes well, you should be asked what issue you want to access:

```
? Please choose an issue  
> Test HW task 1
  Test SW task 2
  Test SW task 1
[↑↓ to move, enter to select, type to filter]
```

Listed issues are *opened issues* that are affected to the user corresponding to the used API key.

Next, you will be asked what type of activity you did:

```
> Please choose an issue Test SW task 1
? Please select a type of activity  
> Analysis
  Design
  Development
  Testing
```

Next, you will be asked to comment what did you do:

```
> Please choose an issue Test SW task 1
> Please select a type of activity Development
? What is the purpose of your work? Did some hard working on the feature.
```

Then, the tool will do its job:

```
> Please choose an issue Test SW task 1
> Please select a type of activity Development
> What is the purpose of your work? Did some hard working on the feature. 
> Task status is "New", changing to "Ongoing"
> Add time tracking entry: 3.75 hours, comment: Did some hard working on the feature. 
> Log entry to time_entries.csv
```

You can see here that a time tracking activity of `3.75` hours have been added the issue, with the
specified parameters. Please note that the added time is set in the `redtime.toml` configuration file.

You can see the logged entry in the `time_entries.csv` file:

```
2023-08-05 15:55:49;PM;Test SW task 1;Development;3.75;Did some hard working on the feature.
```

This is some tool to be able to track what time entries did you correctly log.

The redmine server can be stopped using:

```
just redmine-stop
```

and can be removed using:

```
just redmine-remove
```
