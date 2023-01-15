import sys
import unittest

from coin_change import (
    count,
)

class CoinChangeTest(unittest.TestCase):

    def test_with_zero_amount(self):
        c = count([1, 2, 3], 0)
        self.assertEqual(c, 1)

    def test_with_no_coins(self):
        c = count([], 10)
        self.assertEqual(c, 0)

    def test_with_no_solution_possible(self):
        c = count([2], 3)
        self.assertEqual(c, 0)

    def test_with_one_coins(self):
        c = count([1], 10)
        self.assertEqual(c, 1)

    def test_with_two_coins(self):
        c = count([1, 2], 3)
        self.assertEqual(c, 2)

    def test_with_three_coins(self):
        c = count([1, 2, 3], 4)
        self.assertEqual(c, 4)

    def test_with_four_coins(self):
        c = count([1, 2, 5, 4], 201)
        self.assertEqual(c, 36941)
