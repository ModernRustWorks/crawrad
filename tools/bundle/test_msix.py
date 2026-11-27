#!/usr/bin/env python3
"""Unit tests for tools/bundle/msix.py."""

from __future__ import annotations

import unittest
from msix import fill_manifest, msix_version


class TestMsix(unittest.TestCase):
    def test_msix_version_standard(self) -> None:
        self.assertEqual(msix_version("0.1.0"), "0.1.0.0")

    def test_msix_version_prerelease(self) -> None:
        self.assertEqual(msix_version("0.1.0-beta"), "0.1.0.0")
        self.assertEqual(msix_version("0.1.0-beta.1"), "0.1.0.0")
        self.assertEqual(msix_version("0.1.0+20261119"), "0.1.0.0")

    def test_msix_version_with_leading_v(self) -> None:
        self.assertEqual(msix_version("v0.1.0"), "0.1.0.0")

    def test_msix_version_four_parts_valid_zero(self) -> None:
        self.assertEqual(msix_version("1.2.3.0"), "1.2.3.0")

    def test_msix_version_rejected_last_part_non_zero(self) -> None:
        with self.assertRaises(ValueError) as ctx:
            msix_version("1.2.3.4")
        self.assertIn("last part must be 0", str(ctx.exception))

    def test_msix_version_rejected_invalid_string(self) -> None:
        with self.assertRaises(ValueError):
            msix_version("abc")

    def test_msix_version_rejected_too_few_parts(self) -> None:
        with self.assertRaises(ValueError):
            msix_version("1.2")

    def test_fill_manifest_replaces_all_placeholders(self) -> None:
        template = (
            '<Identity Name="@IDENTITY_NAME@" Publisher="@PUBLISHER@" '
            'Version="@VERSION@" PublisherDisplayName="@PUBLISHER_DISPLAY@" />'
        )
        values = {
            "IDENTITY_NAME": "ModernRustWorks.CrawRAD",
            "PUBLISHER": "CN=TestCert",
            "VERSION": "0.1.0.0",
            "PUBLISHER_DISPLAY": "ModernRustWorks",
        }
        filled = fill_manifest(template, values)
        self.assertNotIn("@", filled)
        self.assertIn('Name="ModernRustWorks.CrawRAD"', filled)
        self.assertIn('Publisher="CN=TestCert"', filled)
        self.assertIn('Version="0.1.0.0"', filled)
        self.assertIn('PublisherDisplayName="ModernRustWorks"', filled)

    def test_fill_manifest_unfilled_placeholder_raises(self) -> None:
        template = '<Identity Name="@IDENTITY_NAME@" Missing="@UNKNOWN@" />'
        values = {"IDENTITY_NAME": "CrawRAD"}
        with self.assertRaises(ValueError) as ctx:
            fill_manifest(template, values)
        self.assertIn("@UNKNOWN@", str(ctx.exception))


if __name__ == "__main__":
    unittest.main()
