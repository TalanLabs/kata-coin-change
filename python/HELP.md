# Help

## Running the tests

We use [pytest][pytest: Getting Started Guide] as our website test runner.
You will need to install `pytest` on your development machine if you want to run tests for the Python track locally.
You should also install the following `pytest` plugins:

- [pytest-cache][pytest-cache]
- [pytest-subtests][pytest-subtests]

Extended information can be found in our website [Python testing guide][Python track tests page].


### Running Tests

To run the included tests, navigate to the folder where the exercise is stored using `cd` in your terminal (_replace `{exercise-folder-location}` below with your path_).
Test files usually end in `_test.py`, and are the same tests that run on the website when a solution is uploaded.

Linux/MacOS
```bash
$ cd {path/to/your-folder-location}
```

Windows
```powershell
PS C:\Users\foobar> cd {path\to\your-folder-location}
```

<br>

Next, run the `pytest` command in your terminal, replacing:

Linux/MacOS
```bash
$ python3 -m pytest  coin_change_test.py
==================== 7 passed in 0.08s ====================
```

Windows
```powershell
PS C:\Users\foobar> py -m pytest  coin_change_test.py
==================== 7 passed in 0.08s ====================
```


### Common options
- `-o` : override default `pytest.ini` (_you can use this to avoid marker warnings_)
- `-v` : enable verbose output.
- `-x` : stop running tests on first failure.
- `--ff` : run failures from previous test before running other test cases.

For additional options, use `python3 -m pytest -h` or `py -m pytest -h`.


### Fixing warnings

If you do not use `pytest -o markers=task` when invoking `pytest`, you might receive a `PytestUnknownMarkWarning` for tests that use our new syntax:

```bash
PytestUnknownMarkWarning: Unknown pytest.mark.task - is this a typo?  You can register custom marks to avoid this warning - for details, see https://docs.pytest.org/en/stable/mark.html
```

To avoid typing `pytest -o markers=task` for every test you run, you can use a `pytest.ini` configuration file.
We have made one that can be downloaded from the top level of the Python track directory: [pytest.ini][pytest.ini].

You can also create your own `pytest.ini` file with the following content:

```ini
[pytest]
markers =
    task: A concept exercise task.
```

Placing the `pytest.ini` file in the _root_ or _working_ directory for your Python track exercises will register the marks and stop the warnings.
More information on pytest marks can be found in the `pytest` documentation on [marking test functions][pytest: marking test functions with attributes] and the `pytest` documentation on [working with custom markers][pytest: working with custom markers].

Information on customizing pytest configurations can be found in the `pytest` documentation on [configuration file formats][pytest: configuration file formats].

