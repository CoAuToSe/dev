

import os
import subprocess
import argparse


resultat = subprocess.run(
    "cargo clean",
    cwd="C:\\Users\\Aurélien\\OneDrive\\Bureau\\work\\ownwork\\app_proto",
    # check=True,
    # capture_output=True,
    # text=True,
    # shell=True
)