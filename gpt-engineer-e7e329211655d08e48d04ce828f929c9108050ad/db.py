
from dataclasses import dataclass
import os
from pathlib import Path

import time

class DB:
    def __init__(self, path):
        self.path = Path(path).absolute()
        os.makedirs(self.path, exist_ok=True)

    def __getitem__(self, key):
        with open(self.path / key) as f:
            time.sleep(1)
            return f.read()

    def __setitem__(self, key, val):
        with open(self.path / key, 'w') as f:
            
            time.sleep(1)
            f.write(val)


# dataclass for all dbs:
@dataclass
class DBs:
    memory: DB
    logs: DB
    identity: DB
    input: DB
    workspace: DB