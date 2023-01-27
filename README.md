# IP Address Lookup with Shodan API
## About
1. Reads a list of IP addresses from input files.
1. Searches for each IP address using the `/shodan/host/{ip}` endpoint of the Shodan API
1. Writes the results for each IP address into a file called `ip_results.json`.

## How to Use

### Configure Settings
To authenticate with the Shodan API, you will need to add a valid API key to the `Settings.toml` file.
1. Open the file `Settings.toml` using Notepad (or any other text editor).
1. replace `yourkeyhere123` between the doublequotes with your own API key.

There are some other configuration settings located in this file that change the output from the API response.
Valid values are `true` or `false`.

### Create Input File
Before using this program, you will need to create a file that contains all IP addresses you want to search.
Open a text file, and put each IP address on its own line with no extra punctuation. See the file
[sample_input.txt](https://github.com/zjtalia/shodan_api/blob/main/sample_input.txt) for an example.

### Usage
The easiest usage of the program is as follows:
1. Download the release package `shodan.zip` [here](https://github.com/zjtalia/shodan/releases).
1. Unzip the file and open the resulting folder.
1. Using the file explorer, drag and drop the file containing your list of IP addresses onto the `shodan.exe` file.
1. Open the resulting `ip_results.json`. For a better viewing experience, use your web browser or VSCode to open this file.

#### A few notes:
- The `ip_results.json` file will be overwritten each time the program is run.
If you want to save the results from a previous search, rename the file or move it to another folder.
- The program accepts multiple input files, all of which are compiled into the same output file.
- You can also run this program from the command line. Example:
```
$ path/to/shodan.exe path/to/sample_input.txt
```
