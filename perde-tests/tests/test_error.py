import pytest
from util import FORMATS


@pytest.mark.parametrize("m", FORMATS)
def test_error(m):
    class Plain:
        pass

    with pytest.raises(Exception) as e:
        m.dumps(Plain())
    print(f'{e}')
