// SPDX-License-Identifier: AGPL-3.0-or-later
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "openzeppelin-contracts/contracts/interfaces/IERC20.sol";
import "src/interfaces/ISwapAdapterTypes.sol";
import "src/libraries/FractionMath.sol";
import "src/etherfi/EtherfiAdapter.sol";

contract EtherfiAdapterTest is Test, ISwapAdapterTypes {
    EtherfiAdapter adapter;
    IWeEth weEth = IWeEth(0xCd5fE23C85820F7B72D0926FC9b05b43E359b7ee);
    IeEth eEth;

    function setUp() public {
        uint256 forkBlock = 19218495;
        vm.createSelectFork(vm.rpcUrl("mainnet"), forkBlock);
        adapter = new EtherfiAdapter(address(weEth));
        eEth = weEth.eETH();

        vm.label(address(weEth), "WeETH");
        vm.label(address(eEth), "eETH");
    }

    receive() external payable {}

    function testPriceFuzzEtherfi(uint256 amount0, uint256 amount1) public {
        bytes32 pair = bytes32(0);
        uint256[] memory limits = adapter.getLimits(pair, IERC20(address(weEth)), IERC20(address(eEth)));
        vm.assume(amount0 < limits[0] && amount0 > 0);
        vm.assume(amount1 < limits[1] && amount1 > 0);
 
        uint256[] memory amounts = new uint256[](2);
        amounts[0] = amount0;
        amounts[1] = amount1;
 
        Fraction[] memory prices = adapter.price(pair, IERC20(address(weEth)), IERC20(address(eEth)), amounts);
 
        for (uint256 i = 0; i < prices.length; i++) {
            assertGt(prices[i].numerator, 0);
            assertGt(prices[i].denominator, 0);
        }
    }

    function testSwapFuzzEtherfiEethWeEth(uint256 specifiedAmount, bool isBuy) public {
        OrderSide side = isBuy ? OrderSide.Buy : OrderSide.Sell;

        IERC20 eEth_ = IERC20(address(eEth));
        IERC20 weEth_ = IERC20(address(weEth));
        bytes32 pair = bytes32(0);
        uint256[] memory limits = adapter.getLimits(pair, eEth_, weEth_);

        if (side == OrderSide.Buy) {
            vm.assume(specifiedAmount < limits[1] && specifiedAmount > 100);

            /// @dev workaround for eETH "deal", as standard ERC20 does not work(balance is shares)
            deal(address(adapter), type(uint256).max);
            adapter.swap(pair, IERC20(address(0)), eEth_, OrderSide.Buy, limits[0]);

            eEth_.approve(address(adapter), type(uint256).max);
        } else {
            vm.assume(specifiedAmount < limits[0] && specifiedAmount > 100);

            /// @dev workaround for eETH "deal", as standard ERC20 does not work(balance is shares)
            deal(address(adapter), type(uint128).max);
            adapter.swap(pair, IERC20(address(0)), eEth_, OrderSide.Buy, specifiedAmount);

            eEth_.approve(address(adapter), specifiedAmount);
        }

        uint256 eEth_balance = eEth_.balanceOf(address(this));
        uint256 weEth_balance = weEth_.balanceOf(address(this));

        Trade memory trade =
            adapter.swap(pair, eEth_, weEth_, side, specifiedAmount);

        if (trade.calculatedAmount > 0) {
            if (side == OrderSide.Buy) {
                assertGe(
                    specifiedAmount,
                    weEth_.balanceOf(address(this)) - weEth_balance
                );
                /// @dev Transfer function contains rounding errors because of rewards in weETH contract, therefore we assume a +/-2 tolerance
                assertLe(
                    specifiedAmount - 2,
                    weEth_.balanceOf(address(this)) - weEth_balance
                );
                assertLe(
                    trade.calculatedAmount - 2,
                    eEth_balance - eEth_.balanceOf(address(this))
                );
            } else {
                assertGe(
                    specifiedAmount,
                    eEth_balance - eEth_.balanceOf(address(this))
                );
                /// @dev Transfer function contains rounding errors because of rewards in eETH contract, therefore we assume a +/-2 tolerance
                assertLe(
                    specifiedAmount - 2,
                    eEth_balance - eEth_.balanceOf(address(this))
                );
                assertEq(
                    trade.calculatedAmount,
                    weEth_.balanceOf(address(this)) - weEth_balance
                );
            }
        }
    }

    function testSwapFuzzEtherfiWeEthEth(uint256 specifiedAmount, bool isBuy) public {
        OrderSide side = isBuy ? OrderSide.Buy : OrderSide.Sell;

        IERC20 eEth_ = IERC20(address(eEth));
        IERC20 weEth_ = IERC20(address(weEth));
        bytes32 pair = bytes32(0);
        uint256[] memory limits = adapter.getLimits(pair, weEth_, eEth_);

        if (side == OrderSide.Buy) {
            vm.assume(specifiedAmount < limits[1] && specifiedAmount > 100);

            /// @dev workaround for eETH "deal", as standard ERC20 does not work(balance is shares)
            deal(address(adapter), type(uint256).max);
            adapter.swap(pair, IERC20(address(0)), weEth_, OrderSide.Buy, limits[0]);

            weEth_.approve(address(adapter), type(uint256).max);
        } else {
            vm.assume(specifiedAmount < limits[0] && specifiedAmount > 100);

            /// @dev workaround for eETH "deal", as standard ERC20 does not work(balance is shares)
            deal(address(adapter), type(uint128).max);
            adapter.swap(pair, IERC20(address(0)), weEth_, OrderSide.Buy, specifiedAmount);

            weEth_.approve(address(adapter), specifiedAmount);
        }

        uint256 eEth_balance = eEth_.balanceOf(address(this));
        uint256 weEth_balance = weEth_.balanceOf(address(this));

        Trade memory trade =
            adapter.swap(pair, weEth_, eEth_, side, specifiedAmount);

        if (trade.calculatedAmount > 0) {
            if (side == OrderSide.Buy) {
                assertGe(
                    specifiedAmount,
                    eEth_.balanceOf(address(this)) - eEth_balance
                );
                /// @dev Transfer function contains rounding errors because of rewards in weETH contract, therefore we assume a +/-2 tolerance
                assertLe(
                    specifiedAmount - 2,
                    eEth_.balanceOf(address(this)) - eEth_balance
                );
                assertLe(
                    trade.calculatedAmount - 2,
                    weEth_balance - weEth_.balanceOf(address(this))
                );
            } else {
                assertGe(
                    specifiedAmount,
                    weEth_balance - weEth_.balanceOf(address(this))
                );
                /// @dev Transfer function contains rounding errors because of rewards in eETH contract, therefore we assume a +/-2 tolerance
                assertLe(
                    specifiedAmount - 2,
                    weEth_balance - weEth_.balanceOf(address(this))
                );
                assertEq(
                    trade.calculatedAmount,
                    eEth_.balanceOf(address(this)) - eEth_balance
                );
            }
        }
    }

    function testSwapFuzzEtherfiEthEeth(uint256 specifiedAmount, bool isBuy) public {
        OrderSide side = isBuy ? OrderSide.Buy : OrderSide.Sell;

        IERC20 eth_ = IERC20(address(0));
        IERC20 eEth_ = IERC20(address(eEth));
        bytes32 pair = bytes32(0);
        uint256[] memory limits = adapter.getLimits(pair, eth_, eEth_);

        if (side == OrderSide.Buy) {
            vm.assume(specifiedAmount < limits[1] && specifiedAmount > 10);

            deal(address(adapter), 100**18);
        } else {
            vm.assume(specifiedAmount < limits[0] && specifiedAmount > 10);

            deal(address(adapter), specifiedAmount);
        }

        uint256 eth_balance = address(adapter).balance;
        uint256 eEth_balance = eEth_.balanceOf(address(this));

        Trade memory trade =
            adapter.swap(pair, eth_, eEth_, side, specifiedAmount);

        if (trade.calculatedAmount > 0) {
            if (side == OrderSide.Buy) {
                assertGe(
                    specifiedAmount,
                    eEth_.balanceOf(address(this)) - eEth_balance
                );
                /// @dev Transfer function contains rounding errors because of rewards in eETH contract, therefore we assume a +/-2 tolerance
                assertLe(
                    specifiedAmount - 2,
                    eEth_.balanceOf(address(this)) - eEth_balance
                );
                assertEq(
                    trade.calculatedAmount,
                    eth_balance - address(adapter).balance
                );
            } else {
                assertEq(
                    specifiedAmount,
                    eth_balance - address(adapter).balance
                );
                assertEq(
                    trade.calculatedAmount,
                    eEth_.balanceOf(address(this)) - eEth_balance
                );
            }
        }
    }
}
