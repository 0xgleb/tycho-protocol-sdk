import datetime
from enum import Enum, IntEnum, auto
from typing import Union

from pydantic import BaseModel, Field

Address = str


class Blockchain(Enum):
    ethereum = "ethereum"
    arbitrum = "arbitrum"
    polygon = "polygon"
    zksync = "zksync"


class EVMBlock(BaseModel):
    id: int
    ts: datetime.datetime = Field(default_factory=datetime.datetime.utcnow)
    hash_: str


class EthereumToken(BaseModel):
    symbol: str
    address: str
    decimals: int
    gas: Union[int, list[int]] = 29000


class DatabaseType(Enum):
    # Make call to the node each time it needs a storage (unless cached from a previous call).
    rpc_reader = "rpc_reader"
    # Connect to Tycho and cache the whole state of a target contract, the state is continuously updated by Tycho.
    # To use this we need Tycho to be configured to index the target contract state.
    tycho = "tycho"


class Capability(IntEnum):
    SellSide = auto()
    BuySide = auto()
    PriceFunction = auto()
    FeeOnTransfer = auto()
    ConstantPrice = auto()
    TokenBalanceIndependent = auto()
    ScaledPrice = auto()
