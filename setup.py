#!/usr/bin/env python

from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="flitton-fib-rs",
    version="0.1",
    # might have to run ".flitton-fib-rs.flitton_fib_rs"
    rust_extensions=[RustExtension(".flitton_fib_rs", binding=Binding.PyO3)],
    packages=["rust_messaging"],
    classifiers=[
            "License :: OSI Approved :: MIT License",
            "Development Status :: 3 - Alpha",
            "Intended Audience :: Developers",
            "Programming Language :: Python",
            "Programming Language :: Rust",
            "Operating System :: POSIX",
            "Operating System :: MacOS :: MacOS X",
        ],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)
