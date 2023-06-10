#! /usr/bin/env python

from lab.parser import Parser


if __name__ == "__main__":
    parser = Parser()

    parser.add_pattern("status", r"^s ([A-Z]+)$", type=str, flags="M", required=False)
    parser.add_pattern("solve_time", r"solve wall-clock time: ([0-9\.]+)s$", type=float, flags="M", required=True, file="driver.log")
    parser.add_pattern("solution", r"^v 0 ([0-9\-  ]+)$", type=str, flags="M", required=False)
    parser.parse()

