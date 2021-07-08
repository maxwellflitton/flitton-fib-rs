import argparse
import yaml
import os
from pprint import pprint

# from .flitton_fib_rs import fibonacci_number


def config_number_command() -> None:
    parser = argparse.ArgumentParser(
        description='Calculate Fibonacci numbers using a config file')
    parser.add_argument('--path', action='store',
                        type=str, required=True,
                        help="path to config file")
    args = parser.parse_args()

    with open(str(os.getcwd()) + "/" + args.path) as f:
        config_data: dict = yaml.safe_load(f)
    pprint(config_data)


config_number_command()
