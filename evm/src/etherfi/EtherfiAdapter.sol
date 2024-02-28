// SPDX-License-Identifier: AGPL-3.0-or-later
pragma experimental ABIEncoderV2;
pragma solidity ^0.8.13;

import {IERC20, ISwapAdapter} from "src/interfaces/ISwapAdapter.sol";
import {SafeERC20} from
    "openzeppelin-contracts/contracts/token/ERC20/utils/SafeERC20.sol";

/// @title Etherfi Adapter
/// @dev This contract supports the following swaps: ETH->eETH, wETH<->eETH,
/// ETH->weETH
contract EtherfiAdapter is ISwapAdapter {
    using SafeERC20 for IERC20;

    IWeEth weEth;
    IeEth eEth;
    ILiquidityPool public liquidityPool;

    constructor(address _weEth) {
        weEth = IWeEth(_weEth);
        eEth = weEth.eETH();
        liquidityPool = eEth.liquidityPool();
    }

    /// @dev Check if tokens in input are supported by this adapter
    modifier checkInputTokens(address sellToken, address buyToken) {
        if (sellToken == buyToken) {
            revert Unavailable("This pool only supports eETH, weEth and ETH");
        }
        if (
            sellToken != address(weEth) && sellToken != address(eEth)
                && sellToken != address(0)
        ) {
            revert Unavailable("This pool only supports eETH, weEth and ETH");
        }
        if (buyToken != address(weEth) && buyToken != address(eEth)) {
            revert Unavailable("This pool only supports eETH, weEth and ETH");
        }
        _;
    }

    /// @dev enable receive as this contract supports ETH
    receive() external payable {}

    /// @inheritdoc ISwapAdapter
    function price(
        bytes32,
        IERC20 _sellToken,
        IERC20 _buyToken,
        uint256[] memory _specifiedAmounts
    )
        external
        view
        override
        checkInputTokens(address(_sellToken), address(_buyToken))
        returns (Fraction[] memory _prices)
    {
        _prices = new Fraction[](_specifiedAmounts.length);
        address sellTokenAddress = address(_sellToken);
        address buyTokenAddress = address(_buyToken);

        for (uint256 i = 0; i < _specifiedAmounts.length; i++) {
            _prices[i] = getPriceAt(
                sellTokenAddress, buyTokenAddress, _specifiedAmounts[i]
            );
        }
    }

    function swap(
        bytes32,
        IERC20 sellToken,
        IERC20 buyToken,
        OrderSide side,
        uint256 specifiedAmount
    )
        external
        override
        checkInputTokens(address(sellToken), address(buyToken))
        returns (Trade memory trade)
    {
        if (specifiedAmount == 0) {
            return trade;
        }

        address sellTokenAddress = address(sellToken);
        address buyTokenAddress = address(buyToken);
        uint256 gasBefore = gasleft();
        if (sellTokenAddress == address(0)) {
            if (buyTokenAddress == address(eEth)) {
                trade.calculatedAmount = swapEthForEeth(specifiedAmount, side);
            } else {
                trade.calculatedAmount = swapEthForWeEth(specifiedAmount, side);
            }
        } else {
            if (sellTokenAddress == address(eEth)) {
                trade.calculatedAmount = swapEthForWeEth(specifiedAmount, side);
            } else {
                trade.calculatedAmount = swapWeEthForEeth(specifiedAmount, side);
            }
        }
        trade.gasUsed = gasBefore - gasleft();
        if (side == OrderSide.Sell) {
            trade.price = getPriceAt(
                sellTokenAddress, buyTokenAddress, specifiedAmount
            );
        } else {
            trade.price = getPriceAt(
                sellTokenAddress, buyTokenAddress, trade.calculatedAmount
            );
        }
    }

    /// @inheritdoc ISwapAdapter
    function getLimits(bytes32, IERC20 sellToken, IERC20 buyToken)
        external
        view
        override
        checkInputTokens(address(sellToken), address(buyToken))
        returns (uint256[] memory limits)
    {
        address sellTokenAddress = address(sellToken);
        address buyTokenAddress = address(buyToken);
        limits = new uint256[](2);

        /// @dev only limit on Etherfi is applied on deposits(eth->eETH), and is type(uint128).max
        /// but we use the same amount for the others to underestimate
        limits[0] = type(uint128).max;
        limits[1] = limits[0];
    }

    function getCapabilities(bytes32 poolId, IERC20 sellToken, IERC20 buyToken)
        external
        returns (Capability[] memory capabilities)
    {
        revert NotImplemented("TemplateSwapAdapter.getCapabilities");
    }

    /// @inheritdoc ISwapAdapter
    function getTokens(bytes32)
        external
        view
        override
        returns (IERC20[] memory tokens)
    {
        tokens = new IERC20[](3);
        tokens[0] = IERC20(address(0));
        tokens[1] = IERC20(address(eEth));
        tokens[2] = IERC20(address(weEth));
    }

    /// @inheritdoc ISwapAdapter
    function getPoolIds(uint256, uint256)
        external
        returns (bytes32[] memory ids)
    {
        ids = new bytes32[](1);
        ids[0] = bytes20(address(liquidityPool));
    }

    /// @notice Swap ETH for eETH
    /// @param amount amountIn or amountOut depending on side
    function swapEthForEeth(uint256 amount, OrderSide side)
        internal
        returns (uint256)
    {
        if (side == OrderSide.Buy) {
            uint256 amountIn = getAmountIn(address(0), address(eEth), amount);
            uint256 receivedAmount = liquidityPool.deposit{value: amountIn}();
            IERC20(address(eEth)).safeTransfer(
                address(msg.sender), receivedAmount
            );
            return amountIn;
        } else {
            uint256 receivedAmount = liquidityPool.deposit{value: amount}();
            IERC20(address(eEth)).safeTransfer(
                address(msg.sender), receivedAmount
            );
            return receivedAmount;
        }
    }

    /// @notice Swap ETH for weEth
    /// @param amount amountIn or amountOut depending on side
    function swapEthForWeEth(uint256 amount, OrderSide side)
        internal
        returns (uint256)
    {
        if (side == OrderSide.Buy) {
            uint256 amountIn = getAmountIn(address(0), address(weEth), amount);
            IERC20(address(eEth)).approve(address(weEth), amountIn);
            uint256 receivedAmountEeth =
                liquidityPool.deposit{value: amountIn}();
            uint256 receivedAmount = weEth.wrap(receivedAmountEeth);
            IERC20(address(weEth)).safeTransfer(
                address(msg.sender), receivedAmount
            );
            return amountIn;
        } else {
            IERC20(address(eEth)).approve(address(weEth), amount);
            uint256 receivedAmountEeth = liquidityPool.deposit{value: amount}();
            uint256 receivedAmount = weEth.wrap(receivedAmountEeth);
            IERC20(address(weEth)).safeTransfer(
                address(msg.sender), receivedAmount
            );
            return receivedAmount;
        }
    }

    /// @notice Swap eETH for weETH
    /// @param amount amountIn or amountOut depending on side
    function swapEethForWeEth(uint256 amount, OrderSide side)
        internal
        returns (uint256)
    {
        if (side == OrderSide.Buy) {} else {}
    }

    /// @notice Swap weETH for eEth
    /// @param amount amountIn or amountOut depending on side
    function swapWeEthForEeth(uint256 amount, OrderSide side)
        internal
        returns (uint256)
    {
        if (side == OrderSide.Buy) {} else {}
    }

    /// @notice Get swap price
    /// @param sellToken token to sell
    /// @param buyToken token to buy
    function getPriceAt(address sellToken, address buyToken, uint256 amount)
        internal
        view
        returns (Fraction memory)
    {
        if (sellToken == address(0)) {
            if (buyToken == address(eEth)) {
                return Fraction(liquidityPool.sharesForAmount(amount), amount);
            } else {
                uint256 eEthOut = liquidityPool.sharesForAmount(amount);
                return Fraction(liquidityPool.sharesForAmount(eEthOut), amount);
            }
        } else if (sellToken == address(eEth)) {
            return Fraction(liquidityPool.sharesForAmount(amount), amount);
        } else {
            return Fraction(liquidityPool.amountForShare(amount), amount);
        }
    }

    /// @notice Get amountIn for swap functions with OrderSide buy
    function getAmountIn(address sellToken, address buyToken, uint256 amountOut)
        internal
        view
        returns (uint256)
    {
        if (sellToken == address(0)) {
            if (buyToken == address(eEth)) {
                return liquidityPool.amountForShare(amountOut);
            } else {
                uint256 ethRequiredForEeth =
                    liquidityPool.amountForShare(amountOut);
                return liquidityPool.amountForShare(ethRequiredForEeth);
            }
        } else if (sellToken == address(eEth)) {
            // eEth-weEth
            return weEth.getEETHByWeETH(amountOut);
        } else {
            // weEth-eEth
            return weEth.getWeETHByeETH(amountOut);
        }
    }
}

interface IWithdrawRequestNFT {
    function requestWithdraw(
        uint96 amountOfEEth,
        uint96 shareOfEEth,
        address recipient,
        uint256 fee
    ) external payable returns (uint256);

    function getClaimableAmount(uint256 tokenId)
        external
        view
        returns (uint256);

    function claimWithdraw(uint256 tokenId) external;
}

interface ILiquidityPool {
    function numPendingDeposits() external view returns (uint32);
    function totalValueOutOfLp() external view returns (uint128);
    function totalValueInLp() external view returns (uint128);
    function getTotalEtherClaimOf(address _user)
        external
        view
        returns (uint256);
    function getTotalPooledEther() external view returns (uint256);
    function sharesForAmount(uint256 _amount) external view returns (uint256);
    function sharesForWithdrawalAmount(uint256 _amount)
        external
        view
        returns (uint256);
    function amountForShare(uint256 _share) external view returns (uint256);

    function deposit() external payable returns (uint256);
    function deposit(address _referral) external payable returns (uint256);
    function deposit(address _user, address _referral)
        external
        payable
        returns (uint256);

    function requestWithdraw(address recipient, uint256 amount)
        external
        returns (uint256);
    function withdrawRequestNFT() external view returns (IWithdrawRequestNFT);
}

interface IeEth {
    function liquidityPool() external view returns (ILiquidityPool);

    function totalShares() external view returns (uint256);

    function shares(address _user) external view returns (uint256);
}

interface IWeEth {
    function eETH() external view returns (IeEth);

    function getWeETHByeETH(uint256 _eETHAmount)
        external
        view
        returns (uint256);

    function getEETHByWeETH(uint256 _weETHAmount)
        external
        view
        returns (uint256);

    function wrap(uint256 _eETHAmount) external returns (uint256);

    function unwrap(uint256 _weETHAmount) external returns (uint256);
}
