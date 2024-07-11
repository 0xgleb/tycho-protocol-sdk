from logging import getLogger
from typing import Any

from tycho.tycho.exceptions import TychoDecodeError
from tycho.tycho.models import EVMBlock, ThirdPartyPool, EthereumToken
from tycho.tycho.utils import decode_tycho_exchange

log = getLogger(__name__)


class ThirdPartyPoolTychoDecoder:
    """ThirdPartyPool decoder for protocol messages from the Tycho feed"""

    def __init__(self, adapter_contract: str, minimum_gas: int, hard_limit: bool):
        self.adapter_contract = adapter_contract
        self.minimum_gas = minimum_gas
        self.hard_limit = hard_limit

    def decode_snapshot(
        self,
        snapshot: dict[str, Any],
        block: EVMBlock,
        tokens: dict[str, EthereumToken],
    ) -> tuple[dict[str, ThirdPartyPool], list[str]]:
        pools = {}
        failed_pools = []
        for snap in snapshot.values():
            try:
                pool = self.decode_pool_state(snap, block, tokens)
                pools[pool.id_] = pool
            except TychoDecodeError as e:
                log.error(f"Failed to decode third party snapshot: {e}")
                failed_pools.append(snap["component"]["id"])
                continue

        return pools, failed_pools

    def decode_pool_state(
        self, snap: dict, block: EVMBlock, tokens: dict[str, EthereumToken]
    ) -> ThirdPartyPool:
        component = snap["component"]
        exchange, _ = decode_tycho_exchange(component["protocol_system"])

        try:
            tokens = tuple(tokens[t] for t in component["tokens"])
        except KeyError as e:
            raise TychoDecodeError("Unsupported token", pool_id=component["id"])

        balances = self.decode_balances(snap, tokens)
        optional_attributes = self.decode_optional_attributes(component, snap)

        return ThirdPartyPool(
            id_=optional_attributes.pop("pool_id", component["id"]),
            tokens=tokens,
            balances=balances,
            block=block,
            spot_prices={},
            trading_fee=Decimal("0"),
            exchange=exchange,
            adapter_contract_name=self.adapter_contract,
            minimum_gas=self.minimum_gas,
            hard_sell_limit=self.hard_limit,
            db_type=DatabaseType.tycho,
            trace=True,
            **optional_attributes,
        )

    @staticmethod
    def decode_optional_attributes(component, snap):
        # Handle optional state attributes
        attributes = snap["state"]["attributes"]
        pool_id = attributes.get("pool_id") or component["id"]
        balance_owner = attributes.get("balance_owner")
        stateless_contracts = {}
        index = 0
        while f"stateless_contract_addr_{index}" in attributes:
            address = attributes[f"stateless_contract_addr_{index}"]
            code = attributes[f"stateless_contract_code_{index}"]
            stateless_contracts[address] = code
            index += 1
        return {
            "balance_owner": balance_owner,
            "pool_id": pool_id,
            "stateless_contracts": stateless_contracts,
        }

    def decode_balances(self, snap, tokens):
        balances = {}
        for addr, balance in snap["state"]["balances"].items():
            checksum_addr = addr
            token = next(t for t in tokens if t.address == checksum_addr)
            balances[token.address] = token.from_onchain_amount(
                int(balance, 16)  # balances are big endian encoded
            )
        return balances

    def apply_update(
        self,
        pool: ThirdPartyPool,
        pool_update: dict[str, Any],
        balance_updates: dict[str, Any],
        block: EVMBlock,
    ) -> ThirdPartyPool:
        # check for and apply optional state attributes
        attributes = pool_update.get("updated_attributes")
        if attributes:
            # TODO: handle balance_owner and stateless_contracts updates
            pass

        for addr, balance_msg in balance_updates.items():
            token = [t for t in pool.tokens if t.address == addr][0]
            balance = int(balance_msg["balance"], 16)  # balances are big endian encoded
            pool.balances[token.address] = token.from_onchain_amount(balance)
        pool.block = block
        # we clear simulation cache and overwrites on the pool and trigger a recalculation of spot prices
        pool.clear_all_cache()
        return pool
