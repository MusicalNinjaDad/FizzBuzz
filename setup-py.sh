#!/bin/bash
rm -rf .venv \
&& python3 -m venv .venv \
&& . .venv/bin/activate \
&& python3 -m ensurepip \
&& pip install -e .[dev] \
&& python3 -m pytest \
&& ruff check .