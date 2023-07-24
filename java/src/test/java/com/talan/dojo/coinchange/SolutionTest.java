package com.talan.dojo.coinchange;

import org.junit.jupiter.api.*;
import org.junit.jupiter.api.Assertions;

public class SolutionTest {

    @Test
    @DisplayName("when amount is zero, number of change should be 1")
    void amountIsZero() {
        int result = Solution.countAllPossibleChange(new int[]{1, 2, 3}, 0);
        Assertions.assertEquals(1, result);
    }

    @Test
    @DisplayName("no coins, number of change should be 0")
    void noCoins() {
        int result = Solution.countAllPossibleChange(new int[]{}, 10);
        Assertions.assertEquals(0, result);
    }

    @Test
    @DisplayName("when no combination matches, number of change should be 0")
    void noMatchingCombination() {
        int result = Solution.countAllPossibleChange(new int[]{2}, 3);
        Assertions.assertEquals(0, result);
    }

    @Test
    @DisplayName("with two coins")
    void withTwoCoins() {
        int result = Solution.countAllPossibleChange(new int[]{1, 2}, 3);
        Assertions.assertEquals(2, result);
    }

    @Test
    @DisplayName("with three coins")
    void withThreeCoins() {
        int result = Solution.countAllPossibleChange(new int[]{1, 2, 3}, 4);
        Assertions.assertEquals(4, result);
    }

    @Test
    @DisplayName("a big one with four coins")
    void withFourCoins() {
        int result = Solution.countAllPossibleChange(new int[]{1, 2, 3, 4}, 201);
        Assertions.assertEquals(36941, result);
    }




}
