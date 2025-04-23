// SPDX-License-Identifier: AGPL-3.0-or-later
pragma solidity ^0.8.13;

import {ISwapAdapter} from "src/interfaces/ISwapAdapter.sol";
import {OrderV3, EvaluableV3, IO, IInterpreterV3, IInterpreterStoreV2, IOrderBookV4, Quote, SignedContextV1} from "rain.orderbook.interface/IOrderBookV4.sol";

/// @title TemplateSwapAdapter
/// @dev This is a template for a swap adapter.
/// Rename it to your own protocol's name and implement it according to the
/// specification.
contract RaindexSwapAdapter is ISwapAdapter {
    OrderV3 public hardcodedOrder;
    IOrderBookV4 public orderBook;

    constructor(address _orderBook) {
        orderBook = IOrderBookV4(_orderBook);

        bytes
            memory bytecode = hex"00000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000068155a43676e0000088436f6f6c646f776e000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001043561a8829300000000000000000000000000000000000000000000000000000002386f26fc1000000000000000000000000000000000000000000000000000000b1a2bc2ec500000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008102000000781d070007031000014911000001100000001000002b12000001100001001000011a100000201200001d0200001a100000031000014a020000001000011a1000004712000001100002001000022e12000000100003011000030110000447120000011000032713000000100004011000043b1200000110000500000000";

        EvaluableV3 memory evaluable = EvaluableV3(
            IInterpreterV3(0xFA4989F5D49197FD9673cE4B7Fe2A045A0F2f9c8),
            IInterpreterStoreV2(0x783b82f0fBF6743882072AE2393B108F5938898B),
            bytecode
        );

        IO[] memory validInputs = new IO[](1);
        validInputs[0] = IO(
            0x655A51e6803faF50D4acE80fa501af2F29C856cF,
            18,
            99546561798188533225209997741243117725332507521743522550239356086338411906792
        );

        IO[] memory validOutputs = new IO[](1);
        validOutputs[0] = IO(
            0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913,
            6,
            99546561798188533225209997741243117725332507521743522550239356086338411906792
        );

        hardcodedOrder = OrderV3(
            0x18a62a3Ac2CA9F775A4A12380EdA03245270b73E,
            evaluable,
            validInputs,
            validOutputs,
            0x088b4841152166154a114b17795f79ce41b3a5c84b07a77d2da39210ed5b7d79
        );
    }

    /// @inheritdoc ISwapAdapter
    function price(
        bytes32 poolId,
        address sellToken,
        address buyToken,
        uint256[] memory specifiedAmounts
    ) external view override returns (Fraction[] memory prices) {
        require(
            sellToken == hardcodedOrder.validInputs[0].token,
            "Only hardcoded order is supported"
        );
        require(
            buyToken == hardcodedOrder.validOutputs[0].token,
            "Only hardcoded order is supported"
        );

        prices = new Fraction[](specifiedAmounts.length);

        Quote memory quoteConfig = Quote(
            hardcodedOrder,
            0,
            0,
            new SignedContextV1[](0)
        );

        (bool exists, uint256 outputMax, uint256 ioRatio) = orderBook.quote(
            quoteConfig
        );

        if (!exists) {
            revert Unavailable("Order does not exist");
        }

        revert NotImplemented("RaindexSwapAdapter.price");
    }

    function swap(
        bytes32 poolId,
        address sellToken,
        address buyToken,
        OrderSide side,
        uint256 specifiedAmount
    ) external returns (Trade memory trade) {
        revert NotImplemented("TemplateSwapAdapter.swap");
    }

    function getLimits(
        bytes32 poolId,
        address sellToken,
        address buyToken
    ) external returns (uint256[] memory limits) {
        revert NotImplemented("TemplateSwapAdapter.getLimits");
    }

    function getCapabilities(
        bytes32 poolId,
        address sellToken,
        address buyToken
    ) external returns (Capability[] memory capabilities) {
        revert NotImplemented("TemplateSwapAdapter.getCapabilities");
    }

    function getTokens(
        bytes32 poolId
    ) external returns (address[] memory tokens) {
        revert NotImplemented("TemplateSwapAdapter.getTokens");
    }

    function getPoolIds(
        uint256 offset,
        uint256 limit
    ) external returns (bytes32[] memory ids) {
        revert NotImplemented("TemplateSwapAdapter.getPoolIds");
    }
}
