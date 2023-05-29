from pathlib import Path
import platform

from lab.environments import LocalEnvironment
from lab.experiment import Experiment

from framework.environments import DelftBlueEnvironment
from framework.experiment import SolverExperiment
from framework.reports import CsvReport


SCRIPT_DIR = Path(__file__).parent.resolve()

# Indicates whether the experiment is run locally or on a cluster.
RUN_ON_CLUSTER = platform.node().startswith("login")


# The attributes which are put into the report. These are gathered by parsers 
# during the 'fetch' step or the 'parse_again' step.
ATTRIBUTES = [
    "benchmark",
    "solve_time",
    "status",
]

# Describe the environment on which the experiment is run. By default, the
# following are implemented:
#   - lab.environments.LocalEnvironment
#   - lab.environments.SlurmEnvironment
#   - framework.environments.DelftBlueEnvironment (a SlurmEnvironment 
#         specifically for DelftBlue)
ENV = LocalEnvironment(processes=4) if not RUN_ON_CLUSTER \
        else DelftBlueEnvironment(...)

# The folder in which experiment files are generated.
EXP_PATH = SCRIPT_DIR / "data" / "exp" if not RUN_ON_CLUSTER \
        else ...

# The parsers in the 'framework/parsers' directory, which you want to apply to
# the logs of the experiment.
# Values in this array should be just the stem of the file, so no directories
# and no '.py' suffix.
PARSERS = ["pumpkin"]


def add_runs(experiment: Experiment):
    # TODO: Add the runs required for your experiment.
    pass


def runner():
    exp = SolverExperiment(environment=ENV, path=EXP_PATH)

    for parser in PARSERS:
        exp.add_parser(f"framework/parsers/{parser}.py")

    add_runs(exp)

    exp.add_step("build", exp.build)
    exp.add_step("start", exp.start_runs)
    exp.add_fetcher(name="fetch")
    exp.add_parse_again_step()

    # Add reporting steps to the experiment. By default, we give a CSV report of 
    # all the specified attributes. However, you can add more reports or replace
    # the one given here.
    exp.add_report(CsvReport(attributes=ATTRIBUTES), outfile="report.csv")

    exp.run_steps()


if __name__ == "__main__":
    runner()
