# DelftBlue Setup Guide
## First Login
You can login to DelftBlue with your NetID and password using `ssh <netid>@login.delftblue.tudelft.nl`
Then first thing to do is change the `.bashrc` so it loads some modules when you log in, this saves a lot of annoyances. Mine looks like this:
```bash
# .bashrc

# Source global definitions
if [ -f /etc/bashrc ]; then
        . /etc/bashrc
fi

# User specific environment
if ! [[ "$PATH" =~ "$HOME/.local/bin:$HOME/bin:" ]]
then
    PATH="$HOME/.local/bin:$HOME/bin:$PATH"
fi
export PATH

# Uncomment the following line if you don't like systemctl's auto-paging feature:
# export SYSTEMD_PAGER=

# User specific aliases and functions
# Load standard modules
module load 2022r2
module load python

# Configure rust
export CARGO_HOME=/scratch/tijslenssen/.cargo
export RUSTUP_HOME=/scratch/tijslenssen/.rustup
. "/scratch/tijslenssen/.cargo/env"

# Load venv
. "/home/tijslenssen/.venv/bin/activate"
```
Re-log afterwards to activate it.

You will also need to install a current version of rust, you can follow https://doc.dhpc.tudelft.nl/delftblue/howtos/rust/ and install it in `/scratch/<NetID>`

I also created a venv with
```
python -m venv .venv
```
to be able to install the requirements of the benchmark runner. 
Again re-log.

### Upload files

You can upload files using:
```
scp -p ${source}  ${target}
```
Check https://doc.dhpc.tudelft.nl/delftblue/Data-transfer-to-DelftBlue/ for more information.
Use that to upload the following, (or you can use `git`):
* your version of *pumpkin* 
* your benchmarking directory. (don't forget to install requirements with `pip install -r requirements.txt`)
* your data files, either `.wcnf` files, or you generate `.wcnf` files on DelftBlue.

You can upload files to your home directory `/home/<netid>` or your scratch directory `/scratch/<netid>`, the scratch directory has more storage, but is not persistent.
## Benchmarking
### Running Benchmarks
To setup `runner.py`, you can modify the version I sent in Discord. I also needed to modify `experiments.py`.
To run a benchmark run: 
* `python runner.py build` to build the experiments
* `python runner.py start` to schedule the jobs
* `python runner.py fetch` to fetch all data
The data will then be in `./data/`

If you want get more data as ouput, for example the solutions, you can modify the parser `pumpkin.py`. For example I added
```python
parser.add_pattern("solution", r"^v 0 ([0-9\-  ]+)$", type=str, flags="M", required=False)
```
to be able to see the solutions.

You can check the current status of your jobs using:
```
sacct -S <date> -u $USER
```
e.g.
```
sacct -S 2023-05-23 -u $USER
```
