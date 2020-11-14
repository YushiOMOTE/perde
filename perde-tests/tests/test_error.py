from dataclasses import dataclass, field
from typing import List, Dict, Optional, Union, Tuple, TypeVar
import enum
import perde, perde_json
import pytest

from util import *

@pytest.mark.parametrize("m", FORMATS)
def test_error(m):
    class Plain:
        pass

    with pytest.raises(RuntimeError) as e:
        m.dumps(Plain())
    print(f'{e}')
