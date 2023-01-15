const coinChange = require("./coin_change");

describe("coin change", () => {
    it("when amount is zero, number of change should be 1", () => {
        expect(coinChange.count([1, 2, 3], 0)).toBe(1);
    })

    it("no coins, number of change sould be 0", () => {
        expect(coinChange.count([], 10)).toBe(0);
    })

    it("when no combinaison matches, number of change should be 0", () => {
        expect(coinChange.count([3], 2)).toBe(0);
    })
    it("with two coins", () => {
        expect(coinChange.count([1, 2], 3)).toBe(2);
    })

    it("with three coins", () => {
        expect(coinChange.count([1, 2, 3], 4)).toBe(4);
    })

    it("with four coins", () => {
        expect(coinChange.count([1, 2, 4, 5], 201)).toBe(36941);
    })
});