from pathlib import Path
import doctest
import os
import sys


PY39 = doctest.register_optionflag("PY39")

runner = doctest.DocTestRunner()
parser = doctest.DocTestParser()

for path in Path(".").rglob("*.md"):
    with open(path) as f:
        print(f"doctest {path}")
        name = os.path.basename(path)
        test = parser.get_doctest(f.read(), {}, name, path, 0)
        for example in test.examples:
            if example.options.get(PY39) and sys.version_info < (3, 9):
                example.options[doctest.SKIP] = True
        runner.run(test)
